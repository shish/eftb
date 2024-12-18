use pathfinding::prelude::astar;
use std::collections::HashMap;

use uom::si::f64::*;
use uom::si::length::{light_year, meter};

use crate::data;

pub fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Length {
    Length::new::<meter>((fuel / mass) * efficiency * 1e23)
}

pub fn calc_fuel(dist: Length, mass: f64, efficiency: f64) -> f64 {
    return dist.get::<meter>() / (efficiency * 1e23) * mass;
}

pub fn calc_path(
    star_map: &HashMap<u64, data::Star>,
    start: &data::Star,
    end: &data::Star,
    jump_distance: Length,
) -> Option<Vec<data::Star>> {
    fn successors(
        star_map: &HashMap<u64, data::Star>,
        star: &data::Star,
        jump_distance: Length,
    ) -> Vec<(data::Star, i64)> {
        star.connections
            .iter()
            .filter_map(|c| {
                if c.conn_type == data::ConnType::Jump && c.distance > jump_distance {
                    return None;
                }
                Some((
                    star_map.get(&c.target).unwrap().clone(),
                    c.distance.get::<light_year>() as i64,
                ))
            })
            .collect()
    }
    astar(
        start,
        |star| successors(&star_map, star, jump_distance),
        |star| (star.distance(end).get::<light_year>() / 3.0) as i64,
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
