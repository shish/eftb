use uom::si::f64::*;
use uom::si::length::light_year;

pub fn calc_jump(mass: f64, fuel: f64, efficiency: f64) -> Length {
    Length::new::<light_year>((fuel / mass) * efficiency * 1e7)
}

#[cfg(test)]
mod tests {
    use uom::si::length::meter;

    use super::*;

    #[test]
    fn test_jump() {
        assert_eq!(
            calc_jump(28000000.0, 500.0, 0.5),
            Length::new::<meter>(8.447080357142858e17)
        );
    }
}
