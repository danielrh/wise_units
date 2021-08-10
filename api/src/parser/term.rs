mod annotation_composable;
mod composable;
mod display;
mod invert;
mod is_compatible_with;
mod reducible;
mod ucum_unit;

use crate::parser::{Atom, Prefix};

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Term {
//     pub factor: Option<u32>,
//     pub prefix: Option<Prefix>,
//     pub atom: Option<Atom>,
//     pub exponent: Option<i32>,
//     pub annotation: Option<String>,
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    FPAEA {
        factor: u32,
        prefix: Prefix,
        atom: Atom,
        exponent: i32,
        annotation: String,
    },
    FPAE {
        factor: u32,
        prefix: Prefix,
        atom: Atom,
        exponent: i32,
    },
    FPAA {
        factor: u32,
        prefix: Prefix,
        atom: Atom,
        annotation: String,
    },
    FPA {
        factor: u32,
        prefix: Prefix,
        atom: Atom,
    },
    FAEA {
        factor: u32,
        atom: Atom,
        exponent: i32,
        annotation: String,
    },
    FAE {
        factor: u32,
        atom: Atom,
        exponent: i32,
    },
    FAA {
        factor: u32,
        atom: Atom,
        annotation: String,
    },
    FA {
        factor: u32,
        atom: Atom,
    },
    FE {
        factor: u32,
        exponent: i32,
    },
    Factor(u32),
    PAEA {
        prefix: Prefix,
        atom: Atom,
        exponent: i32,
        annotation: String,
    },
    PAE {
        prefix: Prefix,
        atom: Atom,
        exponent: i32,
    },
    PAA {
        prefix: Prefix,
        atom: Atom,
        annotation: String,
    },
    PA {
        prefix: Prefix,
        atom: Atom,
    },
    AEA {
        atom: Atom,
        exponent: i32,
        annotation: String,
    },
    AE {
        atom: Atom,
        exponent: i32,
    },
    AA {
        atom: Atom,
        annotation: String,
    },
    Atom(Atom),
    Annotation(String),
}

impl Term {
    /// Creates a new `Term` that's equivalent to the unit "1".
    ///
    #[must_use]
    pub const fn new_unity() -> Self {
        Self::Factor(1)
    }

    /// A `Term` is a unity `Term` if represents "1", which technically means
    /// here:
    ///
    /// * its `factor` is 1
    /// * it has no `exponent`
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    #[must_use]
    pub fn is_unity(&self) -> bool {
        matches!(self, Self::Factor(1))
    }

    pub const fn factor(&self) -> Option<u32> {
        match self {
            Self::FPAEA { factor, .. }
            | Self::FPAE { factor, .. }
            | Self::FPAA { factor, .. }
            | Self::FAEA { factor, .. }
            | Self::FPA { factor, .. }
            | Self::FAE { factor, .. }
            | Self::FAA { factor, .. }
            | Self::FA { factor, .. }
            | Self::FE { factor, .. }
            | Self::Factor(factor) => Some(*factor),
            Self::PAEA { .. }
            | Self::PAE { .. }
            | Self::PAA { .. }
            | Self::PA { .. }
            | Self::AEA { .. }
            | Self::AE { .. }
            | Self::AA { .. }
            | Self::Atom(_)
            | Self::Annotation(_) => None,
        }
    }

    pub const fn prefix(&self) -> Option<Prefix> {
        match self {
            Self::FPAEA { prefix, .. }
            | Self::FPAE { prefix, .. }
            | Self::FPAA { prefix, .. }
            | Self::FPA { prefix, .. }
            | Self::PAEA { prefix, .. }
            | Self::PAA { prefix, .. }
            | Self::PAE { prefix, .. }
            | Self::PA { prefix, .. } => Some(*prefix),
            Self::FAEA { .. }
            | Self::FAE { .. }
            | Self::FAA { .. }
            | Self::FA { .. }
            | Self::FE { .. }
            | Self::Factor(_)
            | Self::AEA { .. }
            | Self::AE { .. }
            | Self::AA { .. }
            | Self::Atom(_)
            | Self::Annotation(_) => None,
        }
    }

    pub const fn atom(&self) -> Option<Atom> {
        match self {
            Self::FPAEA { atom, .. }
            | Self::FAEA { atom, .. }
            | Self::FPAE { atom, .. }
            | Self::FPAA { atom, .. }
            | Self::FPA { atom, .. }
            | Self::FAE { atom, .. }
            | Self::FAA { atom, .. }
            | Self::FA { atom, .. }
            | Self::PAEA { atom, .. }
            | Self::PAE { atom, .. }
            | Self::PAA { atom, .. }
            | Self::PA { atom, .. }
            | Self::AEA { atom, .. }
            | Self::AE { atom, .. }
            | Self::AA { atom, .. }
            | Self::Atom(atom) => Some(*atom),
            Self::FE { .. } | Self::Factor(_) | Self::Annotation(_) => None,
        }
    }

    pub const fn exponent(&self) -> Option<i32> {
        match self {
            Self::FPAEA { exponent, .. }
            | Self::FPAE { exponent, .. }
            | Self::FAEA { exponent, .. }
            | Self::FAE { exponent, .. }
            | Self::FE { exponent, .. }
            | Self::PAEA { exponent, .. }
            | Self::PAE { exponent, .. }
            | Self::AEA { exponent, .. }
            | Self::AE { exponent, .. } => Some(*exponent),
            Self::FPAA { .. }
            | Self::FAA { .. }
            | Self::FPA { .. }
            | Self::FA { .. }
            | Self::Factor(_)
            | Self::PAA { .. }
            | Self::PA { .. }
            | Self::AA { .. }
            | Self::Atom(_)
            | Self::Annotation(_) => None,
        }
    }

    pub const fn annotation(&self) -> Option<&String> {
        match self {
            Self::FPAEA { annotation, .. }
            | Self::FAEA { annotation, .. }
            | Self::FAA { annotation, .. }
            | Self::PAEA { annotation, .. }
            | Self::PAA { annotation, .. }
            | Self::AEA { annotation, .. }
            | Self::AA { annotation, .. }
            | Self::Annotation(annotation) => Some(annotation),
            Self::FPAE { .. }
            | Self::FPAA { .. }
            | Self::FPA { .. }
            | Self::FAE { .. }
            | Self::FA { .. }
            | Self::FE { .. }
            | Self::Factor(_)
            | Self::PAE { .. }
            | Self::PA { .. }
            | Self::AE { .. }
            | Self::Atom(_) => None,
        }
    }

    /// A `Term` is valueless if its fields don't contain data that turns the `Term` into something
    /// with value:
    ///
    /// * it has no `factor`
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    /// A `Term` with _only_ an `exponent` doesn't really make any sense. A `Term` with an
    /// `annotation` certainly makes sense, but does not impact the value of anything.
    ///
    #[must_use]
    pub const fn has_value(&self) -> bool {
        self.factor().is_some() || self.atom().is_some() || self.annotation().is_some()
    }

    /// If `self` has an `exponent`, it checks if its value is positive; if not, it returns `true`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_positive(&self) -> bool {
        match self.exponent() {
            Some(e) => e.is_positive(),
            None => true,
        }
    }

    /// If `self` has an `exponent`, it checks if its value is negative; if not, it returns `false`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_negative(&self) -> bool {
        match self.exponent() {
            Some(e) => e.is_negative(),
            None => false,
        }
    }

    pub fn factor_and_is_not_one<F: FnOnce(u32)>(&self, f: F) {
        if let Some(factor) = self.factor() {
            if factor != 1 {
                f(factor)
            }
        }
    }

    #[must_use]
    pub fn factor_as_u32(&self) -> u32 {
        self.factor().unwrap_or(1)
    }
}

impl ::std::default::Default for Term {
    fn default() -> Self {
        Self::Factor(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn validate_new_unity() {
        let term = Term::new_unity();
        assert_eq!(term.to_string(), "1");
    }
}
