use super::rust_atom::RustAtom;

#[derive(Debug, Serialize)]
pub(crate) struct RustAtomList {
    pub atoms: Vec<RustAtom>,
}
