#![recursion_limit="1024"]

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate failure;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate lazy_static;

// Only include macros for testing
#[cfg(test)]
#[macro_use(consumes_to, fails_with, parses_to)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

// include!(concat!(env!("OUT_DIR"), "/classification.rs"));
// include!(concat!(env!("OUT_DIR"), "/property.rs"));
// include!(concat!(env!("OUT_DIR"), "/atom.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/classification.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/property.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/atom.rs"));

#[macro_use]
mod macros;

pub(self) mod symbols;

#[cfg(test)]
mod atom_test;
mod composable;
mod composition;
mod definition;
mod dimension;
mod error;
mod function_set;
mod prefix;
mod reducible;
mod term;
mod terms;
mod ucum_unit;
mod ucum_symbol;

pub use self::atom::Atom;
pub use self::classification::Classification;
pub use self::composable::Composable;
pub use self::composition::Composition;
pub use self::dimension::Dimension;
pub use self::error::Error;
pub use self::prefix::Prefix;
pub use self::property::Property;
pub use self::reducible::Reducible;
pub use self::symbols::symbol_parser::SymbolParser;
pub use self::term::Term;
pub use self::ucum_symbol::UcumSymbol;
pub use self::ucum_unit::UcumUnit;

use self::terms::term_parser::Rule;
use self::terms::term_parser::TermParser;
use pest::Parser;

pub fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(Rule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}
