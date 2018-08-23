use num_help::BR_0;
use num_rational::BigRational;
use num_traits::identities::One;
use parser::{function_set::FunctionSet, Error, Term};
use reducible::Reducible;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
#[derive(Debug)]
pub(crate) struct Definition {
    value: BigRational,
    terms: Vec<Term>,

    /// Conversion functions only required for special (non-ratio based) atoms.
    function_set: Option<FunctionSet>,
}

impl Definition {
    pub(crate) fn new(
        // TODO: change to BigRational
        value: f64,
        expression: &str,
        function_set: Option<FunctionSet>,
    ) -> Result<Self, Error> {
        let value: BigRational = BigRational::from_float(value).unwrap_or(BR_0.clone());
        let terms = super::parse(expression)?;

        Ok(Self {
            value,
            terms,
            function_set,
        })
    }

    pub fn value<'a>(&'a self) -> &'a BigRational {
        &self.value
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}

impl<'a> Reducible<&'a BigRational> for Definition {
    fn reduce_value(&self, other_value: &'a BigRational) -> BigRational {
        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value.clone()
                } else {
                    &self.value * self.terms.reduce_value(&other_value)
                }
            }
            Some(ref f) => (f.convert_to)(other_value.clone()),
        }
    }

    fn calculate_magnitude(&self, other_value: &'a BigRational) -> BigRational {
        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value.clone()
                } else {
                    &self.value * self.terms.calculate_magnitude(&other_value)
                }
            }
            Some(ref f) => (f.convert_from)(other_value.clone()),
        }
    }
}

impl Default for Definition {
    fn default() -> Self {
        Self {
            value: One::one(),
            terms: vec![term!()],
            function_set: None,
        }
    }
}
