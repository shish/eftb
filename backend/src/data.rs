use std::collections::HashMap;

use indicatif::ProgressIterator;
use log::{info, warn};
use rkyv::{Archive, Deserialize, Serialize};

use crate::raw;
use crate::units::Meters;

pub type ConnectionId = u32;
pub type SolarSystemId = u32;

#[derive(Debug, Archive, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rkyv(compare(PartialEq))]
#[rkyv(derive(Debug))]
pub enum ConnType {
    NpcGate,
    SmartGate,
    Jump,
}

#[derive(Debug, Archive, Deserialize, Serialize, Clone)]
#[rkyv(compare(PartialEq))]
#[rkyv(derive(Debug))]
pub struct Connection {
    pub id: ConnectionId,
    pub conn_type: ConnType,
    pub distance: Meters,
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
            .then_with(|| self.distance.cmp(&other.distance))
    }
}

pub type Point3D = [f64; 3];

#[derive(Archive, Deserialize, Serialize, Clone, Default)]
#[rkyv(compare(PartialEq))]
#[rkyv(derive(Debug))]
pub struct Star {
    pub id: SolarSystemId,
    pub loc: Point3D,
    pub connections: Vec<Connection>,
}

impl Star {
    pub fn distance(&self, other: &Star) -> Meters {
        Meters::new(
            ((self.loc[0] - other.loc[0]).powi(2)
                + (self.loc[1] - other.loc[1]).powi(2)
                + (self.loc[2] - other.loc[2]).powi(2))
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

#[derive(Debug, Archive, Deserialize, Serialize, Clone)]
#[rkyv(derive(Debug))]
pub struct Universe {
    pub star_map: HashMap<SolarSystemId, Star>,
    pub star_id_to_name: HashMap<SolarSystemId, String>,
    pub star_name_to_id: HashMap<String, SolarSystemId>,
}
impl Universe {
    pub fn build(max_jump_dist: Meters) -> anyhow::Result<Universe> {
        info!("Loading raw data");
        let raw_star_data = raw::RawStarMap::from_file("data/starmap.json")?;
        let raw_smart_gates: Vec<raw::RawSmartGate> =
            serde_json::from_str(&std::fs::read_to_string("data/smartgates.json")?)?;
        Universe::build_from_raw(raw_star_data, raw_smart_gates, max_jump_dist)
    }

    pub fn build_from_raw(
        raw_star_data: raw::RawStarMap,
        raw_smart_gates: Vec<raw::RawSmartGate>,
        max_jump_dist: Meters,
    ) -> anyhow::Result<Universe> {
        info!("Building star map");
        let mut star_map: HashMap<SolarSystemId, Star> = HashMap::new();
        for raw_star in raw_star_data.solar_systems.iter() {
            let star = Star {
                id: raw_star.solar_system_id,
                loc: raw_star.center,
                connections: Vec::new(),
            };
            star_map.insert(raw_star.solar_system_id, star);
        }

        info!("Building connections from npc gates");
        let mut conn_count = 1; // Connection #0 is reserved for path init
        for raw_jump in raw_star_data.jumps.iter() {
            // rust only lets us borrow one mutable star at a time, so we can't add
            // from->to and to->from gates in the same block
            for (fid, tid) in [
                (raw_jump.from_system_id, raw_jump.to_system_id),
                (raw_jump.to_system_id, raw_jump.from_system_id),
            ] {
                let Some(to_star) = star_map.get(&tid).cloned() else {
                    warn!("Jump has unknown target {}", tid);
                    continue;
                };
                let Some(from_star) = star_map.get_mut(&fid) else {
                    warn!("Jump has unknown source {}", fid);
                    continue;
                };

                //let to_star = star_map.get(&tid).unwrap().clone();
                //let from_star = star_map.get_mut(&fid).unwrap();
                let distance: Meters = from_star.distance(&to_star);
                let conn_type = match raw_jump.jump_type {
                    0 => ConnType::NpcGate,
                    1 => ConnType::NpcGate, // What are these ???
                    _ => {
                        info!(
                            "{} -> {} is an unknown jump type ({})",
                            fid, tid, raw_jump.jump_type
                        );
                        continue;
                    }
                };
                from_star.connections.push(Connection {
                    id: conn_count,
                    conn_type,
                    distance,
                    target: tid,
                });
                conn_count += 1;
            }
        }

        info!("Building connections from smart gates");
        for gate in raw_smart_gates.iter() {
            let Some(to_star) = star_map.get(&gate.to).cloned() else {
                warn!("Smart gate has unknown target {}", gate.to);
                continue;
            };
            let Some(from_star) = star_map.get_mut(&gate.from) else {
                warn!("Smart gate has unknown source {}", gate.from);
                continue;
            };

            let distance: Meters = from_star.distance(&to_star);
            from_star.connections.push(Connection {
                id: conn_count,
                conn_type: ConnType::SmartGate,
                distance,
                target: gate.to,
            });
            conn_count += 1;
        }

        info!("Building connections from jumps");
        let cloned_star_map = star_map.clone();
        for star in star_map.values_mut().progress() {
            for other_star in cloned_star_map.values() {
                if star.id == other_star.id {
                    continue;
                }
                let distance: Meters = star.distance(other_star);
                if distance < max_jump_dist {
                    star.connections.push(Connection {
                        id: conn_count,
                        conn_type: ConnType::Jump,
                        distance,
                        target: other_star.id,
                    });
                    conn_count += 1;
                }
            }
        }

        info!("Sorting {} connections", conn_count);
        // sort gates first, and then jumps by distance - then when we
        // reach a jump that is too long we can stop searching
        for star in star_map.values_mut().progress() {
            star.connections.sort_unstable();
        }

        let json = raw_star_data.solar_systems;
        let star_id_to_name: HashMap<SolarSystemId, String> = json
            .iter()
            .map(|star| (star.solar_system_id, star.name.clone()))
            .collect();
        let star_name_to_id: HashMap<String, SolarSystemId> = json
            .iter()
            .map(|star| (star.name.clone(), star.solar_system_id))
            .collect();

        Ok(Universe {
            star_map,
            star_id_to_name,
            star_name_to_id,
        })
    }

    pub fn load() -> anyhow::Result<Universe> {
        let bytes = std::fs::read("data/universe.rkyv")?;
        let archived = rkyv::access::<rkyv::Archived<Universe>, rkyv::rancor::Error>(&bytes)
            .map_err(|e| anyhow::anyhow!("Invalid archive format: {}", e))?;
        let universe: Universe = rkyv::deserialize::<Universe, rkyv::rancor::Error>(archived)?;

        Ok(universe)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)?;
        std::fs::write("data/universe.rkyv", bytes.as_ref())?;
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
        Universe::tiny_test_r().unwrap()
    }

    #[cfg(test)]
    pub fn tiny_test_r() -> anyhow::Result<Universe> {
        let raw_star_data = raw::RawStarMap::from_file("data_fixtures/starmap.json")?;
        let raw_smart_gates: Vec<raw::RawSmartGate> =
            serde_json::from_str(&std::fs::read_to_string("data_fixtures/smartgates.json")?)?;
        Universe::build_from_raw(raw_star_data, raw_smart_gates, Meters::new(50.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     *   0..1..2
     *    \ . /
     *  npc\./smart
     *      3
     */
    #[test]
    fn test_tiny_universe() {
        let universe = Universe::tiny_test();
        assert_eq!(universe.star_map.len(), 4);
        assert_eq!(universe.star_id_to_name.len(), 4);
        assert_eq!(universe.star_name_to_id.len(), 4);

        assert_eq!(universe.star_map[&1000].connections.len(), 4);
        assert_eq!(universe.star_map[&1001].connections.len(), 3);
        assert_eq!(universe.star_map[&1002].connections.len(), 4);
        assert_eq!(universe.star_map[&1003].connections.len(), 5);
    }

    #[test]
    fn test_distance() {
        let a = Star {
            id: 1,
            loc: [0.0, 0.0, 0.0],
            ..Default::default()
        };
        let b = Star {
            id: 2,
            loc: [1.0, 0.0, 0.0],
            ..Default::default()
        };
        assert_eq!(a.distance(&b), Meters::new(1.0));
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
            distance: Meters::new(2.0),
            target: 1,
        };
        let b = Connection {
            id: 2,
            conn_type: ConnType::NpcGate,
            distance: Meters::new(2.0),
            target: 1,
        };
        let c = Connection {
            id: 3,
            conn_type: ConnType::SmartGate,
            distance: Meters::new(2.0),
            target: 1,
        };
        let d = Connection {
            id: 4,
            conn_type: ConnType::Jump,
            distance: Meters::new(1.0),
            target: 1,
        };
        let e = Connection {
            id: 5,
            conn_type: ConnType::NpcGate,
            distance: Meters::new(1.0),
            target: 1,
        };
        let f = Connection {
            id: 6,
            conn_type: ConnType::SmartGate,
            distance: Meters::new(1.0),
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

    #[test]
    fn test_tiny_test_star1_sorting() {
        let universe = Universe::tiny_test();
        let star1 = &universe.star_map[&1000];

        // First connection should be the NPC gate to star 4
        assert_eq!(star1.connections[0].conn_type, ConnType::NpcGate);
        assert_eq!(star1.connections[0].target, 1003);

        // All NPC gates should come before all jumps
        let mut seen_jump = false;
        for conn in &star1.connections {
            if conn.conn_type == ConnType::Jump {
                seen_jump = true;
            } else if seen_jump {
                panic!("Found non-jump connection after jump: {:?}", conn);
            }
        }
    }
}
