use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uom::si::f64::*;
use uom::si::length::meter;

pub type ConnectionId = u64;
pub type SolarSystemId = u64;
pub type RegionId = u64;

/// IMPORTANT: Jump must come last, so that when we sort connections
/// by (type, distance) the long gates get sorted before the short jumps.
/// This allows us to stop searching for a path when we reach a jump that
/// is too long.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnType {
    NpcGate = 0,
    SmartGate = 1,
    Jump = 2,
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
    pub fn distance(&self, other: &Star) -> Length {
        Length::new::<meter>(
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
                .sqrt(),
        )
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
