//! This module handles reading .toml files into a `RustAtomList` struct, where
//! the struct is what's used for generating the code for which this crate
//! exists.
//!
//! There are two sources of atom/unit data for which code can be generated:
//!
//! * `Atoms.toml`, defined within the `wise_units` project
//! * `CustomAtoms.toml`, defined within third-party crates who wish to
//!   defined their own units (in addition to those in `Atoms.toml`).
//!
//! In both cases, this module:
//!
//! * Reads the file
//! * Deserializes the TOML into `wise_units-atom_generator` `toml_structs`
//!   structs * Transforms the `toml_structs` into
//!   `wise_units-atom_generator` `rust_structs` structs * Transforms those
//!   into a `RustAtomList` struct.
//!
//! NOTE: Code generated for "special" units can *not* be automatically deduced
//! from the UCUM.xml file; thus, any new special units that get added to the
//! UCUM spec must be manually updated in the relevant functions below, and
//! support for custom special units does not yet exist.
//!

use heck::CamelCase;
use rust_structs::{RustAtom, RustFunctionSet};
use toml_structs::{TomlAtom, TomlBaseUnit, TomlUnit};

pub(crate) mod atoms;
pub(crate) mod custom_atoms;

/// Transforms a `Vec<TomlBaseUnit>` to a `Vec<RustAtom>`.
///
fn transform_base_units(atom_list_base_units: &[TomlBaseUnit]) -> Vec<RustAtom> {
    atom_list_base_units
        .iter()
        .map(|bu| RustAtom {
            type_name: bu.type_name(),
            classification: "Si".to_string(),
            dim: Some(bu.dim.clone()),
            definition_signature: "Ok(Definition::default())".to_string(),
            primary_code: bu.primary_code.clone(),
            print_symbol: Some(bu.print_symbol.clone()),
            property: bu.property.clone(),
            names: bu.names.clone(),
            secondary_code: Some(bu.secondary_code.clone()),
            is_arbitrary: false,
            is_metric: true,
            is_special: false,
        })
        .collect()
}

/// Transforms a `Vec<TomlUnit>` to a `Vec<RustAtom>`.
///
fn transform_units(atom_list_units: &[TomlUnit]) -> Vec<RustAtom> {
    atom_list_units
        .iter()
        .map(|u| {
            let definition_signature = if u.is_special {
                let function_set = RustFunctionSet {
                    convert_from: build_magnitude_function(&u.primary_code),
                    convert_to: build_scalar_function(&u.primary_code),
                };

                let function = u.definition.function.clone().unwrap();
                let function_set_string = build_function_set_string(&function_set);

                format!(
                    "Definition::new({:?}, \"{}\", Some({}))",
                    function.value,
                    function.unit.clone(),
                    function_set_string
                )
            } else if &u.primary_code == "[pi]" {
                format!(
                    "Definition::new(::std::f64::consts::PI, \"{}\", None)",
                    u.definition.unit.clone()
                )
            } else if u.definition.value.eq(&1.0_f64) && &u.definition.unit == "1" {
                "Ok(Definition::default())".to_string()
            } else {
                format!(
                    "Definition::new({:?}, \"{}\", None)",
                    u.definition.value,
                    u.definition.unit.clone()
                )
            };

            RustAtom {
                type_name: u.type_name(),
                classification: u.classification.clone().to_camel_case(),
                dim: None,
                definition_signature,
                primary_code: u.primary_code.clone(),
                print_symbol: u.print_symbol.clone(),
                property: u.property.clone(),
                names: u.names.clone(),
                secondary_code: u.secondary_code.clone(),
                is_arbitrary: u.is_arbitrary,
                is_metric: u.is_metric,
                is_special: u.is_special,
            }
        })
        .collect()
}

/// Determines which function to generate for converting *from* the unit with
/// `primary_code` to its base unit.
///
fn build_scalar_function(primary_code: &str) -> String {
    match primary_code {
        "B"               => "|value: BigRational| BR_10.pow(value)",
        "B[SPL]"          => "|value: BigRational| BR_10.pow(value / 2)",
        "B[V]"            => "|value: BigRational| BR_10.pow(value / 2)",
        "B[mV]"           => "|value: BigRational| BR_10.pow(value / 2)",
        "B[uV]"           => "|value: BigRational| BR_10.pow(value / 2)",
        "B[10.nV]"        => "|value: BigRational| BR_10.pow(value / 2)",
        "B[W]"            => "|value: BigRational| BR_10.pow(value)",
        "B[kW]"           => "|value: BigRational| BR_10.pow(value)",
        "bit_s"           => "|value: BigRational| BR_2.pow(value)",
        "Cel"             => "|value: BigRational| value + &*BR_273_15",
        "Np"              => "|value: BigRational| BR_E.pow(value)",
        "%[slope]"        => "|value: BigRational| value.tan() * 100",
        "[hp'_X]"         => "|value: BigRational| BR_10.pow(-value)",
        "[hp'_C]"         => "|value: BigRational| BR_100.pow(-value)",
        "[hp'_M]"         => "|value: BigRational| BR_1000.pow(-value)",
        "[hp'_Q]"         => "|value: BigRational| BR_50000.pow(-value)",
        "[m/s2/Hz^(1/2)]" => "|value: BigRational| value.pow(2i32)",
        "[pH]"            => "|value: BigRational| -value.log10()",
        "[degF]"          => "|value: BigRational| 5 / 9 * value + &*BR_459_67",
        "[degRe]"         => "|value: BigRational| (value / &*BR_0_8) + &*BR_273_15",
        "[p'diop]"        => "|value: BigRational| value.tan() * 100",
        _                 => panic!("Unknown primary code on special unit: {}", primary_code),
    }.to_string()
}

/// Determines which function to generate for converting *to* the unit with
/// `primary_code` from its base unit. These are only for "special" units and
/// as far as I can tell can *not* be automatically deduced from the UCUM.xml
/// file; thus any new special units that get added to the UCUM spec must be
/// manually updated here, and support for custom special units does not yet
/// exist.
///
fn build_magnitude_function(primary_code: &str) -> String {
    match primary_code {
        "B"               => "|value: BigRational| value.log10()",
        "B[SPL]"          => "|value: BigRational| 2 * value.log10()",
        "B[V]"            => "|value: BigRational| 2 * value.log10()",
        "B[mV]"           => "|value: BigRational| 2 * value.log10()",
        "B[uV]"           => "|value: BigRational| 2 * value.log10()",
        "B[10.nV]"        => "|value: BigRational| 2 * value.log10()",
        "B[W]"            => "|value: BigRational| value.log10()",
        "B[kW]"           => "|value: BigRational| value.log10()",
        "bit_s"           => "|value: BigRational| value.log2()",
        "Cel"             => "|value: BigRational| value - &*BR_273_15",
        "Np"              => "|value: BigRational| value.ln()",
        "%[slope]"        => "|value: BigRational| (value / 100usize).atan()",
        "[hp'_X]"         => "|value: BigRational| -value.log10()",
        "[hp'_C]"         => "|value: BigRational| -value.ln() / BR_100.ln()",
        "[hp'_M]"         => "|value: BigRational| -value.ln() / BR_1000.ln()",
        "[hp'_Q]"         => "|value: BigRational| -value.ln() / &(&*BR_50000).ln()",
        "[m/s2/Hz^(1/2)]" => "|value: BigRational| value.sqrt()",
        "[pH]"            => "|value: BigRational| BR_10.pow(-value)",
        "[degF]"          => "|value: BigRational| 9 * value / 5 - &*BR_459_67",
        "[degRe]"         => "|value: BigRational| (value - &*BR_273_15) * &*BR_0_8",
        "[p'diop]"        => "|value: BigRational| (value / 100usize).atan()",
        _                 => panic!("Unknown primary code on special unit: {}", primary_code),
    }.to_string()
}

fn build_function_set_string(rust_function_set: &RustFunctionSet) -> String {
    format!(
        "FunctionSet {{ convert_from: {}, convert_to: {} }}",
        rust_function_set.convert_from, rust_function_set.convert_to
    )
}
