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
fn successors(
    universe: &Universe,
    conn: &Connection,
    jump_distance: Length,
    optimize: PathOptimize,
    use_smart_gates: bool,
) -> Vec<(Connection, i64)> {
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
            let distance = c.distance.get::<light_year>() as i64;
            match (optimize, &c.conn_type) {
                // For shortest path, we only care about the distance
                // and don't care about the type of connection
                (PathOptimize::Distance, _) => (c.clone(), distance),
                // For fuel efficient, we only care about the distance
                // if it's a jump
                (PathOptimize::Fuel, ConnType::Jump) => (c.clone(), distance),
                // Gate connections are free (-ish. It still takes a tiny
                // amount of fuel to warp to a gate)
                (PathOptimize::Fuel, ConnType::NpcGate) => (c.clone(), 1),
                // Smart gates are slightly more expensive than NPC gates
                (PathOptimize::Fuel, ConnType::SmartGate) => (c.clone(), 2),
                // Treat all hops the same, we want to minimise the total
                (PathOptimize::Hops, _) => (c.clone(), 1),
            }
        })
        .collect()
}

/// Heuristic function for A* pathfinding
/// - Return an approximation of the cost from this connection to the end
/// - Must not return greater than the actual cost, or the path will be suboptimal
///   - Remember that in "optimise for fuel" mode, actual cost might be 1
pub fn heuristic(universe: &Universe, conn: &Connection, end: &Star) -> i64 {
    let d = universe
        .star_map
        .get(&conn.target)
        .unwrap()
        .distance(end)
        .get::<light_year>();
    return d as i64;
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
        id: 0,
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

    #[test]
    fn test_path() {
        let stars = [
            Star {
                id: 1,
                region_id: 1,
                connections: vec![Connection {
                    id: 1,
                    conn_type: ConnType::Jump,
                    distance: Length::new::<light_year>(10.0),
                    target: 2,
                }],
                ..Default::default()
            },
            Star {
                id: 2,
                region_id: 2,
                connections: vec![Connection {
                    id: 2,
                    conn_type: ConnType::Jump,
                    distance: Length::new::<light_year>(10.0),
                    target: 1,
                }],
                ..Default::default()
            },
        ];

        let universe = Universe {
            star_map: stars.iter().map(|s| (s.id, s.clone())).collect(),
        };

        assert_eq!(
            calc_path(
                &universe,
                &stars[0],
                &stars[1],
                Length::new::<light_year>(20.0),
                PathOptimize::Fuel,
                false,
                None
            ),
            PathResult::Found(vec![(stars[0].connections[0].clone())])
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
        C: Zero + Ord + Copy,
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

    impl<K: Ord> PartialOrd for SmallestCostHolder<K> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<K: Ord> Ord for SmallestCostHolder<K> {
        fn cmp(&self, other: &Self) -> Ordering {
            match other.estimated_cost.cmp(&self.estimated_cost) {
                Ordering::Equal => self.cost.cmp(&other.cost),
                s => s,
            }
        }
    }
}
