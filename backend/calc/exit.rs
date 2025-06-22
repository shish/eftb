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

        let star_map: std::collections::HashMap<SolarSystemId, Star> =
            stars.iter().map(|s| (s.id, s.clone())).collect();
        let universe = Universe { star_map };

        assert_eq!(
            calc_exit(&universe, &stars[0], Length::new::<light_year>(20.0)),
            vec![(stars[0].clone(), stars[1].clone())]
        );
    }
}
