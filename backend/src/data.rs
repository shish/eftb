use std::collections::HashMap;

use indicatif::ProgressIterator;
use log::{info, warn};
use rayon::prelude::*;

use crate::raw;
use crate::units::Meters;

pub type ConnectionId = u32;
pub type SolarSystemId = u32;
pub type StarIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnType {
    NpcGate,
    SmartGate,
    Jump,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: ConnectionId,
    pub conn_type: ConnType,
    pub distance: Meters,
    pub target: StarIdx,
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

#[derive(Clone, Default)]
pub struct Star {
    pub name: String,
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

    pub fn bucket(&self, bucket_size: Meters) -> String {
        format!(
            "{}-{}-{}",
            (self.loc[0] / bucket_size.get()).floor() as i64,
            (self.loc[1] / bucket_size.get()).floor() as i64,
            (self.loc[2] / bucket_size.get()).floor() as i64,
        )
    }

    pub fn nearby_buckets(&self, bucket_size: Meters) -> Vec<String> {
        let mut buckets = Vec::new();
        let base_x = (self.loc[0] / bucket_size.get()).floor() as i64;
        let base_y = (self.loc[1] / bucket_size.get()).floor() as i64;
        let base_z = (self.loc[2] / bucket_size.get()).floor() as i64;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    buckets.push(format!("{}-{}-{}", base_x + dx, base_y + dy, base_z + dz));
                }
            }
        }

        buckets
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

#[derive(Debug, Clone)]
pub struct Universe {
    pub stars: Vec<Star>,
    pub star_id_to_idx: HashMap<SolarSystemId, StarIdx>,
    pub star_name_to_idx: HashMap<String, StarIdx>,
}
impl Universe {
    pub fn build(max_jump_dist: Meters) -> anyhow::Result<Universe> {
        let t = std::time::Instant::now();
        let raw_star_data = raw::RawStarMap::from_file("data/starmap.json")?;
        let raw_smart_gates: Vec<raw::RawSmartGate> =
            serde_json::from_str(&std::fs::read_to_string("data/smartgates.json")?)?;
        info!("Loaded raw data in {:.2}s", t.elapsed().as_secs_f64());
        Universe::build_from_raw(raw_star_data, raw_smart_gates, max_jump_dist)
    }

    pub fn build_from_raw(
        raw_star_data: raw::RawStarMap,
        raw_smart_gates: Vec<raw::RawSmartGate>,
        max_jump_dist: Meters,
    ) -> anyhow::Result<Universe> {
        let t = std::time::Instant::now();
        let n = raw_star_data.solar_systems.len();
        let mut star_id_to_idx: HashMap<SolarSystemId, StarIdx> = HashMap::with_capacity(n);
        let mut stars: Vec<Star> = Vec::with_capacity(n);
        let mut star_name_to_idx: HashMap<String, StarIdx> = HashMap::with_capacity(n);
        let mut star_bucket_to_idx: HashMap<String, Vec<StarIdx>> = HashMap::new();

        for (idx, raw_star) in raw_star_data.solar_systems.iter().enumerate() {
            let star = Star {
                name: raw_star.name.clone(),
                id: raw_star.solar_system_id,
                loc: raw_star.center,
                connections: Vec::new(),
            };
            star_bucket_to_idx
                .entry(star.bucket(max_jump_dist))
                .or_insert_with(Vec::new)
                .push(idx);
            stars.push(star);
            star_id_to_idx.insert(raw_star.solar_system_id, idx);
            star_name_to_idx.insert(raw_star.name.clone(), idx);
        }
        info!(
            "Built star map with {} stars in {} buckets in {:.2}s",
            stars.len(),
            star_bucket_to_idx.len(),
            t.elapsed().as_secs_f64()
        );

        let t = std::time::Instant::now();
        let mut conn_count = 1; // Connection #0 is reserved for path init
        for raw_jump in raw_star_data.jumps.iter() {
            // rust only lets us borrow one mutable star at a time, so we can't add
            // from->to and to->from gates in the same block
            for (fid, tid) in [
                (raw_jump.from_system_id, raw_jump.to_system_id),
                (raw_jump.to_system_id, raw_jump.from_system_id),
            ] {
                let Some(to_star_idx) = star_id_to_idx.get(&tid).cloned() else {
                    warn!("Jump has unknown target {}", tid);
                    continue;
                };
                let Some(from_star_idx) = star_id_to_idx.get(&fid).cloned() else {
                    warn!("Jump has unknown source {}", fid);
                    continue;
                };

                let distance = stars[from_star_idx].distance(&stars[to_star_idx]);
                stars[from_star_idx].connections.push(Connection {
                    id: conn_count,
                    conn_type: ConnType::NpcGate,
                    distance: distance,
                    target: to_star_idx,
                });
                conn_count += 1;
            }
        }
        info!(
            "Built connections from npc gates in {:.2}s",
            t.elapsed().as_secs_f64()
        );

        let t = std::time::Instant::now();
        for gate in raw_smart_gates.iter() {
            let Some(to_star_idx) = star_id_to_idx.get(&gate.to).cloned() else {
                warn!("Smart gate has unknown target {}", gate.to);
                continue;
            };
            let Some(from_star_idx) = star_id_to_idx.get(&gate.from).cloned() else {
                warn!("Smart gate has unknown source {}", gate.from);
                continue;
            };

            let distance = stars[from_star_idx].distance(&stars[to_star_idx]);
            stars[from_star_idx].connections.push(Connection {
                id: conn_count,
                conn_type: ConnType::SmartGate,
                distance: distance,
                target: to_star_idx,
            });
            conn_count += 1;
        }
        info!(
            "Built connections from smart gates in {:.2}s",
            t.elapsed().as_secs_f64()
        );

        let t = std::time::Instant::now();
        for from_star_idx in (0..n).progress() {
            for nearby_bucket in stars[from_star_idx].nearby_buckets(max_jump_dist) {
                if !star_bucket_to_idx.contains_key(&nearby_bucket) {
                    continue;
                }
                for to_star_idx in star_bucket_to_idx[&nearby_bucket].iter() {
                    if from_star_idx == *to_star_idx {
                        continue;
                    }
                    let distance = stars[from_star_idx].distance(&stars[*to_star_idx]);
                    if distance < max_jump_dist {
                        stars[from_star_idx].connections.push(Connection {
                            id: conn_count,
                            conn_type: ConnType::Jump,
                            distance: distance,
                            target: *to_star_idx,
                        });
                        conn_count += 1;
                    }
                }
            }
        }
        info!(
            "Built connections from jumps in {:.2}s",
            t.elapsed().as_secs_f64()
        );

        // sort gates first, and then jumps by distance - then when we
        // reach a jump that is too long we can stop searching
        let t = std::time::Instant::now();
        stars.par_iter_mut().for_each(|star| {
            star.connections.sort_unstable();
        });
        info!(
            "Sorted {} connections in {:.2}s",
            conn_count,
            t.elapsed().as_secs_f64()
        );

        Ok(Universe {
            stars,
            star_id_to_idx,
            star_name_to_idx,
        })
    }

    pub fn star_by_name(&self, name: &String) -> anyhow::Result<&Star> {
        let star_idx = self
            .star_name_to_idx
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Star not found: {}", name))?;
        Ok(&self.stars[*star_idx])
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
        assert_eq!(universe.stars.len(), 4);
        assert_eq!(universe.star_name_to_idx.len(), 4);

        assert_eq!(universe.stars[0].connections.len(), 4);
        assert_eq!(universe.stars[1].connections.len(), 3);
        assert_eq!(universe.stars[2].connections.len(), 4);
        assert_eq!(universe.stars[3].connections.len(), 5);
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
        let star1 = &universe.stars[0];

        // First connection should be the NPC gate to star 4
        assert_eq!(star1.connections[0].conn_type, ConnType::NpcGate);
        assert_eq!(star1.connections[0].target, 3);

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
