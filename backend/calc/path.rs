use std::collections::HashMap;

use pathfinding::prelude::astar;
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
    star_map: &HashMap<SolarSystemId, Star>,
    conn: &Connection,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Vec<(Connection, i64)> {
    let star = star_map.get(&conn.target).unwrap();
    star.connections
        .iter()
        // take gates and short jumps - stop searching after we
        // find a long jump
        .take_while(|c| c.conn_type != ConnType::Jump || c.distance <= jump_distance)
        // Turn the connection into a (connection, cost) tuple
        .map(|c| {
            let distance = c.distance.get::<light_year>() as i64;
            match (optimize, c.conn_type) {
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
pub fn heuristic(star_map: &HashMap<SolarSystemId, Star>, conn: &Connection, end: &Star) -> i64 {
    let d = star_map
        .get(&conn.target)
        .unwrap()
        .distance(end)
        .get::<light_year>();
    return d as i64;
}

pub fn calc_path(
    star_map: &HashMap<SolarSystemId, Star>,
    start: &Star,
    end: &Star,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Option<Vec<Connection>> {
    let init_conn = Connection {
        id: 0,
        conn_type: ConnType::Jump,
        distance: Length::new::<light_year>(0.0),
        target: start.id,
    };
    let path = astar(
        &init_conn,
        |conn| successors(&star_map, conn, jump_distance, optimize),
        |conn| heuristic(&star_map, conn, end),
        |conn| conn.target == end.id,
    )
    .map(|(path, _)| path);

    match path {
        Some(path) => {
            // The first connection is the one we invented
            // to start the search, so we can skip it
            return Some(path[1..].to_vec());
        }
        None => return None,
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

        let star_map: HashMap<SolarSystemId, Star> =
            stars.iter().map(|s| (s.id, s.clone())).collect();

        assert_eq!(
            calc_path(
                &star_map,
                &stars[0],
                &stars[1],
                Length::new::<light_year>(20.0),
                PathOptimize::Fuel
            ),
            Some(vec![(stars[0].connections[0].clone())])
        );
    }
}
