use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Unit, UnitType};

#[derive(Debug, Default)]
pub struct Coulomb;

impl Unit for Coulomb {
    fn classification(&self) -> Classification { Classification::SI }
    fn dim(&self) -> Dimension { Dimension::ElectricCharge }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["Coulomb".to_string()] }
    fn primary_code(&self) -> String { "C".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("C".to_string()) }
    fn property(&self) -> Property { Property::ElectricCharge }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "C".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Base }
}
