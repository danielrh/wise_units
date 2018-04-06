use atom::Atom;
use prefix::Prefix;

pub struct Annotatable {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: i32,
}
