use std::time::Instant;

use clap::{Parser, Subcommand};
use eftb::data;
use eftb::data::SolarSystemId;
use eftb::units::Meters;
use log::{info, warn};

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
            info!("Building star map");
            let max_jump_dist: Meters = Meters::from_light_years(*max_jump_distance);
            data::Universe::build(max_jump_dist)?;
            info!("Complete");
        }
        Some(Commands::Dist {
            start_name,
            end_name,
        }) => {
            info!("Loading star map");
            let universe = data::Universe::build(Meters::new(0.0))?;
            info!("Loaded star map");

            let start = universe.star_by_name(start_name)?;
            let end = universe.star_by_name(end_name)?;
            let distance: Meters = start.distance(end);
            println!(
                "Distance between {} and {} is {} LY",
                universe.star_id_to_name[&start.id],
                universe.star_id_to_name[&end.id],
                distance.to_light_years() as i32
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
            let jump_distance: Meters = Meters::from_light_years(*jump_distance);
            let universe = data::Universe::build(jump_distance)?;
            let start = universe.star_by_name(start_name)?;
            let end = universe.star_by_name(end_name)?;
            info!("Loaded star map in {:.3}", now.elapsed().as_secs_f64());

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
                            conn.distance.to_light_years() as i32
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
            let jump_distance: Meters = Meters::from_light_years(*jump_distance);
            let universe = data::Universe::build(jump_distance)?;
            info!("Loaded star map");

            let start = universe.star_by_name(start_name)?;

            info!("Finding exits");
            let exits = eftb::calc_exit(&universe, start, jump_distance);
            for (from, to) in exits {
                println!(
                    "{} -> {} ({} ly)",
                    universe.star_id_to_name[&from.id],
                    universe.star_id_to_name[&to.id],
                    from.distance(&to).to_light_years() as i32
                );
            }
        }
        Some(Commands::Star {
            name,
            jump_distance,
        }) => {
            info!("Loading star map");
            let max_jump_distance: Meters = if let Some(jd) = *jump_distance {
                Meters::from_light_years(jd)
            } else {
                Meters::new(0.0)
            };
            let universe = data::Universe::build(max_jump_distance)?;
            info!("Loaded star map");

            let star = universe.star_by_name(name)?;
            println!("{} ({}):", universe.star_id_to_name[&star.id], star.id);
            println!("  Connections:");
            for conn in &star.connections {
                if conn.conn_type != data::ConnType::Jump {
                    println!(
                        "    {} ({:?}, {} ly)",
                        universe.star_id_to_name[&conn.target],
                        conn.conn_type,
                        conn.distance.to_light_years() as i32
                    );
                }
            }
            if let Some(jump_distance) = jump_distance {
                println!("  Nearby stars:");
                for conn in &star.connections {
                    let d = conn.distance.to_light_years();
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
            let universe = data::Universe::build(Meters::new(0.0))?;
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
