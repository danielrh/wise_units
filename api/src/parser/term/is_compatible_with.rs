use crate::is_compatible_with::IsCompatibleWith;
use crate::parser::{annotation_composition::AnnotationComposable, Composable, Term};

/// In order to enforce compatibility on "non-units" (ex. `{each}`, `{total}`, `{heartbeats}`),
/// `Term`s need to compare their annotations along with their `Composition`s. In practice, and
/// taking from [UCUM's Examples for some
/// non-units](http://unitsofmeasure.org/ucum.html#section-Examples-for-some-Non-Units.), if the
/// following are measurements we want to say :
///
/// * "100 grams of creatine" are compatible with "200 grams of creatine"
/// * "100 grams of creatine" are _not_ compatible with "200 grams of wet tissue"
/// * "100 grams" are _not_ compatible with "200 grams of creatine"
///
/// ...or when used without a Unit Atom...
///
/// * "100 {tree}" is compatible with "200 {tree}"
/// * "100 {tree}" is _not_ compatible with "200 {cell}"
/// * "100" is _not_ compatible with "200 {cell}"
///
/// These distinctions are important as unit-compatibility is what determines if/when/how
/// measurements can be:
///
/// * converted to another unit
/// * added to, subtracted from, multiplied, divided
/// * checked for equality
/// * sorted
///
/// ...and thus the annotation plays an important role in that.
///
/// More info at nih.gov, [here](https://ucum.nlm.nih.gov/ucum-service.html) (look under the
/// "Annotations" section for starters).
///
impl IsCompatibleWith for Term {
    fn is_compatible_with(&self, rhs: &Self) -> bool {
        self.composition() == rhs.composition() && self.annotation() == rhs.annotation()
    }
}

impl IsCompatibleWith for Vec<Term> {
    fn is_compatible_with(&self, rhs: &Self) -> bool {
        let lhs_annotation_composition = self.annotation_composition();
        let rhs_annotation_composition = rhs.annotation_composition();

        self.composition() == rhs.composition()
            && rhs_annotation_composition == lhs_annotation_composition
    }
}

#[cfg(test)]
mod tests {
    use crate::is_compatible_with::IsCompatibleWith;
    use crate::parser::{Atom, Prefix, Term};

    mod without_annotations {
        use super::*;

        #[test]
        fn validate_term() {
            let lhs = Term::Atom(Atom::Meter);
            let rhs = Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            };
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_term_with_factor() {
            let lhs = Term::Atom(Atom::Meter);
            let rhs = Term::FPA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                factor: 20,
            };
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_term_with_factor_and_exponent() {
            let lhs = Term::Atom(Atom::Meter);
            let rhs = Term::FPAE {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                factor: 20,
                exponent: 2,
            };
            assert!(!lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms() {
            let lhs = vec![Term::Atom(Atom::Meter)];
            let rhs = vec![Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            }];
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms_with_factor() {
            let lhs = vec![Term::Atom(Atom::Meter)];
            let rhs = vec![Term::FPA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                factor: 20,
            }];
            assert!(lhs.is_compatible_with(&rhs));
        }

        #[test]
        fn validate_terms_with_factor_and_exponent() {
            let lhs = vec![Term::Atom(Atom::Meter)];
            let rhs = vec![Term::FPAE {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                factor: 20,
                exponent: 2,
            }];
            assert!(!lhs.is_compatible_with(&rhs));
        }
    }

    mod with_annotations {
        use super::*;

        #[test]
        fn validate_term() {
            let m = Term::AA {
                atom: Atom::Meter,
                annotation: "stuff".to_string(),
            };
            let km_stuff = Term::PAA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: "stuff".to_string(),
            };
            assert!(m.is_compatible_with(&km_stuff));

            // Different annotation
            let km_pants = Term::PAA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: "pants".to_string(),
            };
            assert!(!m.is_compatible_with(&km_pants));

            // No annotation
            let km_no_annotation = Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            };
            assert!(!m.is_compatible_with(&km_no_annotation));
        }

        #[test]
        fn validate_terms() {
            let m = vec![Term::AA {
                atom: Atom::Meter,
                annotation: "stuff".to_string(),
            }];
            let km_stuff = vec![Term::PAA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: "stuff".to_string(),
            }];
            assert!(m.is_compatible_with(&km_stuff));

            // Different annotation
            let km_pants = vec![Term::PAA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: "pants".to_string(),
            }];
            assert!(!m.is_compatible_with(&km_pants));

            // No annotation
            let km_no_annotation = vec![Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            }];
            assert!(!m.is_compatible_with(&km_no_annotation));
        }
    }
}
