use uom::si::f64::*;
use uom::si::length::light_year;

use crate::data::*;

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOptimize {
    Fuel,
    Distance,
    Hops,
}

/// Given a connection, return a list of all possible next-connections,
/// and what each of those connections costs
pub fn successors(
    universe: &Universe,
    conn: &Connection,
    jump_distance: Length,
    optimize: PathOptimize,
    use_smart_gates: bool,
) -> Vec<(Connection, f64)> {
    let star = universe.star_map.get(&conn.target).unwrap();
    star.connections
        .iter()
        // take gates and short jumps - stop searching after we
        // find a long jump
        .take_while(|c| c.conn_type != ConnType::Jump || c.distance <= jump_distance)
        // If we're not using smart gates, skip them
        .filter(|c| use_smart_gates || c.conn_type != ConnType::SmartGate)
        // Turn the connection into a (connection, cost) tuple
        .map(|c| {
            let distance = c.distance.get::<light_year>();
            match (optimize, &c.conn_type) {
                // For shortest path, we only care about the distance
                // and don't care about the type of connection
                (PathOptimize::Distance, _) => (c.clone(), distance),
                // For fuel efficient, we only care about the distance
                // if it's a jump
                (PathOptimize::Fuel, ConnType::Jump) => (c.clone(), distance),
                // Gate connections are free (-ish. It still takes a tiny
                // amount of fuel to warp to a gate)
                (PathOptimize::Fuel, ConnType::NpcGate) => (c.clone(), 1.0),
                // Smart gates are slightly more expensive than NPC gates
                (PathOptimize::Fuel, ConnType::SmartGate) => (c.clone(), 2.0),
                // Treat all hops the same, we want to minimise the total
                (PathOptimize::Hops, _) => (c.clone(), 1.0),
            }
        })
        .collect()
}

/// Heuristic function for A* pathfinding
/// - Return an approximation of the cost from this connection to the end
/// - Must not return greater than the actual cost, or the path will be suboptimal
///   - Remember that in "optimise for fuel" mode, actual cost might be 1
pub fn heuristic(universe: &Universe, conn: &Connection, end: &Star) -> f64 {
    let d = universe.star_map[&conn.target]
        .distance(end)
        .get::<light_year>();
    return d;
}

#[derive(Debug, PartialEq)]
pub enum PathResult {
    Found(Vec<Connection>),
    NotFound,
    Timeout,
}

pub fn calc_path(
    universe: &Universe,
    start: &Star,
    end: &Star,
    jump_distance: Length,
    optimize: PathOptimize,
    use_smart_gates: bool,
    timeout: Option<u64>,
) -> PathResult {
    let init_conn = Connection {
        id: ConnectionId::MAX,
        conn_type: ConnType::Jump,
        distance: Length::new::<light_year>(0.0),
        target: start.id,
    };
    let path = pathfinding::astar(
        &init_conn,
        |conn| successors(&universe, conn, jump_distance, optimize, use_smart_gates),
        |conn| heuristic(&universe, conn, end),
        |conn| conn.target == end.id,
        timeout,
    );

    match path {
        pathfinding::PathFindResult::Found((path, _)) => {
            // The first connection is the one we invented
            // to start the search, so we can skip it
            return PathResult::Found(path[1..].to_vec());
        }
        pathfinding::PathFindResult::NotFound => return PathResult::NotFound,
        pathfinding::PathFindResult::Timeout => return PathResult::Timeout,
    }
}

#[cfg(test)]
mod tests {
    use uom::si::length::light_year;

    use super::*;

    fn call_calc_path(
        universe: &Universe,
        start_id: SolarSystemId,
        end_id: SolarSystemId,
        jump_distance: f64,
        optimize: PathOptimize,
        use_smart_gates: bool,
    ) -> PathResult {
        calc_path(
            universe,
            &universe.star_map[&start_id],
            &universe.star_map[&end_id],
            Length::new::<light_year>(jump_distance),
            optimize,
            use_smart_gates,
            None,
        )
    }

    // Gate uses less fuel than jump
    #[test]
    fn test_path_fuel_prefer_gate_over_jump() {
        let universe = Universe::tiny_test();
        assert_eq!(
            call_calc_path(&universe, 1, 4, 25.0, PathOptimize::Fuel, true),
            PathResult::Found(vec![universe.star_map[&1].connections[0].clone()])
        );
    }

    // Gate + short-jump uses less fuel than long-jump
    #[test]
    fn test_path_fuel_prefer_more_hops_over_more_fuel() {
        let universe = Universe::tiny_test();
        assert_eq!(
            call_calc_path(&universe, 4, 2, 25.0, PathOptimize::Fuel, false),
            PathResult::Found(vec![
                universe.star_map[&4].connections[0].clone(),
                universe.star_map[&1].connections[1].clone(),
            ])
        );
    }

    // One long jump is shorter than a gate and a short jump
    #[test]
    fn test_path_distance() {
        let universe = Universe::tiny_test();
        assert_eq!(
            call_calc_path(&universe, 2, 4, 25.0, PathOptimize::Distance, false),
            PathResult::Found(vec![universe.star_map[&2].connections[2].clone(),])
        );
    }

    // Jump instead of smart-gate if smart-gate is disabled
    #[test]
    fn test_path_hops_jump_if_smart_gate_disabled() {
        let universe = Universe::tiny_test();
        assert_eq!(
            call_calc_path(&universe, 4, 3, 25.0, PathOptimize::Hops, false),
            PathResult::Found(vec![universe.star_map[&4].connections[4].clone()])
        );
    }

    // Take smart gate if enabled
    #[test]
    fn test_path_hops_use_smart_gate_if_smart_gate_enabled() {
        let universe = Universe::tiny_test();
        assert_eq!(
            call_calc_path(&universe, 4, 3, 25.0, PathOptimize::Hops, true),
            PathResult::Found(vec![universe.star_map[&4].connections[1].clone()])
        );
    }
}

// https://docs.rs/crate/pathfinding/latest/source/src/directed/astar.rs
// modified to return both nodes and edges
mod pathfinding {
    use indexmap::map::Entry::{Occupied, Vacant};
    use num_traits::Zero;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::hash::Hash;
    use std::time::Instant;

    use indexmap::IndexMap;
    use rustc_hash::FxHasher;
    use std::hash::BuildHasherDefault;

    type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

    pub enum PathFindResult<N, C> {
        Found((Vec<N>, C)),
        NotFound,
        Timeout,
    }

    pub fn astar<N, C, FN, IN, FH, FS>(
        start: &N,
        mut successors: FN,
        mut heuristic: FH,
        mut success: FS,
        timeout: Option<u64>,
    ) -> PathFindResult<N, C>
    where
        N: Eq + Hash + Clone,
        C: Zero + PartialOrd + Copy,
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = (N, C)>,
        FH: FnMut(&N) -> C,
        FS: FnMut(&N) -> bool,
    {
        let start_time = Instant::now();
        let mut to_see = BinaryHeap::new();
        to_see.push(SmallestCostHolder {
            estimated_cost: Zero::zero(),
            cost: Zero::zero(),
            index: 0,
        });
        let mut parents: FxIndexMap<N, (usize, C)> = FxIndexMap::default();
        parents.insert(start.clone(), (usize::MAX, Zero::zero()));
        while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
            if timeout.is_some() && start_time.elapsed().as_secs() >= timeout.unwrap() {
                return PathFindResult::Timeout;
            }
            let successors = {
                let (node, &(_, c)) = parents.get_index(index).unwrap(); // Cannot fail
                if success(node) {
                    let path = reverse_path(&parents, |&(p, _)| p, index);
                    return PathFindResult::Found((path, cost));
                }
                // We may have inserted a node several time into the binary heap if we found
                // a better way to access it. Ensure that we are currently dealing with the
                // best path and discard the others.
                if cost > c {
                    continue;
                }
                successors(node)
            };
            for (successor, move_cost) in successors {
                let new_cost = cost + move_cost;
                let h; // heuristic(&successor)
                let n; // index for successor
                match parents.entry(successor) {
                    Vacant(e) => {
                        h = heuristic(e.key());
                        n = e.index();
                        e.insert((index, new_cost));
                    }
                    Occupied(mut e) => {
                        if e.get().1 > new_cost {
                            h = heuristic(e.key());
                            n = e.index();
                            e.insert((index, new_cost));
                        } else {
                            continue;
                        }
                    }
                }

                to_see.push(SmallestCostHolder {
                    estimated_cost: new_cost + h,
                    cost: new_cost,
                    index: n,
                });
            }
        }
        PathFindResult::NotFound
    }

    #[allow(clippy::needless_collect)]
    fn reverse_path<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
    where
        N: Eq + Hash + Clone,
        F: FnMut(&V) -> usize,
    {
        let mut i = start;
        let path = std::iter::from_fn(|| {
            parents.get_index(i).map(|(node, value)| {
                i = parent(value);
                node
            })
        })
        .collect::<Vec<&N>>();
        // Collecting the going through the vector is needed to revert the path because the
        // unfold iterator is not double-ended due to its iterative nature.
        path.into_iter().rev().cloned().collect()
    }

    struct SmallestCostHolder<K> {
        estimated_cost: K,
        cost: K,
        index: usize,
    }

    impl<K: PartialEq> PartialEq for SmallestCostHolder<K> {
        fn eq(&self, other: &Self) -> bool {
            self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
        }
    }

    impl<K: PartialEq> Eq for SmallestCostHolder<K> {}

    impl<K: PartialOrd> PartialOrd for SmallestCostHolder<K> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<K: PartialOrd> Ord for SmallestCostHolder<K> {
        fn cmp(&self, other: &Self) -> Ordering {
            match other.estimated_cost.partial_cmp(&self.estimated_cost) {
                Some(Ordering::Equal) | None => match self.cost.partial_cmp(&other.cost) {
                    Some(o) => o,
                    None => Ordering::Equal,
                },
                Some(s) => s,
            }
        }
    }
}
