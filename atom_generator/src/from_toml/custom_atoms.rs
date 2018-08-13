use rust_structs::RustAtomList;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use toml_structs::TomlCustomAtomList;

// static CUSTOM_ATOMS_FILE_NAME: &'static str = "CustomAtoms.toml";

/// Reads the `Atoms.toml` file (containing UCUM unit definitions) that's part
/// of the repo *and* a `CustomAtoms.toml` file that's part of the calling
/// library and transforms those into a `RustAtomList`.
///
pub(crate) fn build_rust_atom_list(custom_file_path: PathBuf) -> RustAtomList {
    let mut rust_atom_list = super::atoms::build_rust_atom_list();

    if let Some(contents) = read_custom_atoms_toml(custom_file_path) {
        let toml_custom_atoms = TomlCustomAtomList::from_str(&contents)
            .expect("unable to deserialize Toml to TomlCustomAtomList");

        transform(&toml_custom_atoms, &mut rust_atom_list);
    }

    rust_atom_list
}

/// Reads `CustomAtoms.toml` from the consuming crate's root directory.
///
fn read_custom_atoms_toml(custom_file_path: PathBuf) -> Option<String> {
    // let src_dir = env::current_dir().unwrap();
    // debug!("[read_custom_atoms_toml] src_dir: {:?}", &src_dir);

    // let src_path = Path::new(&src_dir).join(CUSTOM_ATOMS_FILE_NAME);
    // debug!("[read_custom_atoms_toml] src_path: {:?}", src_path.to_str());

    File::open(&custom_file_path)
        .ok()
        .and_then(|mut atoms_file| {
            let mut read_contents = String::new();

            atoms_file
                .read_to_string(&mut read_contents)
                .expect("something went wrong reading the file");
            debug!("[read_custom_atoms_toml] Read {}", custom_file_path.display());

            Some(read_contents)
        })
}

/// Transforms a `TomlCustomAtomlList` and appends to a `RustAtomList`.
///
fn transform(toml_atom_list: &TomlCustomAtomList, rust_atom_list: &mut RustAtomList) {
    if let Some(ref bu) = toml_atom_list.base_units {
        rust_atom_list
            .atoms
            .append(&mut super::transform_base_units(bu));
    }

    if let Some(ref u) = toml_atom_list.units {
        rust_atom_list.atoms.append(&mut super::transform_units(u));
    }
}
