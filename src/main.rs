use bincode;
use clap::{Parser, Subcommand};
use indicatif::ProgressIterator;
use log::{info, warn};
use pathfinding::prelude::astar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use uom::si::f64::*;
use uom::si::length::{light_year, meter};

mod raw;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
enum ConnType {
    NpcGate = 0,
    SmartGate = 1,
    Jump = 2,
}

type SolarSystemId = u64;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Connection {
    conn_type: ConnType,
    distance: Length,
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
    fn distance(&self, other: &Star) -> Length {
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
    Build {
        #[clap(default_value = "100.0")]
        max_jump_distance: f64,
    },
    /// Find the direct distance between two stars
    Dist {
        start_name: String,
        end_name: String,
    },
    /// Find the shortest path between two stars
    Path {
        start_name: String,
        end_name: String,
        #[clap(default_value = "100.0")]
        jump_distance: f64,
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

fn main() -> anyhow::Result<()> {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build { max_jump_distance }) => {
            info!("Loading raw data");
            let raw_star_data = raw::RawStarMap::from_file("data/extracted-starmap.json");
            let max_jump_dist: Length = Length::new::<light_year>(*max_jump_distance);

            info!("Building star map");
            let mut star_map: HashMap<u64, Star> = HashMap::new();
            for (id_str, raw_star) in raw_star_data.solar_systems.iter() {
                let id = id_str.parse()?;
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
                let distance: Length = from_star.distance(&to_star);
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
                for other_star in cloned_star_map.values() {
                    if star.id == other_star.id {
                        continue;
                    }
                    let distance: Length = star.distance(&other_star);
                    if distance < max_jump_dist {
                        star.connections.push(Connection {
                            conn_type: ConnType::Jump,
                            distance,
                            target: other_star.id,
                        });
                    }
                }
            }

            info!("Saving star map");
            // std::fs::write("data/starmap.json", serde_json::to_string(&star_map)?)?;
            std::fs::write("data/starmap.bin", bincode::serialize(&star_map)?)?;
            info!("Complete");
        }
        Some(Commands::Dist {
            start_name,
            end_name,
        }) => {
            info!("Loading star map");
            let (star_id_to_name, star_name_to_id) = get_name_maps()?;
            let star_map: HashMap<u64, Star> =
                bincode::deserialize(&std::fs::read("data/starmap.bin")?)?;
            info!("Loaded star map");

            let start = star_map
                .get(star_name_to_id.get(start_name).unwrap())
                .unwrap();
            let end = star_map
                .get(star_name_to_id.get(end_name).unwrap())
                .unwrap();

            let distance: Length = start.distance(end);
            println!(
                "Distance between {} and {} is {:.2} LY",
                star_id_to_name[&start.id],
                star_id_to_name[&end.id],
                distance.get::<light_year>()
            );
        }
        Some(Commands::Path {
            start_name,
            end_name,
            jump_distance,
        }) => {
            info!("Loading star map");
            let (star_id_to_name, star_name_to_id) = get_name_maps()?;
            let star_map: HashMap<u64, Star> =
                bincode::deserialize(&std::fs::read("data/starmap.bin")?)?;
            info!("Loaded star map");

            let start = star_map
                .get(star_name_to_id.get(start_name).unwrap())
                .unwrap();
            let end = star_map
                .get(star_name_to_id.get(end_name).unwrap())
                .unwrap();
            let jump_distance: Length = Length::new::<light_year>(*jump_distance);

            info!("Finding path");
            fn successors(
                star_map: &HashMap<u64, Star>,
                star: &Star,
                jump_distance: Length,
            ) -> Vec<(Star, i64)> {
                star.connections
                    .iter()
                    .filter_map(|c| {
                        if c.conn_type == ConnType::Jump && c.distance > jump_distance {
                            return None;
                        }
                        Some((
                            star_map.get(&c.target).unwrap().clone(),
                            c.distance.get::<light_year>() as i64,
                        ))
                    })
                    .collect()
            }
            let result = astar(
                start,
                |star| successors(&star_map, star, jump_distance),
                |star| (star.distance(end).get::<light_year>() / 3.0) as i64,
                |star| star.id == end.id,
            );
            if let Some((result, _)) = result {
                let mut last = start.clone();
                for star in result {
                    println!(
                        "{} ({} LY)",
                        star_id_to_name[&star.id],
                        last.distance(&star).get::<light_year>()
                    );
                    last = star;
                }
            } else {
                warn!("No path found");
            }
        }
        None => {
            warn!("No command specified");
        }
    }

    Ok(())
}
