use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uom::si::f64::*;
use uom::si::length::meter;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ConnType {
    NpcGate = 0,
    SmartGate = 1,
    Jump = 2,
}

type SolarSystemId = u64;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Connection {
    pub conn_type: ConnType,
    pub distance: Length,
    pub target: SolarSystemId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let data = std::fs::read_to_string("data/star-names.json")?;
    let json = serde_json::from_str::<HashMap<String, String>>(&data)?;
    let star_id_to_name: HashMap<SolarSystemId, String> = json
        .iter()
        .map(|(k, v)| (k.clone().parse().unwrap(), v.clone()))
        .collect();
    let star_name_to_id: HashMap<String, SolarSystemId> = star_id_to_name
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
    Ok((star_id_to_name, star_name_to_id))
}
