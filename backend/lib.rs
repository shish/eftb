use std::collections::HashMap;

use pathfinding::prelude::astar;
use uom::si::f64::*;
use uom::si::length::light_year;

pub mod data;
pub mod raw;

pub fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Length {
    Length::new::<light_year>((fuel / mass) * efficiency * 1e7)
}

pub fn calc_fuel(dist: Length, mass: f64, efficiency: f64) -> f64 {
    return dist.get::<light_year>() / (efficiency * 1e7) * mass;
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOptimize {
    Fuel,
    Distance,
    Hops,
}

/// Given a connection, return a list of all possible next-connections,
/// and what each of those connections costs
fn successors(
    star_map: &HashMap<u64, data::Star>,
    conn: &data::Connection,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Vec<(data::Connection, i64)> {
    let star = star_map.get(&conn.target).unwrap();
    star.connections
        .iter()
        // take gates and short jumps - stop searching after we
        // find a long jump
        .take_while(|c| c.conn_type != data::ConnType::Jump || c.distance <= jump_distance)
        // Turn the connection into a (connection, cost) tuple
        .map(|c| {
            let distance = c.distance.get::<light_year>() as i64;
            match (optimize, c.conn_type) {
                // For shortest path, we only care about the distance
                // and don't care about the type of connection
                (PathOptimize::Distance, _) => (c.clone(), distance),
                // For fuel efficient, we only care about the distance
                // if it's a jump
                (PathOptimize::Fuel, data::ConnType::Jump) => (c.clone(), distance),
                // Gate connections are free (-ish. It still takes a tiny
                // amount of fuel to warp to a gate)
                (PathOptimize::Fuel, data::ConnType::NpcGate) => (c.clone(), 1),
                // Smart gates are slightly more expensive than NPC gates
                (PathOptimize::Fuel, data::ConnType::SmartGate) => (c.clone(), 2),
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
pub fn heuristic(
    star_map: &HashMap<u64, data::Star>,
    conn: &data::Connection,
    end: &data::Star,
) -> i64 {
    let d = star_map
        .get(&conn.target)
        .unwrap()
        .distance(end)
        .get::<light_year>();
    return d as i64;
}

pub fn calc_path(
    star_map: &HashMap<u64, data::Star>,
    start: &data::Star,
    end: &data::Star,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Option<Vec<data::Connection>> {
    let init_conn = data::Connection {
        id: 0,
        conn_type: data::ConnType::Jump,
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

pub fn calc_exits(
    star_map: &HashMap<u64, data::Star>,
    start: &data::Star,
    jump_distance: Length,
) -> Vec<(data::Star, data::Star)> {
    let mut exits: Vec<(data::Star, data::Star)> = Vec::new();
    let mut checked: Vec<data::SolarSystemId> = Vec::new();
    let mut to_check: Vec<data::SolarSystemId> = Vec::new();

    to_check.push(start.id);

    while !to_check.is_empty() {
        let current = to_check.pop().unwrap();
        checked.push(current);
        let star = star_map.get(&current).unwrap();
        for conn in &star.connections {
            let target = star_map.get(&conn.target).unwrap();
            if conn.conn_type == data::ConnType::NpcGate
                || conn.conn_type == data::ConnType::SmartGate
            {
                if !checked.contains(&target.id) && !to_check.contains(&target.id) {
                    to_check.push(target.id);
                }
            } else if conn.conn_type == data::ConnType::Jump
                && conn.distance <= jump_distance
                && target.region_id != start.region_id
            {
                let target = star_map.get(&conn.target).unwrap();
                exits.push((star.clone(), target.clone()));
            }
        }
    }

    return exits;
}
