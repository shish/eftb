use crate::data::*;
use crate::units::Meters;

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
    jump_distance: Meters,
    optimize: PathOptimize,
    use_smart_gates: bool,
) -> Vec<(Connection, f64)> {
    let star = universe
        .stars
        .get(conn.target)
        .expect("Target star not found in universe");
    star.connections
        .iter()
        // take gates and short jumps - stop searching after we
        // find a long jump
        .take_while(|c| c.conn_type != ConnType::Jump || c.distance <= jump_distance)
        // If we're not using smart gates, skip them
        .filter(|c| use_smart_gates || c.conn_type != ConnType::SmartGate)
        // Turn the connection into a (connection, cost) tuple
        .map(|c| {
            let distance = match (optimize, &c.conn_type) {
                // For shortest path, we only care about the distance
                // and don't care about the type of connection
                (PathOptimize::Distance, _) => c.distance,
                // For fuel efficient, we only care about the distance
                // if it's a jump
                (PathOptimize::Fuel, ConnType::Jump) => c.distance,
                // Gate connections are free (-ish. It still takes a tiny
                // amount of fuel to warp to a gate)
                (PathOptimize::Fuel, ConnType::NpcGate) => Meters::new(1.0),
                // Smart gates are slightly more expensive than NPC gates
                (PathOptimize::Fuel, ConnType::SmartGate) => Meters::new(2.0),
                // Treat all hops the same, we want to minimise the total
                (PathOptimize::Hops, _) => Meters::new(1.0),
            };
            (c.clone(), distance.to_light_years())
        })
        .collect()
}

/// Heuristic function for A* pathfinding
/// - Return an approximation of the cost from this connection to the end
/// - Must not return greater than the actual cost, or the path will be suboptimal
///   - Remember that in "optimise for fuel" mode, actual cost might be 1
pub fn heuristic(universe: &Universe, conn: &Connection, end: &Star) -> f64 {
    universe.stars[conn.target].distance(end).to_light_years()
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
    jump_distance: Meters,
    optimize: PathOptimize,
    use_smart_gates: bool,
    timeout: Option<u64>,
) -> PathResult {
    let init_conn = Connection {
        id: 0,
        conn_type: ConnType::Jump,
        distance: Meters::from_light_years(0.0),
        target: universe.star_id_to_idx[&start.id],
    };
    let path = pathfinding::astar(
        &init_conn,
        |conn| successors(universe, conn, jump_distance, optimize, use_smart_gates),
        |conn| heuristic(universe, conn, end),
        |conn| conn.target == universe.star_id_to_idx[&end.id],
        timeout,
    );

    match path {
        pathfinding::PathFindResult::Found((path, _)) => {
            // The first connection is the one we invented
            // to start the search, so we can skip it
            PathResult::Found(path[1..].to_vec())
        }
        pathfinding::PathFindResult::NotFound => PathResult::NotFound,
        pathfinding::PathFindResult::Timeout => PathResult::Timeout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_calc_path(
        universe: &Universe,
        start_idx: StarIdx,
        end_idx: StarIdx,
        jump_distance: f64,
        optimize: PathOptimize,
        use_smart_gates: bool,
    ) -> Vec<Connection> {
        match calc_path(
            universe,
            &universe.stars[start_idx],
            &universe.stars[end_idx],
            Meters::new(jump_distance),
            optimize,
            use_smart_gates,
            None,
        ) {
            PathResult::Found(path) => path,
            PathResult::NotFound => vec![],
            PathResult::Timeout => panic!("Path search timed out"),
        }
    }

    // Tiny jump distance can't get to the target, even with NPC gate
    #[test]
    fn test_path_no_path() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 0, 1, 0.1, PathOptimize::Fuel, true);
        assert!(path.is_empty());
    }

    // Gate uses less fuel than jump
    #[test]
    fn test_path_fuel_prefer_gate_over_jump() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 0, 3, 25.0, PathOptimize::Fuel, true);

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].target, 3);
        assert_eq!(path[0].conn_type, ConnType::NpcGate);
    }

    // Gate + short-jump uses less fuel than long-jump
    #[test]
    fn test_path_fuel_prefer_more_hops_over_more_fuel() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 3, 1, 25.0, PathOptimize::Fuel, false);

        assert_eq!(path.len(), 2);

        assert_eq!(path[0].target, 0);
        assert_eq!(path[0].conn_type, ConnType::NpcGate);

        assert_eq!(path[1].target, 1);
        assert_eq!(path[1].conn_type, ConnType::Jump);
        assert_eq!(path[1].distance, Meters::new(10.0));
    }

    // One long jump is shorter than a gate and a short jump
    #[test]
    fn test_path_distance() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 1, 3, 25.0, PathOptimize::Distance, false);

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].target, 3);
        assert_eq!(path[0].conn_type, ConnType::Jump);
    }

    // Jump instead of smart-gate if smart-gate is disabled
    #[test]
    fn test_path_hops_jump_if_smart_gate_disabled() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 3, 2, 25.0, PathOptimize::Hops, false);

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].target, 2);
        assert_eq!(path[0].conn_type, ConnType::Jump);
    }

    // Take smart gate if enabled
    #[test]
    fn test_path_hops_use_smart_gate_if_smart_gate_enabled() {
        let universe = Universe::tiny_test();
        let path = call_calc_path(&universe, 3, 2, 25.0, PathOptimize::Hops, true);

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].target, 2);
        assert_eq!(path[0].conn_type, ConnType::SmartGate);
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
            if let Some(timeout) = timeout {
                if start_time.elapsed().as_secs() >= timeout {
                    return PathFindResult::Timeout;
                }
            }
            let successors = {
                let (node, &(_, c)) = parents
                    .get_index(index)
                    .expect("Can't find index which we inserted earlier");
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
