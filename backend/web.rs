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
use uom::si::f64::Length;

use eftb::data;
use uom::si::length::light_year;

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
    star_map: HashMap<u64, data::Star>,
    star_id_to_name: HashMap<u64, String>,
    star_name_to_id: HashMap<String, u64>,
}
impl Db {
    fn get_star(&self, name: String) -> Result<&data::Star, CustomError> {
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

#[get("/jump?<mass>&<fuel>&<efficiency>")]
fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Json<f64> {
    let dist: Length = eftb::calc_jump(mass, fuel, efficiency);
    Json(dist.get::<uom::si::length::light_year>())
}

#[get("/dist?<start>&<end>")]
fn calc_dist(db: &State<Db>, start: String, end: String) -> Result<Json<f64>, CustomError> {
    let start = db.get_star(start)?;
    let end = db.get_star(end)?;
    let dist: Length = start.distance(end);
    Ok(Json(dist.get::<uom::si::length::light_year>()))
}

#[get("/path?<start>&<end>&<jump>&<optimize>")]
fn calc_path(
    db: &State<Db>,
    start: String,
    end: String,
    jump: f64,
    optimize: String,
) -> Result<Json<Vec<(String, String, String, f64)>>, CustomError> {
    let start = db.get_star(start)?;
    let end = db.get_star(end)?;
    let optimize = match optimize.as_str() {
        "fuel" => eftb::PathOptimize::Fuel,
        "distance" => eftb::PathOptimize::Distance,
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
    )
    .ok_or(CustomError(Status::NotFound, format!("No path found")))?;

    let mut result: Vec<(String, String, String, f64)> = Vec::new();
    let mut last_id = start.id;
    for conn in path {
        result.push((
            db.star_id_to_name[&last_id].clone(),
            db.star_id_to_name[&conn.target].clone(),
            match conn.conn_type {
                data::ConnType::Jump => "jump".to_string(),
                data::ConnType::NpcGate => "npc_gate".to_string(),
                data::ConnType::SmartGate => "smart_gate".to_string(),
            },
            conn.distance.get::<light_year>() as f64,
        ));
        last_id = conn.target;
    }
    Ok(Json(result))
}

#[get("/fuel?<dist>&<mass>&<efficiency>")]
fn calc_fuel(dist: f64, mass: f64, efficiency: f64) -> Json<f64> {
    Json(eftb::calc_fuel(
        Length::new::<uom::si::length::light_year>(dist),
        mass,
        efficiency,
    ))
}

#[get("/exit?<start>&<jump>")]
fn calc_exit(
    db: &State<Db>,
    start: String,
    jump: f64,
) -> Result<Json<Vec<(String, String, f64)>>, CustomError> {
    let start = db.get_star(start)?;

    let exits = eftb::calc_exits(
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
    Ok(Json(result))
}

#[launch]
fn rocket() -> _ {
    let star_map: HashMap<u64, data::Star> =
        bincode::deserialize(&std::fs::read("data/starmap.bin").unwrap()).unwrap();
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
            routes![calc_jump, calc_dist, calc_path, calc_fuel, calc_exit],
        )
        .mount("/", routes![index])
}
