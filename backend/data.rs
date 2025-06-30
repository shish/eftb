use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uom::si::f64::*;
use uom::si::length::meter;

pub type ConnectionId = u64;
pub type SolarSystemId = u64;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnType {
    NpcGate,
    SmartGate,
    Jump,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connection {
    pub id: ConnectionId,
    pub conn_type: ConnType,
    pub distance: Length,
    pub target: SolarSystemId,
}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Connection {}
impl std::hash::Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.conn_type
            .cmp(&other.conn_type)
            .then_with(|| self.distance.partial_cmp(&other.distance).unwrap())
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Star {
    pub id: SolarSystemId,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub connections: Vec<Connection>,
}

impl Star {
    pub fn distance(&self, other: &Star) -> Length {
        Length::new::<meter>(
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
                .sqrt(),
        )
    }
}
impl std::fmt::Debug for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Star").field("id", &self.id).finish()
    }
}
impl PartialEq for Star {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Star {}
impl std::hash::Hash for Star {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Universe {
    pub star_map: HashMap<SolarSystemId, Star>,
    pub star_id_to_name: HashMap<SolarSystemId, String>,
    pub star_name_to_id: HashMap<String, SolarSystemId>,
}
impl Universe {
    pub fn load() -> anyhow::Result<Universe> {
        let star_map: HashMap<SolarSystemId, Star> = bincode::serde::decode_from_slice(
            &std::fs::read("data/starmap.bin")?,
            bincode::config::legacy(),
        )?
        .0;

        let data = std::fs::read_to_string("data/solarsystems.json")?;
        let json = serde_json::from_str::<Vec<crate::raw::RawStar>>(&data)?;
        let star_id_to_name: HashMap<SolarSystemId, String> = json
            .iter()
            .map(|star| (star.id, star.name.clone()))
            .collect();
        let star_name_to_id: HashMap<String, SolarSystemId> = json
            .iter()
            .map(|star| (star.name.clone(), star.id))
            .collect();

        Ok(Universe {
            star_map,
            star_id_to_name,
            star_name_to_id,
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        // std::fs::write("data/starmap.json", serde_json::to_string(&star_map)?)?;
        std::fs::write(
            "data/starmap.bin",
            bincode::serde::encode_to_vec(&self.star_map, bincode::config::legacy())?,
        )?;
        Ok(())
    }

    pub fn star_by_name(&self, name: &String) -> anyhow::Result<&Star> {
        let star_id = self
            .star_name_to_id
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Star not found: {}", name))?;

        self.star_map
            .get(star_id)
            .ok_or_else(|| anyhow::anyhow!("Star not found: {}", name))
    }

    #[cfg(test)]
    pub fn tiny_test() -> Universe {
        use uom::si::length::light_year;

        struct MockStar {
            id: SolarSystemId,
            name: &'static str,
            x: f64,
            y: f64,
        }
        struct MockConn {
            conn_type: ConnType,
            s1: SolarSystemId,
            s2: SolarSystemId,
        }

        /*
         *   A..B..C
         *    \   /
         *    n\ /s
         *      D
         */
        #[rustfmt::skip]
        let stars = [
            MockStar { id: 1, name: "A", x: 0.0, y: 0.0 },
            MockStar { id: 2, name: "B", x: 10.0, y: 0.0 },
            MockStar { id: 3, name: "C", x: 20.0, y: 0.0 },
            MockStar { id: 4, name: "D", x: 10.0, y: 20.0 },
        ];

        #[rustfmt::skip]
        let conns = [
            MockConn { conn_type: ConnType::NpcGate, s1: 1, s2: 4 },
            MockConn { conn_type: ConnType::SmartGate, s1: 4, s2: 3 },
            MockConn { conn_type: ConnType::Jump, s1: 1, s2: 2 },
            MockConn { conn_type: ConnType::Jump, s1: 1, s2: 3 },
            MockConn { conn_type: ConnType::Jump, s1: 1, s2: 4 },
            MockConn { conn_type: ConnType::Jump, s1: 2, s2: 3 },
            MockConn { conn_type: ConnType::Jump, s1: 2, s2: 4 },
            MockConn { conn_type: ConnType::Jump, s1: 3, s2: 4 },
        ];

        let mut star_map: HashMap<SolarSystemId, Star> = stars
            .iter()
            .map(|s| {
                (
                    s.id,
                    Star {
                        id: s.id,
                        x: Length::new::<light_year>(s.x).get::<meter>(),
                        y: Length::new::<light_year>(s.y).get::<meter>(),
                        z: 0.0,                  // Z is not used in this test
                        connections: Vec::new(), // Connections will be added later
                    },
                )
            })
            .collect();

        let mut conn_id = 0;
        for conn in conns {
            let star1 = star_map.get(&conn.s1).unwrap().clone();
            let star2 = star_map.get(&conn.s2).unwrap().clone();
            let dist = star1.distance(&star2);

            star_map
                .get_mut(&conn.s1)
                .unwrap()
                .connections
                .push(Connection {
                    id: conn_id,
                    conn_type: conn.conn_type.clone(),
                    distance: dist,
                    target: star2.id,
                });
            conn_id += 1;

            star_map
                .get_mut(&conn.s2)
                .unwrap()
                .connections
                .push(Connection {
                    id: conn_id,
                    conn_type: conn.conn_type,
                    distance: dist,
                    target: star1.id,
                });
            conn_id += 1;
        }

        for star in star_map.values_mut() {
            star.connections.sort();
        }

        Universe {
            star_map,
            star_id_to_name: stars.iter().map(|s| (s.id, s.name.to_string())).collect(),
            star_name_to_id: stars.iter().map(|s| (s.name.to_string(), s.id)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Star {
            id: 1,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            ..Default::default()
        };
        let b = Star {
            id: 2,
            x: 1.0,
            y: 0.0,
            z: 0.0,
            ..Default::default()
        };
        assert_eq!(a.distance(&b), Length::new::<meter>(1.0));
    }

    /// Check that connections are sorted by type (Jump last) then distance
    /// (shortest first) - this means that when we're searching for a path we
    /// can search all the gates, then search the jumps up until we reach a
    /// jump that is too long, and we can stop there.
    #[test]
    fn test_conn_order() {
        let a = Connection {
            id: 1,
            conn_type: ConnType::Jump,
            distance: Length::new::<meter>(2.0),
            target: 1,
        };
        let b = Connection {
            id: 2,
            conn_type: ConnType::NpcGate,
            distance: Length::new::<meter>(2.0),
            target: 1,
        };
        let c = Connection {
            id: 3,
            conn_type: ConnType::SmartGate,
            distance: Length::new::<meter>(2.0),
            target: 1,
        };
        let d = Connection {
            id: 4,
            conn_type: ConnType::Jump,
            distance: Length::new::<meter>(1.0),
            target: 1,
        };
        let e = Connection {
            id: 5,
            conn_type: ConnType::NpcGate,
            distance: Length::new::<meter>(1.0),
            target: 1,
        };
        let f = Connection {
            id: 6,
            conn_type: ConnType::SmartGate,
            distance: Length::new::<meter>(1.0),
            target: 1,
        };
        let mut conns = vec![
            a.clone(),
            b.clone(),
            c.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
        ];
        conns.sort();
        assert_eq!(conns, vec![e, b, f, c, d, a]);
    }
}
