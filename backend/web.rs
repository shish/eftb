#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use uom::si::f64::*;
use uom::si::length::light_year;
use uom::si::mass::kilogram;

use eftb::calc::path::PathOptimize;
use eftb::data;
use eftb::data::{ConnType, SolarSystemId, Star};

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

struct Db {
    star_map: HashMap<SolarSystemId, Star>,
    star_id_to_name: HashMap<SolarSystemId, String>,
    star_name_to_id: HashMap<String, SolarSystemId>,
}
impl Db {
    fn get_star(&self, name: String) -> Result<&Star, CustomError> {
        let id = self.star_name_to_id.get(&name).ok_or(CustomError(
            Status::NotFound,
            format!("Solar system {} not found", name),
        ))?;
        let star = self.star_map.get(id).ok_or(CustomError(
            Status::NotFound,
            format!("Solar system {} not found", name),
        ))?;
        Ok(star)
    }
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
fn get_stars(db: &State<Db>) -> Json<StarsReturn> {
    let names = db.star_name_to_id.keys().cloned().collect();
    Json(StarsReturn {
        version: 1,
        data: names,
    })
}

// ====================================================================
// calc_jump

#[derive(Debug, Serialize)]
struct JumpReturn {
    version: u32,
    data: f64,
}

#[get("/jump?<mass>&<fuel>&<efficiency>")]
fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Json<JumpReturn> {
    let dist: Length = eftb::calc_jump(Mass::new::<kilogram>(mass), fuel, efficiency);
    Json(JumpReturn {
        version: 1,
        data: dist.get::<uom::si::length::light_year>(),
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
fn calc_dist(db: &State<Db>, start: String, end: String) -> Result<Json<DistReturn>, CustomError> {
    let start = db.get_star(start)?;
    let end = db.get_star(end)?;
    let dist: Length = start.distance(end);
    Ok(Json(DistReturn {
        version: 1,
        data: dist.get::<uom::si::length::light_year>(),
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
struct PathReturn {
    version: u32,
    data: Vec<PathStep>,
}

#[get("/path?<start>&<end>&<jump>&<optimize>&<use_smart_gates>")]
fn calc_path(
    db: &State<Db>,
    start: String,
    end: String,
    jump: f64,
    optimize: String,
    use_smart_gates: bool,
) -> Result<Json<PathReturn>, CustomError> {
    let start = db.get_star(start)?;
    let end = db.get_star(end)?;
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

    let path = eftb::calc_path(
        &db.star_map,
        start,
        end,
        Length::new::<uom::si::length::light_year>(jump),
        optimize,
        use_smart_gates,
    )
    .ok_or(CustomError(Status::NotFound, format!("No path found")))?;

    let mut result = Vec::new();
    let mut last_id = start.id;
    for conn in path {
        result.push(PathStep {
            from: WebStar {
                id: last_id,
                name: db.star_id_to_name[&last_id].clone(),
            },
            conn_type: match conn.conn_type {
                ConnType::Jump => "jump".to_string(),
                ConnType::NpcGate => "npc_gate".to_string(),
                ConnType::SmartGate => "smart_gate".to_string(),
            },
            distance: conn.distance.get::<light_year>() as f64,
            to: WebStar {
                id: conn.target,
                name: db.star_id_to_name[&conn.target].clone(),
            },
        });
        last_id = conn.target;
    }
    Ok(Json(PathReturn {
        version: 2,
        data: result,
    }))
}

// ====================================================================
// calc_fuel

#[derive(Debug, Serialize)]
struct FuelReturn {
    version: u32,
    data: f64,
}

#[get("/fuel?<dist>&<mass>&<efficiency>")]
fn calc_fuel(dist: f64, mass: f64, efficiency: f64) -> Json<FuelReturn> {
    Json(FuelReturn {
        version: 1,
        data: eftb::calc_fuel(
            Length::new::<uom::si::length::light_year>(dist),
            Mass::new::<kilogram>(mass),
            efficiency,
        ),
    })
}

// ====================================================================
// calc_exit

#[derive(Debug, Serialize)]
struct ExitReturn {
    version: u32,
    data: Vec<(String, String, f64)>,
}

#[get("/exit?<start>&<jump>")]
fn calc_exit(db: &State<Db>, start: String, jump: f64) -> Result<Json<ExitReturn>, CustomError> {
    let start = db.get_star(start)?;

    let exits = eftb::calc_exit(
        &db.star_map,
        start,
        Length::new::<uom::si::length::light_year>(jump),
    );

    let mut result: Vec<(String, String, f64)> = Vec::new();
    for (from, to) in exits {
        result.push((
            db.star_id_to_name[&from.id].clone(),
            db.star_id_to_name[&to.id].clone(),
            from.distance(&to).get::<uom::si::length::light_year>(),
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
    let star_map = data::get_star_map().unwrap();
    let (star_id_to_name, star_name_to_id) = data::get_name_maps().unwrap();

    let db = Db {
        star_map,
        star_id_to_name,
        star_name_to_id,
    };

    rocket::build()
        .manage(db)
        .mount("/", rocket::fs::FileServer::from("./dist").rank(1))
        .mount(
            "/api",
            routes![get_stars, calc_jump, calc_dist, calc_path, calc_fuel, calc_exit],
        )
        .mount("/", routes![index])
}
