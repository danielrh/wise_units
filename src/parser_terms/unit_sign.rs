use std::fmt;

#[derive(PartialEq)]
pub enum UnitSign {
    Positive,
    Negative
}

impl fmt::Display for UnitSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnitSign::Positive => write!(f, ""),
            UnitSign::Negative => write!(f, "-"),
        }
    }
}
