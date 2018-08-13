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

use heck::{CamelCase, ShoutySnakeCase};
use rust_structs::{RustAtom, RustFunctionSet};
use toml_structs::{TomlAtom, TomlBaseUnit, TomlUnit};

pub(crate) mod atoms;
pub(crate) mod custom_atoms;

/// Transforms a `Vec<TomlBaseUnit>` to a `Vec<RustAtom>`.
///
fn transform_base_units(atom_list_base_units: &[TomlBaseUnit]) -> Vec<RustAtom> {
    atom_list_base_units
        .iter()
        .map(|toml_base_unit| RustAtom {
            type_name: toml_base_unit.type_name(),
            classification: "Si".to_string(),
            dim: Some(toml_base_unit.dim.clone()),
            definition_constant: toml_base_unit.type_name().to_shouty_snake_case(),
            definition_signature: "Definition { value: 1.0, terms: vec![Term::new(None, None)], function_set: None }".to_string(),
            primary_code: toml_base_unit.primary_code.clone(),
            print_symbol: Some(toml_base_unit.print_symbol.clone()),
            property: toml_base_unit.property.clone(),
            names: toml_base_unit.names.clone(),
            secondary_code: Some(toml_base_unit.secondary_code.clone()),
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
        .map(|toml_unit| {
            let definition_signature = if toml_unit.is_special {
                build_definition_special(&toml_unit)
            } else if &toml_unit.primary_code == "[pi]" {
                format!(
                    "Definition::new(::std::f64::consts::PI, \"{}\", None).{}",
                    toml_unit.definition.unit.clone(),
                    build_definition_expect(),
                )
            } else if toml_unit.definition.value.eq(&1.0_f64) && &toml_unit.definition.unit == "1" {
                "Definition::default()".to_string()
            } else {
                format!(
                    "Definition::new({:?}, \"{}\", None).{}",
                    toml_unit.definition.value,
                    toml_unit.definition.unit.clone(),
                    build_definition_expect(),
                )
            };

            RustAtom {
                type_name: toml_unit.type_name(),
                classification: toml_unit.classification.clone().to_camel_case(),
                dim: None,
                definition_constant: toml_unit.type_name().to_shouty_snake_case(),
                definition_signature,
                primary_code: toml_unit.primary_code.clone(),
                print_symbol: toml_unit.print_symbol.clone(),
                property: toml_unit.property.clone(),
                names: toml_unit.names.clone(),
                secondary_code: toml_unit.secondary_code.clone(),
                is_arbitrary: toml_unit.is_arbitrary,
                is_metric: toml_unit.is_metric,
                is_special: toml_unit.is_special,
            }
        })
        .collect()
}

fn build_definition_special(toml_unit: &TomlUnit) -> String {
    let function_set = RustFunctionSet {
        convert_from: build_magnitude_function(&toml_unit.primary_code),
        convert_to: build_scalar_function(&toml_unit.primary_code),
    };

    let function = toml_unit.definition.function.clone().unwrap();
    let function_set_string = build_function_set_string(&function_set);

    format!(
        "Definition::new({:?}, \"{}\", Some({})).{}",
        function.value,
        function.unit.clone(),
        function_set_string,
        build_definition_expect(),
    )
}

fn build_definition_expect() -> &'static str {
    "expect(\"BUG! Bad Atom -> Definition mapping!\")"
}

/// Determines which function to generate for converting *from* the unit with
/// `primary_code` to its base unit.
///
fn build_scalar_function(primary_code: &str) -> String {
    match primary_code {
        "B"               => "|value: f64| 10_f64.powf(value)",
        "B[SPL]"          => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[V]"            => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[mV]"           => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[uV]"           => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[10.nV]"        => "|value: f64| 10_f64.powf(value / 2.0)",
        "B[W]"            => "|value: f64| 10_f64.powf(value)",
        "B[kW]"           => "|value: f64| 10_f64.powf(value)",
        "bit_s"           => "|value: f64| 2_f64.powf(value)",
        "Cel"             => "|value: f64| value + 273.15",
        "Np"              => "|value: f64| ::std::f64::consts::E.powf(value)",
        "%[slope]"        => "|value: f64| value.tan() * 100.0",
        "[hp'_X]"         => "|value: f64| 10_f64.powf(-value)",
        "[hp'_C]"         => "|value: f64| 100_f64.powf(-value)",
        "[hp'_M]"         => "|value: f64| 1_000_f64.powf(-value)",
        "[hp'_Q]"         => "|value: f64| 50_000_f64.powf(-value)",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value.powi(2)",
        "[pH]"            => "|value: f64| -value.log10()",
        "[degF]"          => "|value: f64| 5.0 / 9.0 * (value + 459.67)",
        "[degRe]"         => "|value: f64| (value / 0.8) + 273.15",
        "[p'diop]"        => "|value: f64| value.tan() * 100.0",
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
        "B"               => "|value: f64| value.log10()",
        "B[SPL]"          => "|value: f64| 2.0 * value.log10()",
        "B[V]"            => "|value: f64| 2.0 * value.log10()",
        "B[mV]"           => "|value: f64| 2.0 * value.log10()",
        "B[uV]"           => "|value: f64| 2.0 * value.log10()",
        "B[10.nV]"        => "|value: f64| 2.0 * value.log10()",
        "B[W]"            => "|value: f64| value.log10()",
        "B[kW]"           => "|value: f64| value.log10()",
        "bit_s"           => "|value: f64| value.log2()",
        "Cel"             => "|value: f64| value - 273.15",
        "Np"              => "|value: f64| value.ln()",
        "%[slope]"        => "|value: f64| (value / 100.0).atan()",
        "[hp'_X]"         => "|value: f64| -value.log10()",
        "[hp'_C]"         => "|value: f64| -value.ln() / 100_f64.ln()",
        "[hp'_M]"         => "|value: f64| -value.ln() / 1_000_f64.ln()",
        "[hp'_Q]"         => "|value: f64| -value.ln() / 50_000_f64.ln()",
        "[m/s2/Hz^(1/2)]" => "|value: f64| value.sqrt()",
        "[pH]"            => "|value: f64| 10.0_f64.powf(-value)",
        "[degF]"          => "|value: f64| 9.0 * value / 5.0 - 459.67",
        "[degRe]"         => "|value: f64| (value - 273.15) * 0.8",
        "[p'diop]"        => "|value: f64| (value / 100.0).atan()",
        _                 => panic!("Unknown primary code on special unit: {}", primary_code),
    }.to_string()
}

fn build_function_set_string(rust_function_set: &RustFunctionSet) -> String {
    format!(
        "FunctionSet {{ convert_from: {}, convert_to: {} }}",
        rust_function_set.convert_from, rust_function_set.convert_to
    )
}
