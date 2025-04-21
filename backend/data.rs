use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type ConnectionId = u64;
pub type SolarSystemId = u64;
pub type RegionId = u64;
const M_PER_LY: f64 = 9.461e15;

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
    pub distance: f64,
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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Star {
    pub id: SolarSystemId,
    pub region_id: RegionId,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub connections: Vec<Connection>,
}

impl Star {
    pub fn distance(&self, other: &Star) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
            / M_PER_LY
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

pub fn get_name_maps() -> anyhow::Result<(
    HashMap<SolarSystemId, String>,
    HashMap<String, SolarSystemId>,
)> {
    let data = std::fs::read_to_string("data/solarsystems.json")?;
    let json = serde_json::from_str::<HashMap<String, crate::raw::RawStar>>(&data)?;
    let star_id_to_name: HashMap<SolarSystemId, String> = json
        .values()
        .map(|star| (star.solar_system_id, star.solar_system_name.clone()))
        .collect();
    let star_name_to_id: HashMap<String, SolarSystemId> = json
        .values()
        .map(|star| (star.solar_system_name.clone(), star.solar_system_id))
        .collect();
    Ok((star_id_to_name, star_name_to_id))
}

pub fn save_star_map(star_map: &HashMap<SolarSystemId, Star>) -> anyhow::Result<()> {
    // std::fs::write("data/starmap.json", serde_json::to_string(&star_map)?)?;
    std::fs::write("data/starmap.bin", bincode::serialize(&star_map)?)?;
    Ok(())
}

pub fn get_star_map() -> anyhow::Result<HashMap<SolarSystemId, Star>> {
    let map: HashMap<SolarSystemId, Star> =
        bincode::deserialize(&std::fs::read("data/starmap.bin")?)?;
    Ok(map)
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
            x: 9.461e+15,
            y: 0.0,
            z: 0.0,
            ..Default::default()
        };
        assert_eq!(a.distance(&b), 1.0);
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
            distance: 2.0,
            target: 1,
        };
        let b = Connection {
            id: 2,
            conn_type: ConnType::NpcGate,
            distance: 2.0,
            target: 1,
        };
        let c = Connection {
            id: 3,
            conn_type: ConnType::SmartGate,
            distance: 2.0,
            target: 1,
        };
        let d = Connection {
            id: 4,
            conn_type: ConnType::Jump,
            distance: 1.0,
            target: 1,
        };
        let e = Connection {
            id: 5,
            conn_type: ConnType::NpcGate,
            distance: 1.0,
            target: 1,
        };
        let f = Connection {
            id: 6,
            conn_type: ConnType::SmartGate,
            distance: 1.0,
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
