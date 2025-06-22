use uom::si::f64::*;

use crate::data::*;

pub fn calc_exit(universe: &Universe, start: &Star, jump_distance: Length) -> Vec<(Star, Star)> {
    let mut exits: Vec<(Star, Star)> = Vec::new();
    let mut checked: Vec<SolarSystemId> = Vec::new();
    let mut to_check: Vec<SolarSystemId> = Vec::new();

    to_check.push(start.id);

    while !to_check.is_empty() {
        let current = to_check.pop().unwrap();
        checked.push(current);
        let star = universe.star_map.get(&current).unwrap();
        for conn in &star.connections {
            let target = universe.star_map.get(&conn.target).unwrap();
            if conn.conn_type == ConnType::NpcGate || conn.conn_type == ConnType::SmartGate {
                if !checked.contains(&target.id) && !to_check.contains(&target.id) {
                    to_check.push(target.id);
                }
            } else if conn.conn_type == ConnType::Jump
                && conn.distance <= jump_distance
                && target.region_id != start.region_id
            {
                let target = universe.star_map.get(&conn.target).unwrap();
                exits.push((star.clone(), target.clone()));
            }
        }
    }

    return exits;
}

#[cfg(test)]
mod tests {
    use uom::si::length::light_year;

    use super::*;

    #[test]
    fn test_exit() {
        let universe = Universe::tiny_test();

        assert_eq!(
            calc_exit(
                &universe,
                &universe.star_map[&1],
                Length::new::<light_year>(10.0)
            ),
            vec![
                (universe.star_map[&1].clone(), universe.star_map[&2].clone()),
                (universe.star_map[&3].clone(), universe.star_map[&2].clone()),
            ]
        );
    }
}
