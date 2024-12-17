use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RawStarMap {
    pub constellations: HashMap<String, RawConstellation>,
    pub jumps: Vec<RawJump>,
    pub regions: HashMap<String, RawRegion>,
    #[serde(rename(deserialize = "solarSystems"))]
    pub solar_systems: HashMap<String, RawSolarSystem>,
}

impl RawStarMap {
    pub fn from_file(file: &str) -> Self {
        let file = std::fs::read_to_string(file).unwrap();
        serde_json::from_str(&file).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct RawConstellation {
    // pub center: [f64; 3],
    // pub neighbours: Vec<String>,
    // #[serde(rename(deserialize = "regionID"))]
    // pub region_id: u64,
    // #[serde(rename(deserialize = "solarSystemIDs"))]
    // pub solar_system_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RawJump {
    // #[serde(rename(deserialize = "fromCenter")]
    // from_center: RawJumpCenter,
    #[serde(rename(deserialize = "fromSystemID"))]
    pub from_system_id: u64,
    // #[serde(rename(deserialize = "jumpType"))]
    // pub jump_type: u8,
    // #[serde(rename(deserialize = "toCenter"))]
    // to_center: RawJumpCenter,
    #[serde(rename(deserialize = "toSystemID"))]
    pub to_system_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct RawRegion {
    // pub center: [f64; 3],
    //#[serde(rename(deserialize = "constellationIDs"))]
    //pub constellation_ids: Vec<u64>,
    //pub neighbours: Vec<String>,
    //#[serde(rename(deserialize = "solarSystemIDs"))]
    //pub solar_system_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RawSolarSystem {
    pub center: [f64; 3],
    //#[serde(rename(deserialize = "constellationID"))]
    //pub constellation_id: u64,
    //#[serde(rename(deserialize = "factionID"))]
    //pub faction_id: Option<u64>,
    //pub neighbours: Vec<String>,
    //#[serde(rename(deserialize = "planetCountByType"))]
    //pub planet_count_by_type: HashMap<String, u64>,
    //#[serde(rename(deserialize = "planetItemIDs"))]
    //pub planet_item_ids: Vec<u64>,
    //#[serde(rename(deserialize = "regionID"))]
    //pub region_id: u64,
    //#[serde(rename(deserialize = "sunTypeID"))]
    //pub sun_type_id: u64,
}
