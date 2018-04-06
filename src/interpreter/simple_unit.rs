use atom::Atom;
use prefix::Prefix;

pub struct SimpleUnit {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}
