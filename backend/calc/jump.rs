use uom::si::f64::*;
use uom::si::length::light_year;
use uom::si::mass::kilogram;

pub fn calc_jump(mass: Mass, fuel: f64, efficiency: f64) -> Length {
    Length::new::<light_year>((fuel / mass.get::<kilogram>()) * efficiency * 1e7)
}

#[cfg(test)]
mod tests {
    use uom::si::length::light_year;

    use super::*;

    // FIXME: not verified
    #[test]
    fn test_jump() {
        assert_eq!(
            calc_jump(Mass::new::<kilogram>(28000000.0), 112.0, 0.5),
            Length::new::<light_year>(20.0)
        );
    }
}
