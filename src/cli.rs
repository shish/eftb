use bincode;
use clap::{Parser, Subcommand};
use indicatif::ProgressIterator;
use log::{info, warn};
use std::collections::HashMap;

use uom::si::f64::*;
use uom::si::length::light_year;

mod calcs;
mod data;
mod raw;

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
    /// Figure out how far a given ship can jump
    Jump {
        mass: f64,
        fuel: f64,
        #[clap(default_value = "0.4")]
        efficiency: f64,
    },
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
            let mut star_map: HashMap<u64, data::Star> = HashMap::new();
            for (id_str, raw_star) in raw_star_data.solar_systems.iter() {
                let id = id_str.parse()?;
                let star = data::Star {
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
                // rust only lets us borrow one mutable star at a time, so we can't add
                // from->to and to->from gates in the same block
                for (fid, tid) in [
                    (raw_jump.from_system_id, raw_jump.to_system_id),
                    (raw_jump.to_system_id, raw_jump.from_system_id),
                ] {
                    let to_star = star_map.get(&tid).unwrap().clone();
                    let from_star = star_map.get_mut(&fid).unwrap();
                    let distance: Length = from_star.distance(&to_star);
                    from_star.connections.push(data::Connection {
                        conn_type: data::ConnType::NpcGate,
                        distance,
                        target: tid,
                    });
                }
            }

            let cloned_star_map = star_map.clone();
            for star in star_map.values_mut().progress() {
                for other_star in cloned_star_map.values() {
                    if star.id == other_star.id {
                        continue;
                    }
                    let distance: Length = star.distance(&other_star);
                    if distance < max_jump_dist {
                        star.connections.push(data::Connection {
                            conn_type: data::ConnType::Jump,
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
            let (star_id_to_name, star_name_to_id) = data::get_name_maps()?;
            let star_map: HashMap<u64, data::Star> =
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
                "Distance between {} and {} is {} LY",
                star_id_to_name[&start.id],
                star_id_to_name[&end.id],
                distance.get::<light_year>() as i32
            );
        }
        Some(Commands::Path {
            start_name,
            end_name,
            jump_distance,
        }) => {
            info!("Loading star map");
            let (star_id_to_name, star_name_to_id) = data::get_name_maps()?;
            let star_map: HashMap<u64, data::Star> =
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
            let path = calcs::calc_path(&star_map, start, end, jump_distance);
            if let Some(path) = path {
                let mut last = start.clone();
                for star in path {
                    println!(
                        "{} ({} ly)",
                        star_id_to_name[&star.id],
                        last.distance(&star).get::<light_year>() as i32
                    );
                    last = star;
                }
            } else {
                warn!("No path found");
            }
        }
        Some(Commands::Jump {
            mass,
            fuel,
            efficiency,
        }) => {
            let dist: Length = calcs::calc_jump(*mass, *fuel, *efficiency);
            println!("Jump distance: {:.0} ly", dist.get::<light_year>())
        }
        None => {
            warn!("No command specified");
        }
    }

    Ok(())
}
