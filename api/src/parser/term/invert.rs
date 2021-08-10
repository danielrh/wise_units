use super::Term;
use crate::invert::{Invert, ToInverse};

// Term
impl Invert for Term {
    fn invert(self) -> Self {
        match self {
            Term::FPAEA {
                factor,
                prefix,
                atom,
                exponent,
                annotation,
            } => {
                if exponent == -1 {
                    Term::FPAA {
                        factor,
                        prefix,
                        atom,
                        annotation,
                    }
                } else {
                    Term::FPAEA {
                        factor,
                        prefix,
                        atom,
                        exponent: -exponent,
                        annotation,
                    }
                }
            }
            Term::FPAE {
                factor,
                prefix,
                atom,
                exponent,
            } => {
                if exponent == -1 {
                    Term::FPA {
                        factor,
                        prefix,
                        atom,
                    }
                } else {
                    Term::FPAE {
                        factor,
                        prefix,
                        atom,
                        exponent: -exponent,
                    }
                }
            }
            Term::FPAE {
                factor,
                prefix,
                atom,
                exponent,
            } => {
                if exponent == -1 {
                    Term::FPA {
                        factor,
                        prefix,
                        atom,
                    }
                } else {
                    Term::FPAE {
                        factor,
                        prefix,
                        atom,
                        exponent: -exponent,
                    }
                }
            }
            Term::FPAA {
                factor,
                prefix,
                atom,
                annotation,
            } => Term::FPAEA {
                factor,
                prefix,
                atom,
                exponent: -1,
                annotation,
            },
            Term::FPA {
                factor,
                prefix,
                atom,
            } => Term::FPAE {
                factor,
                prefix,
                atom,
                exponent: -1,
            },
            Term::FAEA {
                factor,
                atom,
                exponent,
                annotation,
            } => {
                if exponent == -1 {
                    Term::FAA {
                        factor,
                        atom,
                        annotation,
                    }
                } else {
                    Term::FAEA {
                        factor,
                        atom,
                        exponent: -exponent,
                        annotation,
                    }
                }
            }
            Term::FAE {
                factor,
                atom,
                exponent,
            } => {
                if exponent == -1 {
                    Term::FA { factor, atom }
                } else {
                    Term::FAE {
                        factor,
                        atom,
                        exponent: -exponent,
                    }
                }
            }
            Term::FAA {
                factor,
                atom,
                annotation,
            } => Term::FAEA {
                factor,
                atom,
                exponent: -1,
                annotation,
            },
            Term::FA { factor, atom } => Term::FAE {
                factor,
                atom,
                exponent: -1,
            },
            Term::FE { factor, exponent } => {
                if exponent == -1 {
                    Term::Factor(factor)
                } else {
                    Term::FE {
                        factor,
                        exponent: -1,
                    }
                }
            }
            Term::Factor(factor) => Term::FE {
                factor,
                exponent: -1,
            },
            Term::PAEA {
                prefix,
                atom,
                exponent,
                annotation,
            } => {
                if exponent == -1 {
                    Term::PAA {
                        prefix,
                        atom,
                        annotation,
                    }
                } else {
                    Term::PAEA {
                        prefix,
                        atom,
                        exponent: -exponent,
                        annotation,
                    }
                }
            }
            Term::PAE {
                prefix,
                atom,
                exponent,
            } => {
                if exponent == -1 {
                    Term::PA { prefix, atom }
                } else {
                    Term::PAE {
                        prefix,
                        atom,
                        exponent: -exponent,
                    }
                }
            }
            Term::PAA {
                prefix,
                atom,
                annotation,
            } => Term::PAEA {
                prefix,
                atom,
                exponent: -1,
                annotation,
            },
            Term::PA { prefix, atom } => Term::PAE {
                prefix,
                atom,
                exponent: -1,
            },
            Term::AEA {
                atom,
                exponent,
                annotation,
            } => {
                if exponent == -1 {
                    Term::AA { atom, annotation }
                } else {
                    Term::AEA {
                        atom,
                        exponent: -exponent,
                        annotation,
                    }
                }
            }
            Term::AE { atom, exponent } => {
                if exponent == -1 {
                    Term::Atom(atom)
                } else {
                    Term::AE {
                        atom,
                        exponent: -exponent,
                    }
                }
            }
            Term::AA { atom, annotation } => Term::AEA {
                atom,
                exponent: -1,
                annotation,
            },
            Term::Atom(atom) => Term::AE { atom, exponent: -1 },
            Term::Annotation(_annotation) => {
                todo!()
            }
        }
    }
}

impl ToInverse for Term {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        self.clone().invert()
    }
}

// Vec<Term>
impl Invert for Vec<Term> {
    fn invert(self) -> Self {
        let mut v = Vec::with_capacity(self.len());

        for term in self.into_iter() {
            v.push(term.invert());
        }

        v
    }
}

impl ToInverse for Vec<Term> {
    type Output = Self;

    fn to_inverse(&self) -> Self::Output {
        self.iter().map(ToInverse::to_inverse).collect()
    }
}

#[cfg(test)]
mod tests {
    mod term {
        use crate::{invert::Invert, invert::ToInverse, Atom, Term};

        #[test]
        fn validate_invert_numerator_no_exponent() {
            let term = Term::Atom(Atom::Meter);
            assert_eq!(
                term.invert(),
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }
            );
        }

        #[test]
        fn validate_invert_numerator_with_exponent_1() {
            let term = Term::AE {
                atom: Atom::Meter,
                exponent: 1,
            };
            assert_eq!(
                term.invert(),
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }
            );
        }

        #[test]
        fn validate_invert_denominator_with_exponent_minus_1() {
            let term = Term::AE {
                atom: Atom::Meter,
                exponent: -1,
            };
            assert_eq!(term.invert(), Term::Atom(Atom::Meter));
        }

        #[test]
        fn validate_to_inverse_numerator_no_exponent() {
            let term = Term::Atom(Atom::Meter);
            let new_term = term.to_inverse();
            assert_eq!(
                new_term,
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }
            );
        }

        #[test]
        fn validate_to_inverse_numerator_with_exponent() {
            let term = Term::AE {
                atom: Atom::Meter,
                exponent: 1,
            };
            let new_term = term.to_inverse();
            assert_eq!(
                new_term,
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }
            );
        }

        #[test]
        fn validate_to_inverse_denominator_with_exponent_minus_1() {
            let term = Term::AE {
                atom: Atom::Meter,
                exponent: -1,
            };
            let new_term = term.to_inverse();
            assert_eq!(new_term, Term::Atom(Atom::Meter));
        }
    }

    mod terms {
        use crate::{invert::Invert, invert::ToInverse, Atom, Prefix, Term};

        #[test]
        fn validate_invert_numerator_no_exponent() {
            let terms = vec![Term::Atom(Atom::Meter)];
            assert_eq!(
                terms.invert(),
                vec![Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }]
            );
        }

        #[test]
        fn validate_invert_numerator_with_exponent_1() {
            let terms = vec![Term::AE {
                atom: Atom::Meter,
                exponent: 1,
            }];
            assert_eq!(
                terms.invert(),
                vec![Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }]
            );
        }

        #[test]
        fn validate_invert_denominator_with_exponent_minus_1() {
            let terms = vec![Term::AE {
                atom: Atom::Meter,
                exponent: -1,
            }];
            assert_eq!(terms.invert(), vec![Term::Atom(Atom::Meter)]);
        }

        #[test]
        fn validate_invert_numerator_and_denominator() {
            let terms = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: -2,
                },
            ];
            assert_eq!(
                terms.invert(),
                vec![
                    Term::AE {
                        atom: Atom::Meter,
                        exponent: -2
                    },
                    Term::AE {
                        atom: Atom::Second,
                        exponent: 2
                    }
                ]
            );
        }

        #[test]
        fn validate_invert_numerators_and_denominators_mixed() {
            let terms = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: -2,
                },
                Term::AE {
                    atom: Atom::Gram,
                    exponent: -4,
                },
                Term::PAE {
                    prefix: Prefix::Kilo,
                    atom: Atom::Meter,
                    exponent: 4,
                },
                Term::PAE {
                    prefix: Prefix::Hecto,
                    atom: Atom::Are,
                    exponent: -5,
                },
            ];

            let result = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Gram,
                    exponent: 4,
                },
                Term::PAE {
                    prefix: Prefix::Kilo,
                    atom: Atom::Meter,
                    exponent: -4,
                },
                Term::PAE {
                    prefix: Prefix::Hecto,
                    atom: Atom::Are,
                    exponent: 5,
                },
            ];
            assert_eq!(terms.invert(), result);
        }

        #[test]
        fn validate_to_inverse_numerator_no_exponent() {
            let terms = vec![Term::Atom(Atom::Meter)];
            let new_terms = terms.to_inverse();
            assert_eq!(
                new_terms,
                vec![Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }]
            );
        }

        #[test]
        fn validate_to_inverse_numerator_with_exponent_1() {
            let terms = vec![Term::AE {
                atom: Atom::Meter,
                exponent: 1,
            }];
            let new_terms = terms.to_inverse();
            assert_eq!(
                new_terms,
                vec![Term::AE {
                    atom: Atom::Meter,
                    exponent: -1
                }]
            );
        }

        #[test]
        fn validate_to_inverse_denominator_with_exponent_minus_1() {
            let terms = vec![Term::AE {
                atom: Atom::Meter,
                exponent: -1,
            }];
            let new_terms = terms.to_inverse();
            assert_eq!(new_terms, vec![Term::Atom(Atom::Meter)]);
        }

        #[test]
        fn validate_to_inverse_numerator_and_denominator() {
            let terms = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: -2,
                },
            ];
            let new_terms = terms.to_inverse();
            assert_eq!(
                new_terms,
                vec![
                    Term::AE {
                        atom: Atom::Meter,
                        exponent: -2
                    },
                    Term::AE {
                        atom: Atom::Second,
                        exponent: 2
                    }
                ]
            );
        }

        #[test]
        fn validate_to_inverse_numerators_and_denominators_mixed() {
            let terms = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: -2,
                },
                Term::AE {
                    atom: Atom::Gram,
                    exponent: -4,
                },
                Term::PAE {
                    prefix: Prefix::Kilo,
                    atom: Atom::Meter,
                    exponent: 4,
                },
                Term::PAE {
                    prefix: Prefix::Hecto,
                    atom: Atom::Are,
                    exponent: -5,
                },
            ];
            let new_terms = terms.to_inverse();

            let result = vec![
                Term::AE {
                    atom: Atom::Meter,
                    exponent: -2,
                },
                Term::AE {
                    atom: Atom::Second,
                    exponent: 2,
                },
                Term::AE {
                    atom: Atom::Gram,
                    exponent: 4,
                },
                Term::PAE {
                    prefix: Prefix::Kilo,
                    atom: Atom::Meter,
                    exponent: -4,
                },
                Term::PAE {
                    prefix: Prefix::Hecto,
                    atom: Atom::Are,
                    exponent: 5,
                },
            ];
            assert_eq!(new_terms, result);
        }
    }
}
