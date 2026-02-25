#[macro_use]
extern crate rocket;

use std::io::Cursor;
use std::path::Path;

use eftb::calc::path::PathOptimize;
use eftb::data;
use eftb::data::{ConnType, SolarSystemId, Star};
use eftb::units::Meters;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

// ====================================================================
// common

//#[derive(Error)]
#[derive(Debug, Clone)]
pub struct CustomError(pub Status, pub String);

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.0)
            .header(ContentType::Text)
            .sized_body(self.1.len(), Cursor::new(self.1))
            .ok()
    }
}

impl From<anyhow::Error> for CustomError {
    fn from(err: anyhow::Error) -> Self {
        CustomError(Status::InternalServerError, format!("{:?}", err))
    }
}

fn get_star(universe: &data::Universe, name: String) -> Result<&Star, CustomError> {
    let star = universe.star_by_name(&name).or(Err(CustomError(
        Status::NotFound,
        format!("Solar system {} not found", name),
    )))?;
    Ok(star)
}

#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./dist").join("index.html"))
        .await
        .ok()
}

// ====================================================================
// get_stars

#[derive(Debug, Serialize)]
struct StarsReturn {
    version: u32,
    data: Vec<String>,
}

#[get("/stars")]
fn get_stars(universe: &State<data::Universe>) -> Json<StarsReturn> {
    let names = universe.star_name_to_id.keys().cloned().collect();
    Json(StarsReturn {
        version: 1,
        data: names,
    })
}

// ====================================================================
// calc_dist

#[derive(Debug, Serialize)]
struct DistReturn {
    version: u32,
    data: f64,
}

#[get("/dist?<start>&<end>")]
fn calc_dist(
    universe: &State<data::Universe>,
    start: String,
    end: String,
) -> Result<Json<DistReturn>, CustomError> {
    let start = get_star(universe, start)?;
    let end = get_star(universe, end)?;
    let dist: Meters = start.distance(end);
    Ok(Json(DistReturn {
        version: 1,
        data: dist.to_light_years(),
    }))
}

// ====================================================================
// calc_path

#[derive(Debug, Serialize)]
struct WebStar {
    id: SolarSystemId,
    name: String,
}
#[derive(Debug, Serialize)]
struct PathStep {
    from: WebStar,
    conn_type: String,
    distance: f64,
    to: WebStar,
}
#[derive(Debug, Serialize)]
enum PathResult {
    Found(Vec<PathStep>),
    Timeout,
    NotFound,
}
#[derive(Debug, Serialize)]
struct PathReturn {
    version: u32,
    data: PathResult,
}

#[get("/path?<start>&<end>&<jump>&<optimize>&<use_smart_gates>")]
fn calc_path(
    universe: &State<data::Universe>,
    start: String,
    end: String,
    jump: f64,
    optimize: String,
    use_smart_gates: bool,
) -> Result<Json<PathReturn>, CustomError> {
    let start = get_star(universe, start)?;
    let end = get_star(universe, end)?;
    let optimize = match optimize.as_str() {
        "fuel" => PathOptimize::Fuel,
        "distance" => PathOptimize::Distance,
        "hops" => PathOptimize::Hops,
        _ => {
            return Err(CustomError(
                Status::BadRequest,
                format!("Invalid optimize value"),
            ))
        }
    };

    let result = eftb::calc_path(
        &universe,
        start,
        end,
        Meters::from_light_years(jump),
        optimize,
        use_smart_gates,
        Some(5),
    );
    match result {
        eftb::calc::path::PathResult::Found(path) => {
            let mut result = Vec::new();
            let mut last_id = start.id;
            for conn in path {
                result.push(PathStep {
                    from: WebStar {
                        id: last_id,
                        name: universe.star_id_to_name[&last_id].clone(),
                    },
                    conn_type: match conn.conn_type {
                        ConnType::Jump => "jump".to_string(),
                        ConnType::NpcGate => "npc_gate".to_string(),
                        ConnType::SmartGate => "smart_gate".to_string(),
                    },
                    distance: conn.distance.to_light_years() as f64,
                    to: WebStar {
                        id: conn.target,
                        name: universe.star_id_to_name[&conn.target].clone(),
                    },
                });
                last_id = conn.target;
            }
            Ok(Json(PathReturn {
                version: 3,
                data: PathResult::Found(result),
            }))
        }
        eftb::calc::path::PathResult::Timeout => Ok(Json(PathReturn {
            version: 3,
            data: PathResult::Timeout,
        })),
        eftb::calc::path::PathResult::NotFound => Ok(Json(PathReturn {
            version: 3,
            data: PathResult::NotFound,
        })),
    }
}

// ====================================================================
// calc_exit

#[derive(Debug, Serialize)]
struct ExitReturn {
    version: u32,
    data: Vec<(String, String, f64)>,
}

#[get("/exit?<start>&<jump>")]
fn calc_exit(
    universe: &State<data::Universe>,
    start: String,
    jump: f64,
) -> Result<Json<ExitReturn>, CustomError> {
    let start = get_star(universe, start)?;

    let exits = eftb::calc_exit(&universe, start, Meters::from_light_years(jump));

    let mut result: Vec<(String, String, f64)> = Vec::new();
    for (from, to) in exits {
        result.push((
            universe.star_id_to_name[&from.id].clone(),
            universe.star_id_to_name[&to.id].clone(),
            from.distance(&to).to_light_years(),
        ));
    }
    Ok(Json(ExitReturn {
        version: 1,
        data: result,
    }))
}

// ====================================================================
// launch

#[launch]
fn rocket() -> _ {
    let universe = data::Universe::load().unwrap();

    rocket::build()
        .manage(universe)
        .mount("/", rocket::fs::FileServer::from("./dist").rank(1))
        .mount("/api", routes![get_stars, calc_dist, calc_path, calc_exit])
        .mount("/", routes![index])
}
