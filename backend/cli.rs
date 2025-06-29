use std::collections::HashMap;
use std::time::Instant;

use clap::{Parser, Subcommand};
use eftb::data::SolarSystemId;
use indicatif::ProgressIterator;
use log::{info, warn};
use uom::si::f64::*;
use uom::si::length::light_year;

use eftb::data;
use eftb::raw;

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
        #[clap(default_value = "500.0")]
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
        #[clap(short, long, default_value = "100.0")]
        jump_distance: f64,
        #[clap(short, long, default_value = "fuel")]
        optimize: eftb::calc::path::PathOptimize,
        #[clap(short, long)]
        use_smart_gates: bool,
    },
    /// Find the exits from a given point
    Exits {
        start_name: String,
        #[clap(short, long, default_value = "100.0")]
        jump_distance: f64,
    },
    /// Show info about a solar system
    Star {
        name: String,
        #[clap(short, long)]
        jump_distance: Option<f64>,
    },
    /// Constellation
    Constellation { name: String },
}

fn main() -> anyhow::Result<()> {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build { max_jump_distance }) => {
            info!("Loading raw data");
            let raw_star_data = raw::RawStarMap::from_file("data/starmap.json");
            let max_jump_dist: Length = Length::new::<light_year>(*max_jump_distance);

            info!("Building star map");
            let mut star_map: HashMap<data::SolarSystemId, data::Star> = HashMap::new();
            for (id_str, raw_star) in raw_star_data.solar_systems.iter() {
                let id = id_str.parse()?;
                let star = data::Star {
                    id,
                    region_id: raw_star.region_id,
                    x: raw_star.center[0],
                    y: raw_star.center[1],
                    z: raw_star.center[2],
                    connections: Vec::new(),
                };
                star_map.insert(id, star);
            }

            info!("Building connections from npc gates");
            let mut conn_count = 0;
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
                    let conn_type = match raw_jump.jump_type {
                        0 => data::ConnType::NpcGate,
                        1 => data::ConnType::NpcGate, // What are these ???
                        _ => {
                            info!(
                                "{} -> {} is an unknown jump type ({})",
                                fid, tid, raw_jump.jump_type
                            );
                            continue;
                        }
                    };
                    from_star.connections.push(data::Connection {
                        id: conn_count,
                        conn_type,
                        distance,
                        target: tid,
                    });
                    conn_count += 1;
                }
            }

            info!("Building connections from smart gates");
            let smart_gates: Vec<raw::RawSmartGate> =
                serde_json::from_str(&std::fs::read_to_string("data/smartgates.json")?)?;
            for gate in smart_gates.iter() {
                if !star_map.contains_key(&gate.from) {
                    warn!("Smart gate has unknown source {}", gate.from);
                    continue;
                }
                if !star_map.contains_key(&gate.to) {
                    warn!("Smart gate has unknown target {}", gate.to);
                    continue;
                }
                let to_star = star_map.get(&gate.to).unwrap().clone();
                let from_star = star_map.get_mut(&gate.from).unwrap();
                let distance: Length = from_star.distance(&to_star);
                from_star.connections.push(data::Connection {
                    id: conn_count,
                    conn_type: data::ConnType::SmartGate,
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
                    let distance: Length = star.distance(&other_star);
                    if distance < max_jump_dist {
                        star.connections.push(data::Connection {
                            id: conn_count,
                            conn_type: data::ConnType::Jump,
                            distance,
                            target: other_star.id,
                        });
                        conn_count += 1;
                    }
                }
            }

            info!("Sorting connections");
            // sort gates first, and then jumps by distance - then when we
            // reach a jump that is too long we can stop searching
            for star in star_map.values_mut().progress() {
                star.connections.sort_unstable();
            }

            info!("Saving star map");
            let u = data::Universe {
                star_map,
                star_id_to_name: HashMap::new(),
                star_name_to_id: HashMap::new(),
            };
            u.save()?;
            info!("Complete");
        }
        Some(Commands::Dist {
            start_name,
            end_name,
        }) => {
            info!("Loading star map");
            let universe = data::Universe::load()?;
            info!("Loaded star map");

            let start = universe.star_by_name(start_name)?;
            let end = universe.star_by_name(end_name)?;
            let distance: Length = start.distance(end);
            println!(
                "Distance between {} and {} is {} LY",
                universe.star_id_to_name[&start.id],
                universe.star_id_to_name[&end.id],
                distance.get::<light_year>() as i32
            );
        }
        Some(Commands::Path {
            start_name,
            end_name,
            jump_distance,
            optimize,
            use_smart_gates,
        }) => {
            info!("Loading star map");
            let now = Instant::now();
            let universe = data::Universe::load()?;
            info!("Loaded star map in {:.3}", now.elapsed().as_secs_f64());

            let start = universe.star_by_name(start_name)?;
            let end = universe.star_by_name(end_name)?;
            let jump_distance: Length = Length::new::<light_year>(*jump_distance);

            info!("Finding path");
            let now = Instant::now();
            let path = eftb::calc_path(
                &universe,
                start,
                end,
                jump_distance,
                *optimize,
                *use_smart_gates,
                Some(30),
            );
            info!("Found path in {:.3}", now.elapsed().as_secs_f64());
            match path {
                eftb::calc::path::PathResult::Found(path) => {
                    println!("Path from {} to {}:", start_name, end_name);
                    let mut last_id = start.id;
                    for conn in path {
                        println!(
                            "{} -> {} ({:?}, {} ly)",
                            universe.star_id_to_name[&last_id],
                            universe.star_id_to_name[&conn.target],
                            conn.conn_type,
                            conn.distance.get::<light_year>() as i32
                        );
                        last_id = conn.target;
                    }
                }
                eftb::calc::path::PathResult::NotFound => {
                    warn!("No path found");
                }
                eftb::calc::path::PathResult::Timeout => {
                    warn!("Path search timed out");
                }
            }
        }
        Some(Commands::Exits {
            start_name,
            jump_distance,
        }) => {
            info!("Loading star map");
            let universe = data::Universe::load()?;
            info!("Loaded star map");

            let start = universe.star_by_name(start_name)?;
            let jump_distance: Length = Length::new::<light_year>(*jump_distance);

            info!("Finding exits");
            let exits = eftb::calc_exit(&universe, start, jump_distance);
            for (from, to) in exits {
                println!(
                    "{} -> {} ({}) ({} ly)",
                    universe.star_id_to_name[&from.id],
                    universe.star_id_to_name[&to.id],
                    &to.region_id,
                    from.distance(&to).get::<light_year>() as i32
                );
                for conn in &universe.star_map[&to.id].connections {
                    let d = conn.distance.get::<light_year>();
                    if conn.conn_type == data::ConnType::Jump
                        && conn.distance < jump_distance
                        && universe.star_map[&conn.target].region_id != start.region_id
                    {
                        println!(
                            "  -> {} ({} ly)",
                            universe.star_id_to_name[&conn.target], d as i32
                        );
                    }
                }
            }
        }
        Some(Commands::Star {
            name,
            jump_distance,
        }) => {
            info!("Loading star map");
            let universe = data::Universe::load()?;
            info!("Loaded star map");

            let star = universe.star_by_name(name)?;
            println!("{} ({}):", universe.star_id_to_name[&star.id], star.id);
            println!("  Region: {}", star.region_id);
            println!("  Connections:");
            for conn in &star.connections {
                if conn.conn_type != data::ConnType::Jump {
                    println!(
                        "    {} ({:?}, {} ly)",
                        universe.star_id_to_name[&conn.target],
                        conn.conn_type,
                        conn.distance.get::<light_year>() as i32
                    );
                }
            }
            if let Some(jump_distance) = jump_distance {
                println!("  Nearby stars:");
                for conn in &star.connections {
                    let d = conn.distance.get::<light_year>();
                    if conn.conn_type == data::ConnType::Jump && d < *jump_distance {
                        println!(
                            "    {} ({} ly)",
                            universe.star_id_to_name[&conn.target], d as i32
                        );
                    }
                }
            }
        }
        Some(Commands::Constellation { name }) => {
            info!("Loading star map");
            let universe = data::Universe::load()?;
            info!("Loaded star map");

            let star = universe.star_by_name(name)?;
            let mut visited: Vec<SolarSystemId> = Vec::new();
            let mut to_visit: Vec<SolarSystemId> = vec![star.id];

            while let Some(id) = to_visit.pop() {
                if visited.contains(&id) {
                    continue;
                }
                println!("{}", universe.star_id_to_name[&id]);
                visited.push(id);
                for conn in &universe.star_map[&id].connections {
                    if conn.conn_type != data::ConnType::Jump {
                        to_visit.push(conn.target);
                    }
                }
            }
        }
        None => {
            warn!("No command specified");
        }
    }

    Ok(())
}
