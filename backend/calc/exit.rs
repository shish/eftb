use std::collections::HashMap;

use uom::si::f64::*;

use crate::data;

pub fn calc_exit(
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
