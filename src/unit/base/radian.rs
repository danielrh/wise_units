use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Unit, UnitType};

#[derive(Debug, Default)]
pub struct Radian;

impl Unit for Radian {
    fn classification(&self) -> Classification { Classification::SI }
    fn dim(&self) -> Dimension { Dimension::PlaneAngle }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["radian".to_string()] }
    fn primary_code(&self) -> String { "rad".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("rad".to_string()) }
    fn property(&self) -> Property { Property::PlaneAngle }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "RAD".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Base }
}
