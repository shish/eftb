use uom::si::f64::*;

use crate::data::*;

pub fn calc_exit(universe: &Universe, start: &Star, jump_distance: Length) -> Vec<(Star, Star)> {
    let mut gate_network: Vec<SolarSystemId> = Vec::new();
    let mut to_add_to_network: Vec<SolarSystemId> = Vec::new();

    to_add_to_network.push(start.id);
    while !to_add_to_network.is_empty() {
        let current = to_add_to_network.pop().unwrap();
        gate_network.push(current);
        for conn in &universe.star_map[&current].connections {
            if conn.conn_type == ConnType::NpcGate
            // || conn.conn_type == ConnType::SmartGate
            {
                if !gate_network.contains(&conn.target) && !to_add_to_network.contains(&conn.target)
                {
                    to_add_to_network.push(conn.target);
                }
            }
        }
    }

    let mut exits: Vec<(Star, Star)> = Vec::new();

    gate_network.iter().for_each(|id| {
        let star = &universe.star_map[id];
        for conn in &star.connections {
            if conn.conn_type == ConnType::Jump {
                let other = &universe.star_map[&conn.target];
                let distance = star.distance(other);

                if distance <= jump_distance {
                    exits.push((star.clone(), other.clone()));
                }
            }
        }
    });

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
                // via SmartGate
                // (universe.star_map[&3].clone(), universe.star_map[&2].clone()),
            ]
        );
    }
}
