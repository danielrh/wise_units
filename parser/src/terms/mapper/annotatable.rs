use atom::Atom;
use prefix::Prefix;

pub(super) struct Annotatable {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: Option<i32>,
}
