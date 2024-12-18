#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::serde::{json::Json, Serialize};
use rocket::State;
use uom::si::f64::Length;

mod calcs;
mod data;
mod raw;

struct Db {
    star_map: HashMap<u64, data::Star>,
    star_id_to_name: HashMap<u64, String>,
    star_name_to_id: HashMap<String, u64>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/jump?<mass>&<fuel>&<efficiency>")]
fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Json<f64> {
    let dist: Length = calcs::calc_jump(mass, fuel, efficiency);
    Json(dist.get::<uom::si::length::light_year>())
}

#[get("/dist?<start>&<end>")]
fn calc_dist(db: &State<Db>, start: String, end: String) -> Result<Json<f64>, String> {
    let start_id = db
        .star_name_to_id
        .get(&start)
        .ok_or("Start system not found")?;
    let start = db
        .star_map
        .get(start_id)
        .ok_or("Start system ID not found")?;
    let end_id = db.star_name_to_id.get(&end).ok_or("End system not found")?;
    let end = db.star_map.get(end_id).ok_or("End system ID not found")?;

    let dist: Length = start.distance(end);
    Ok(Json(dist.get::<uom::si::length::light_year>()))
}

#[get("/path?<start>&<end>&<jump>")]
fn calc_path(
    db: &State<Db>,
    start: String,
    end: String,
    jump: f64,
) -> Result<Json<Vec<(String, f64)>>, String> {
    let start_id = db
        .star_name_to_id
        .get(&start)
        .ok_or("Start system not found")?;
    let start = db
        .star_map
        .get(start_id)
        .ok_or("Start system ID not found")?;
    let end_id = db.star_name_to_id.get(&end).ok_or("End system not found")?;
    let end = db.star_map.get(end_id).ok_or("End system ID not found")?;

    let path = calcs::calc_path(
        &db.star_map,
        start,
        end,
        Length::new::<uom::si::length::light_year>(jump),
    )
    .ok_or("No path found")?;

    let mut result: Vec<(String, f64)> = Vec::new();
    let mut last_star = start.clone();
    for star in path {
        result.push((
            db.star_id_to_name[&star.id].clone(),
            last_star
                .distance(&star)
                .get::<uom::si::length::light_year>(),
        ));
        last_star = star;
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
        .mount("/", routes![index])
        .mount("/calc", routes![calc_jump, calc_dist, calc_path])
        .mount("/assets", rocket::fs::FileServer::from("./assets"))
}
