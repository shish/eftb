use uom::si::f64::*;
use uom::si::length::light_year;

pub fn calc_fuel(dist: Length, mass: f64, efficiency: f64) -> f64 {
    return dist.get::<light_year>() / (efficiency * 1e7) * mass;
}
