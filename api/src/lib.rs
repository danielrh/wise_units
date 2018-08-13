#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(feature = "with_serde")]
extern crate serde;

#[cfg(feature = "with_serde")]
#[cfg_attr(feature = "with_serde", macro_use)]
extern crate serde_derive;

#[cfg(all(test, feature = "with_serde"))]
extern crate serde_json;

#[cfg_attr(test, macro_use)]
extern crate wise_units_parser;

pub mod convertible;
pub mod decomposer;
pub mod field_eq;
pub mod measurement;
pub mod unit;

pub use convertible::Convertible;
pub use field_eq::FieldEq;
pub use measurement::Measurement;
pub use unit::Unit;

pub use wise_units_parser::{
    Atom, Classification, Composable, Composition, Dimension, Error, Prefix, Property, Term,
    UcumSymbol, UcumUnit,
};
