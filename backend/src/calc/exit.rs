use crate::data::*;
use crate::units::Meters;

pub fn calc_exit(universe: &Universe, start: &Star, jump_distance: Meters) -> Vec<(Star, Star)> {
    let mut gate_network: Vec<StarIdx> = Vec::new();
    let mut to_add_to_network: Vec<StarIdx> = Vec::new();

    to_add_to_network.push(universe.star_id_to_idx[&start.id]);
    while let Some(current) = to_add_to_network.pop() {
        gate_network.push(current);
        for conn in &universe.stars[current].connections {
            if (
                conn.conn_type == ConnType::NpcGate
                // || conn.conn_type == ConnType::SmartGate
            ) && !gate_network.contains(&conn.target)
                && !to_add_to_network.contains(&conn.target)
            {
                to_add_to_network.push(conn.target);
            }
        }
    }

    let mut exits: Vec<(Star, Star)> = Vec::new();

    gate_network.iter().for_each(|idx| {
        let star = &universe.stars[*idx];
        for conn in &star.connections {
            if conn.conn_type == ConnType::Jump {
                let other = &universe.stars[conn.target];
                let distance = star.distance(other);

                if distance <= jump_distance {
                    exits.push((star.clone(), other.clone()));
                }
            }
        }
    });

    exits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit() {
        let universe = Universe::tiny_test();

        assert_eq!(
            calc_exit(&universe, &universe.stars[0], Meters::new(10.0)),
            vec![
                (universe.stars[0].clone(), universe.stars[1].clone()),
                // via SmartGate
                // (universe.star_map[&3].clone(), universe.star_map[&2].clone()),
            ]
        );
    }
}
