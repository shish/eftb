use std::cmp::min;
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
}

/// Given a star, return a list of all possible stars that can be reached,
/// and the distance to each of them
fn successors(
    star_map: &HashMap<u64, data::Star>,
    star: &data::Star,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Vec<(data::Star, i64)> {
    star.connections
        .iter()
        .filter_map(|c| {
            // If we can't jump that far, don't consider it at all
            if c.conn_type == data::ConnType::Jump && c.distance > jump_distance {
                return None;
            }
            let target = star_map.get(&c.target).unwrap().clone();
            let distance = c.distance.get::<light_year>() as i64;
            match (optimize, c.conn_type) {
                // For shortest path, we only care about the distance
                // and don't care about the type of connection
                (PathOptimize::Distance, _) => Some((target, distance)),
                // For fuel efficient, we only care about the distance
                // if it's a jump
                (PathOptimize::Fuel, data::ConnType::Jump) => Some((target, distance)),
                // Gate connections are free (-ish. It still takes a tiny
                // amount of fuel to warp to a gate)
                (PathOptimize::Fuel, data::ConnType::NpcGate) => Some((target, 1)),
                // Smart gates are slightly more expensive than NPC gates
                (PathOptimize::Fuel, data::ConnType::SmartGate) => Some((target, 2)),
            }
        })
        .collect()
}

/// Heuristic function for A* pathfinding
/// Given multiple possibe paths forward, which one is probably the best?
pub fn heuristic(star: &data::Star, end: &data::Star, optimize: PathOptimize) -> i64 {
    let mut min_distance = i64::MAX;
    if optimize == PathOptimize::Fuel {
        // If we're optimizing for fuel, give stars with gate
        // connections to the goal higher priority
        for conn in &star.connections {
            if conn.target == end.id {
                if conn.conn_type == data::ConnType::NpcGate {
                    min_distance = min(min_distance, 1);
                }
                if conn.conn_type == data::ConnType::SmartGate {
                    min_distance = min(min_distance, 2);
                }
            }
        }
    }
    let min_jump = (star.distance(end).get::<light_year>() / 2.0) as i64;
    min_distance = min(min_distance, min_jump);
    return min_distance;
}

pub fn calc_path(
    star_map: &HashMap<u64, data::Star>,
    start: &data::Star,
    end: &data::Star,
    jump_distance: Length,
    optimize: PathOptimize,
) -> Option<Vec<data::Star>> {
    astar(
        start,
        |star| successors(&star_map, star, jump_distance, optimize),
        |star| heuristic(star, end, optimize),
        |star| star.id == end.id,
    )
    .map(|(path, _)| path)
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
