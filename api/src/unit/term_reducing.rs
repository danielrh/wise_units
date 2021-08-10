use crate::parser::{Atom, Prefix, Term};
use std::collections::BTreeMap;

/// Internal struct used for reducing `Term`s.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ComposableTerm {
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
    FAA {
        factor: u32,
        atom: Atom,
        annotation: String,
    },
    FA {
        factor: u32,
        atom: Atom,
    },
    Factor(u32),
    PAA {
        prefix: Prefix,
        atom: Atom,
        annotation: String,
    },
    PA {
        prefix: Prefix,
        atom: Atom,
    },
    AA {
        atom: Atom,
        annotation: String,
    },
    Atom(Atom),
    Annotation(String),
}

impl<'a> From<&'a Term> for ComposableTerm {
    fn from(term: &'a Term) -> Self {
        // Self {
        //     atom: term.atom(),
        //     prefix: term.prefix(),
        //     factor: term.factor(),
        //     annotation: term.annotation().cloned(),
        // }
        match term {
            Term::FPAEA {
                factor,
                prefix,
                atom,
                annotation,
                ..
            }
            | Term::FPAA {
                factor,
                prefix,
                atom,
                annotation,
            } => Self::FPAA {
                factor: *factor,
                prefix: *prefix,
                atom: *atom,
                annotation: annotation.clone(),
            },
            Term::FPAE {
                factor,
                prefix,
                atom,
                ..
            }
            | Term::FPA {
                factor,
                prefix,
                atom,
            } => Self::FPA {
                factor: *factor,
                prefix: *prefix,
                atom: *atom,
            },
            Term::FAEA {
                factor,
                atom,
                annotation,
                ..
            }
            | Term::FAA {
                factor,
                atom,
                annotation,
            } => Self::FAA {
                factor: *factor,
                atom: *atom,
                annotation: annotation.clone(),
            },
            Term::FAE { factor, atom, .. } | Term::FA { factor, atom } => Self::FA {
                factor: *factor,
                atom: *atom,
            },
            Term::FE { factor, .. } | Term::Factor(factor) => Self::Factor(*factor),
            Term::PAEA {
                prefix,
                atom,
                annotation,
                ..
            }
            | Term::PAA {
                prefix,
                atom,
                annotation,
            } => Self::PAA {
                prefix: *prefix,
                atom: *atom,
                annotation: annotation.clone(),
            },
            Term::PAE { prefix, atom, .. } | Term::PA { prefix, atom } => Self::PA {
                prefix: *prefix,
                atom: *atom,
            },
            Term::AEA {
                atom, annotation, ..
            }
            | Term::AA { atom, annotation } => Self::AA {
                atom: *atom,
                annotation: annotation.clone(),
            },
            Term::AE { atom, .. } | Term::Atom(atom) => Self::Atom(*atom),
            Term::Annotation(annotation) => Self::Annotation(annotation.clone()),
        }
    }
}

type Parts = (ComposableTerm, i32);

impl From<Parts> for Term {
    fn from(parts: Parts) -> Self {
        let exponent = if parts.1 == 1 { None } else { Some(parts.1) };

        match (parts.0, exponent) {
            (
                ComposableTerm::FPAA {
                    factor,
                    prefix,
                    atom,
                    annotation,
                },
                Some(e),
            ) => Self::FPAEA {
                factor,
                prefix,
                atom,
                exponent: e,
                annotation,
            },
            (
                ComposableTerm::FPAA {
                    factor,
                    prefix,
                    atom,
                    annotation,
                },
                None,
            ) => Self::FPAA {
                factor,
                prefix,
                atom,
                annotation,
            },
            (
                ComposableTerm::FPA {
                    factor,
                    prefix,
                    atom,
                },
                Some(e),
            ) => Self::FPAE {
                factor,
                prefix,
                atom,
                exponent: e,
            },
            (
                ComposableTerm::FPA {
                    factor,
                    prefix,
                    atom,
                },
                None,
            ) => Self::FPA {
                factor,
                prefix,
                atom,
            },
            (
                ComposableTerm::FAA {
                    factor,
                    atom,
                    annotation,
                },
                Some(e),
            ) => Self::FAEA {
                factor,
                atom,
                exponent: e,
                annotation,
            },
            (
                ComposableTerm::FAA {
                    factor,
                    atom,
                    annotation,
                },
                None,
            ) => Self::FAA {
                factor,
                atom,
                annotation,
            },
            (ComposableTerm::FA { factor, atom }, Some(e)) => Self::FAE {
                factor,
                atom,
                exponent: e,
            },
            (ComposableTerm::FA { factor, atom }, None) => Self::FA { factor, atom },
            (ComposableTerm::Factor(factor), Some(e)) => Self::FE {
                factor,
                exponent: e,
            },
            (ComposableTerm::Factor(factor), None) => Self::Factor(factor),
            (
                ComposableTerm::PAA {
                    prefix,
                    atom,
                    annotation,
                },
                Some(e),
            ) => Self::PAEA {
                prefix,
                atom,
                exponent: e,
                annotation,
            },
            (
                ComposableTerm::PAA {
                    prefix,
                    atom,
                    annotation,
                },
                None,
            ) => Self::PAA {
                prefix,
                atom,
                annotation,
            },
            (ComposableTerm::PA { prefix, atom }, Some(e)) => Self::PAE {
                prefix,
                atom,
                exponent: e,
            },
            (ComposableTerm::PA { prefix, atom }, None) => Self::PA { prefix, atom },
            (ComposableTerm::AA { atom, annotation }, Some(e)) => Self::AEA {
                atom,
                exponent: e,
                annotation,
            },
            (ComposableTerm::AA { atom, annotation }, None) => Self::AA { atom, annotation },
            (ComposableTerm::Atom(atom), Some(e)) => Self::AE { atom, exponent: e },
            (ComposableTerm::Atom(atom), None) => Self::Atom(atom),
            (ComposableTerm::Annotation(_annotation), Some(e)) => todo!(),
            (ComposableTerm::Annotation(annotation), None) => Self::Annotation(annotation),
        }
    }
}

/// Function used in `Unit` for reducing its `Term`s.
///
pub(super) fn reduce_terms(terms: &[Term]) -> Vec<Term> {
    let map = reduce_to_map(terms);

    // If everything is reduced away, the effective Unit should be "1".
    if map.is_empty() {
        vec![Term::new_unity()]
    } else {
        // Reconstructs the map into the Vec<Term>.
        map.into_iter().map(Term::from).collect()
    }
}

/// Iterates through `terms`, finds `Term`s that have the same attributes that determine
/// uniqueness (`atom`, `prefix`, `factor`), and sums those exponents. This is the destructuring
/// part of `reduce_terms()`.
///
fn reduce_to_map(terms: &[Term]) -> BTreeMap<ComposableTerm, i32> {
    terms
        .iter()
        .map(|term| (ComposableTerm::from(term), term.exponent().unwrap_or(1)))
        .fold(
            BTreeMap::<ComposableTerm, i32>::new(),
            |mut map, (key, exponent)| {
                let _ = map
                    .entry(key)
                    .and_modify(|entry| *entry += exponent)
                    .or_insert(exponent);

                map
            },
        )
        .into_iter()
        // Filter out things that have no values
        .filter(|(_, exponent)| *exponent != 0)
        .collect()
}
