mod annotation_composable;
mod composable;
mod display;
mod field_eq;
mod invert;
mod is_compatible_with;
mod partial_eq;
mod reducible;
mod ucum_unit;

use crate::parser::{Atom, Prefix};

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
#[derive(Clone, Debug, Eq, Default)]
pub struct Term {
    pub factor: Option<u32>,
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
    pub exponent: Option<i32>,
    pub annotation: Option<String>,
}

impl Term {
    #[must_use]
    pub const fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
        Self {
            atom,
            prefix,
            factor: None,
            exponent: None,
            annotation: None,
        }
    }

    /// Creates a new `Term` that's equivalent to the unit "1".
    ///
    #[must_use]
    pub const fn new_unity() -> Self {
        Self {
            atom: None,
            prefix: None,
            factor: Some(1),
            exponent: None,
            annotation: None,
        }
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
        self.factor == Some(1_u32)
            && self.exponent.is_none()
            && self.atom.is_none()
            && self.prefix.is_none()
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
        self.factor.is_some() || self.atom.is_some() || self.annotation.is_some()
    }

    /// If `self` has an `exponent`, it checks if its value is positive; if not, it returns `true`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_positive(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_positive(),
            // None is analogous to an exponent of 1.
            None => true,
        }
    }

    /// If `self` has an `exponent`, it checks if its value is negative; if not, it returns `false`
    /// (since `None` is analogous to an exponent of 1).
    ///
    #[must_use]
    pub const fn exponent_is_negative(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_negative(),
            // None is analogous to an exponent of 1.
            None => false,
        }
    }

    pub fn factor_and_is_not_one<F: FnOnce(u32)>(&self, f: F) {
        if let Some(factor) = self.factor {
            if factor != 1 {
                f(factor);
            }
        }
    }

    #[must_use]
    pub fn factor_as_u32(&self) -> u32 {
        self.factor.unwrap_or(1)
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
