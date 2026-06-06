use anyhow::Result;
use serde::Deserialize;

// ====================================================================
// Data structures for the starmap pickle extracted from the client
// ====================================================================

#[derive(Debug, Deserialize)]
pub struct RawStarMap {
    #[serde(rename(deserialize = "solarSystems"))]
    pub solar_systems: Vec<RawSolarSystem>,
    pub jumps: Vec<RawJump>,
}

impl RawStarMap {
    pub fn from_file(file: &str) -> Result<Self> {
        let file = std::fs::read_to_string(file)?;
        serde_json::from_str::<Self>(&file).map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct RawJump {
    #[serde(rename(deserialize = "fromSystemID"))]
    pub from_system_id: u32,
    #[serde(rename(deserialize = "toSystemID"))]
    pub to_system_id: u32,
    #[serde(rename(deserialize = "jumpType"))]
    pub jump_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct RawSolarSystem {
    pub name: String,
    pub center: [f64; 3],
    #[serde(rename(deserialize = "regionID"))]
    pub region_id: u32,
    #[serde(rename(deserialize = "solarSystemID"))]
    pub solar_system_id: u32,
}

// ====================================================================
// Data structures for smartgates.json
// ====================================================================

#[derive(Debug, Deserialize)]
pub struct RawSmartGate {
    pub id: String,
    #[serde(rename(deserialize = "itemId"))]
    pub item_id: u32,
    pub name: String,
    pub from: u32,
    pub to: u32,
}
