use super::{Error, Term};
use function_set::FunctionSet;
use reducible::Reducible;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
#[derive(Debug)]
pub(crate) struct Definition {
    pub(crate) value: f64,
    // TODO: Change to Option<Vec<Term>>
    pub(crate) terms: Vec<Term>,

    /// Conversion functions only required for special (non-ratio based) atoms.
    pub(crate) function_set: Option<FunctionSet>,
}

impl Definition {
    pub(crate) fn new(
        value: f64,
        expression: &str,
        function_set: Option<FunctionSet>,
    ) -> Result<Self, Error> {
        let terms = super::parse(expression)?;

        Ok(Self {
            value,
            terms,
            function_set,
        })
    }

    pub(crate) fn new_only_value(value: f64) -> Self {
        Self {
            value,
            terms: vec![],
            function_set: None,
        }
    }

    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}

impl Reducible for Definition {
    fn reduce_value(&self, other_value: f64) -> f64 {
        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value
                } else {
                    self.value * self.terms.reduce_value(other_value)
                }
            }
            Some(ref f) => (f.convert_to)(other_value),
        }
    }

    fn calculate_magnitude(&self, other_value: f64) -> f64 {
        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value
                } else {
                    self.value * self.terms.calculate_magnitude(other_value)
                }
            }
            Some(ref f) => (f.convert_from)(other_value),
        }
    }
}

impl Default for Definition {
    fn default() -> Self {
        Self {
            value: 1.0,
            terms: vec![term!()],
            function_set: None,
        }
    }
}
