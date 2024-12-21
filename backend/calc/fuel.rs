use uom::si::f64::*;
use uom::si::length::light_year;
use uom::si::mass::kilogram;

pub fn calc_fuel(dist: Length, mass: Mass, efficiency: f64) -> f64 {
    let dist = dist.get::<light_year>();
    let mass = mass.get::<kilogram>();
    return dist / (efficiency * 1e7) * mass;
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: not verified
    #[test]
    fn test_fuel() {
        assert_eq!(
            calc_fuel(
                Length::new::<light_year>(20.0),
                Mass::new::<kilogram>(28000000.0),
                0.5
            ),
            112.0
        );
    }
}
