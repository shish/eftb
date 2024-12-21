use uom::si::f64::*;
use uom::si::length::light_year;

pub fn calc_fuel(dist: Length, mass: f64, efficiency: f64) -> f64 {
    return dist.get::<light_year>() / (efficiency * 1e7) * mass;
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: not verified
    #[test]
    fn test_fuel() {
        assert_eq!(
            calc_fuel(Length::new::<light_year>(20.0), 28000000.0, 0.5),
            112.0
        );
    }
}
