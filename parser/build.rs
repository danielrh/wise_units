extern crate wise_units_atom_generator;

use std::env;
use std::path::Path;

fn main() {
    let src_dir = env::current_dir().unwrap();
    let src_path = Path::new(&src_dir).join("CustomAtoms.toml");

    ::wise_units_atom_generator::build_with_custom_atoms(src_path);
    // ::wise_units_atom_generator::build_with_custom_atoms();
}
