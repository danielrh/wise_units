use crate::parser::{
    annotation_composition::{AnnotationComposable, AnnotationComposition},
    Term,
};

/// Similar to `Composable`, this is only to allow for checking compatibility on `Unit`s whose
/// `Term`s have annotations. For those cases, we want to be able to ensure that, for example,
/// `m{foo}` is not comparable to `m{bar}`. This implementation treats each `Term`s `annotation`
/// as its own `Dimension` of sorts, allowing `m2{foo}/m{foo}` to be comparable to `m{foo}`, since
/// they have equivalent `AnnotationComposable`s.
///
impl<'a> AnnotationComposable<'a> for &'a [Term] {
    fn annotation_composition(self) -> Option<AnnotationComposition<'a>> {
        let mut map = self
            .iter()
            .filter_map(|term| {
                term.annotation()
                    .as_ref()
                    .map(|annotation| (annotation.clone(), term.exponent().unwrap_or(1)))
            })
            .fold(AnnotationComposition::new(), |mut map, (key, exponent)| {
                let _ = map
                    .entry(key)
                    .and_modify(|entry| *entry += exponent)
                    .or_insert(exponent);

                map
            });

        // Filter out things that have no values
        map.retain(|_annotation, exponent| *exponent != 0);

        if map.is_empty() {
            None
        } else {
            Some(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::annotation_composition::{AnnotationComposable, AnnotationComposition};
    use crate::parser::Atom;
    use crate::parser::Term;

    mod without_annotations {
        use super::*;

        #[test]
        fn validate_no_atom() {
            let terms = &[Term::Factor(42)];
            assert!(terms.annotation_composition().is_none());
        }

        #[test]
        fn validate_with_atom() {
            let terms = &[Term::Atom(Atom::Meter)];
            assert!(terms.annotation_composition().is_none());
        }

        #[test]
        fn validate_with_atom_and_exponent() {
            let terms = &[
                Term::AE {
                    atom: Atom::Meter,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: -1,
                },
            ];
            assert!(terms.annotation_composition().is_none());
        }
    }

    mod with_annotations {
        use super::*;

        #[test]
        fn validate_no_atom() {
            let terms = &[Term::Annotation("foo".to_string())];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo", 1);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atom() {
            let terms = &[Term::AA {
                atom: Atom::Meter,
                annotation: "foo".to_string(),
            }];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo", 1);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atom_and_negative_exponent() {
            let terms = &[Term::AEA {
                atom: Atom::Meter,
                exponent: -2,
                annotation: "foo".to_string(),
            }];

            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("foo", -2);

            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atoms_and_positive_and_negative_exponents() {
            let terms = &[
                Term::AEA {
                    atom: Atom::Gram,
                    exponent: 3,
                    annotation: "bar".to_string(),
                },
                Term::AEA {
                    atom: Atom::Meter,
                    exponent: -2,
                    annotation: "foo".to_string(),
                },
            ];
            let mut annotation_composition = AnnotationComposition::new();
            let _ = annotation_composition.insert("bar", 3);
            let _ = annotation_composition.insert("foo", -2);
            assert_eq!(terms.annotation_composition(), Some(annotation_composition));
        }

        #[test]
        fn validate_with_atoms_cancelling_exponents() {
            let terms = &[
                Term::AEA {
                    atom: Atom::Meter,
                    exponent: 2,
                    annotation: "foo".to_string(),
                },
                Term::AEA {
                    atom: Atom::Meter,
                    exponent: -2,
                    annotation: "foo".to_string(),
                },
            ];
            assert!(terms.annotation_composition().is_none());
        }
    }
}
