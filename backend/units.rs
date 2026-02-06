use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

/// Distance in meters
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    SerdeSerialize,
    SerdeDeserialize,
    Archive,
    Deserialize,
    Serialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Meters(pub f64);

// Conversion constant: 1 light year = 9.4607304725808e15 meters
const METERS_PER_LIGHT_YEAR: f64 = 9.4607304725808e15;

impl Meters {
    pub fn new(value: f64) -> Self {
        Meters(value)
    }

    pub fn from_light_years(light_years: f64) -> Self {
        Meters(light_years * METERS_PER_LIGHT_YEAR)
    }

    pub fn to_light_years(self) -> f64 {
        self.0 / METERS_PER_LIGHT_YEAR
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

// Allow comparing Meters values
impl Eq for Meters {}

impl Ord for Meters {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_to_light_years() {
        let meters = Meters::new(METERS_PER_LIGHT_YEAR);
        let ly = meters.to_light_years();
        assert!((ly - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_from_light_years() {
        let meters = Meters::from_light_years(1.0);
        assert!((meters.get() - METERS_PER_LIGHT_YEAR).abs() < 1e-6);
    }

    #[test]
    fn test_round_trip() {
        let original_meters = Meters::new(1.234e16);
        let ly = original_meters.to_light_years();
        let back_to_meters = Meters::from_light_years(ly);
        assert!((original_meters.get() - back_to_meters.get()).abs() < 1.0);
    }

    #[test]
    fn test_comparison() {
        let a = Meters::new(100.0);
        let b = Meters::new(200.0);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    }
}
