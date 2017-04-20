use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Unit, UnitType};

#[derive(Debug, Default)]
pub struct Gram;

impl Unit for Gram {
    fn classification(&self) -> Classification { Classification::SI }
    fn dim(&self) -> Dimension { Dimension::Mass }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["gram".to_string()] }
    fn primary_code(&self) -> String { "g".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("g".to_string()) }
    fn property(&self) -> Property { Property::Mass }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "G".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Base }
}