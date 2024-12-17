use clap::{Parser, Subcommand};
use indicatif::ProgressIterator;
use log::{info, warn};
use pathfinding::prelude::astar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod raw;

const M_IN_LIGHT_YEAR: f64 = 9.461e15;
const MAX_JUMP_DIST: f64 = 500.0 * M_IN_LIGHT_YEAR;

#[derive(Debug, Deserialize, Serialize, Clone)]
enum ConnType {
    NpcGate = 0,
    SmartGate = 1,
    Jump = 2,
}

type SolarSystemId = u64;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Connection {
    conn_type: ConnType,
    distance: f64,
    target: SolarSystemId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Star {
    id: SolarSystemId,
    x: f64,
    y: f64,
    z: f64,
    connections: Vec<Connection>,
}

impl Star {
    fn distance(&self, other: &Star) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the starmap from star_data.json and star_names.json
    Build {},
    /// Find the direct distance between two stars
    Dist {
        star_name_1: String,
        star_name_2: String,
    },
    /// Find the shortest path between two stars
    Path {
        star_name_1: String,
        star_name_2: String,
        jump_distance: Option<f64>,
    },
}

fn get_name_maps() -> anyhow::Result<(
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

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build {}) => {
            info!("Loading raw data");
            let raw_star_data = raw::RawStarMap::from_file("data/extracted-starmap.json");

            info!("Building star map");
            let mut star_map: HashMap<u64, Star> = HashMap::new();
            for (id_str, raw_star) in raw_star_data.solar_systems.iter() {
                let id = id_str.parse().unwrap();
                let star = Star {
                    id,
                    x: raw_star.center[0],
                    y: raw_star.center[1],
                    z: raw_star.center[2],
                    connections: Vec::new(),
                };
                star_map.insert(id, star);
            }

            info!("Building connections");
            for raw_jump in raw_star_data.jumps.iter() {
                let to_star = star_map.get(&raw_jump.to_system_id).unwrap().clone();
                let from_star = star_map.get_mut(&raw_jump.from_system_id).unwrap();
                let distance = from_star.distance(&to_star);
                from_star.connections.push(Connection {
                    conn_type: ConnType::NpcGate,
                    distance,
                    target: raw_jump.to_system_id,
                });
                /*
                to_star.connections.push(Connection {
                    conn_type: ConnType::NpcGate,
                    distance,
                    target: from_star.name.clone(),
                });
                */
            }

            let cloned_star_map = star_map.clone();
            for star in star_map.values_mut().progress() {
                let mut n = 0;
                for other_star in cloned_star_map.values() {
                    if star.id == other_star.id {
                        continue;
                    }
                    let distance = star.distance(&other_star);
                    if distance < MAX_JUMP_DIST {
                        star.connections.push(Connection {
                            conn_type: ConnType::Jump,
                            distance,
                            target: other_star.id,
                        });
                        n += 1;
                        if n > 100 {
                            //    break;
                        }
                    }
                }
            }

            info!("Saving star map");
            let star_map_json = serde_json::to_string_pretty(&star_map).unwrap();
            std::fs::write("data/star_map.json", star_map_json).unwrap();
            info!("Complete");
        }
        Some(Commands::Dist {
            star_name_1,
            star_name_2,
        }) => {
            info!("Loading star map");
            let (star_id_to_name, star_name_to_id) = get_name_maps().unwrap();
            let star_map: HashMap<u64, Star> =
                serde_json::from_str(&std::fs::read_to_string("data/star_map.json").unwrap())
                    .unwrap();
            info!("Loaded star map");

            let start = star_map
                .get(star_name_to_id.get(star_name_1).unwrap())
                .unwrap();
            let end = star_map
                .get(star_name_to_id.get(star_name_2).unwrap())
                .unwrap();

            let distance = start.distance(end);
            println!(
                "Distance between {} and {} is {:.2} LY",
                star_id_to_name[&start.id],
                star_id_to_name[&end.id],
                distance / M_IN_LIGHT_YEAR
            );
        }
        Some(Commands::Path {
            star_name_1,
            star_name_2,
            jump_distance,
        }) => {
            info!("Loading star map");
            let (star_id_to_name, star_name_to_id) = get_name_maps().unwrap();
            let star_map: HashMap<String, Star> =
                serde_json::from_str(&std::fs::read_to_string("data/star_map.json").unwrap())
                    .unwrap();
            info!("Loaded star map");
            /*
            let start = star_map
                .get(star_name_to_id.get(star_name_1).unwrap())
                .unwrap();
            let end = star_map
                .get(star_name_to_id.get(star_name_2).unwrap())
                .unwrap();
            let _jump_distance = jump_distance.unwrap_or(MAX_JUMP_DIST) as i64;

            let distance = start.distance(end);
            println!(
                "Distance between {} and {} is {:.2} LY",
                star_id_to_name[&start.id],
                star_id_to_name[&end.id],
                distance / M_IN_LIGHT_YEAR
            );

            info!("Finding path");
            fn successors(star_map: &HashMap<String, Star>, star: &Star) -> Vec<(Star, i64)> {
                star.connections
                    .iter()
                    .map(|c| {
                        (
                            star_map.get(&c.target).unwrap().clone(),
                            (c.distance / M_IN_LIGHT_YEAR) as i64,
                        )
                    })
                    .collect()
            }
            let result = astar(
                start,
                |star| successors(&star_map, star),
                |star| (star.distance(end) / M_IN_LIGHT_YEAR / 3.0) as i64,
                |star| star.id == end.id,
            );
            if let Some((result, _)) = result {
                let mut last = start.clone();
                for star in result {
                    println!(
                        "{} ({} LY)",
                        star.name,
                        last.distance(&star) / M_IN_LIGHT_YEAR
                    );
                    last = star;
                }
            } else {
                warn!("No path found");
            }*/
        }
        None => {
            warn!("No command specified");
        }
    }
}
