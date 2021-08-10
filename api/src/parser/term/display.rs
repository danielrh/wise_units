use super::Term;
use std::fmt;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", extract_term_string(self))
    }
}

fn extract_term_string(term: &Term) -> String {
    if term.is_unity() && term.annotation().is_none() {
        return 1.to_string();
    };

    let mut term_string = String::new();
    extract_term_string_factor(&mut term_string, term.factor());
    extract_term_string_atom(&mut term_string, term);

    if let Some(ref annotation) = term.annotation() {
        term_string.push_str(&format!("{{{}}}", annotation));
    }

    term_string
}

fn extract_term_string_factor(term_string: &mut String, term_factor: Option<u32>) {
    if let Some(factor) = term_factor {
        if factor != 1 {
            term_string.push_str(&factor.to_string())
        }
    }
}

fn extract_term_string_atom(term_string: &mut String, term: &Term) {
    if let Some(atom) = term.atom() {
        if let Some(prefix) = term.prefix() {
            term_string.push_str(&prefix.to_string());
        }

        match term.exponent() {
            Some(exponent) => {
                if exponent == 1 {
                    term_string.push_str(&atom.to_string());
                } else {
                    term_string.push_str(&format!("{}{}", atom, exponent));
                }
            }
            None => term_string.push_str(&atom.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Atom, Prefix, Term};

    macro_rules! validate_display {
        ($test_name:ident, $term:expr, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_eq!(term.to_string().as_str(), $output);
            }
        };
    }

    validate_display!(
        validate_empty_annotation,
        Term::Annotation("seed".to_string()),
        "{seed}"
    );
    validate_display!(validate_unity, Term::new_unity(), "1");
    validate_display!(validate_atom, Term::Atom(Atom::Meter), "m");
    validate_display!(
        validate_atom_exponent1,
        Term::AE {
            atom: Atom::Meter,
            exponent: -1
        },
        "m-1"
    );
    validate_display!(
        validate_atom_exponent2,
        Term::AE {
            atom: Atom::Meter,
            exponent: -2
        },
        "m-2"
    );
    validate_display!(
        validate_atom_exponent1_annotation,
        Term::AEA {
            atom: Atom::Meter,
            exponent: -1,
            annotation: "seed".to_string()
        },
        "m-1{seed}"
    );
    validate_display!(
        validate_prefix_atom,
        Term::PA {
            prefix: Prefix::Kilo,
            atom: Atom::Meter
        },
        "km"
    );
    validate_display!(
        validate_prefix_atom_annotation,
        Term::PAA {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            annotation: "meow".to_string()
        },
        "10km{meow}"
    );
    validate_display!(
        validate_prefix_atom_exponent,
        Term::PAE {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -2,
        },
        "10km-1{seed}"
    );
    validate_display!(
        validate_prefix_atom_exponent_annotation,
        Term::PAEA {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -2,
            annotation: "seed".to_string()
        },
        "10km-1{seed}"
    );
    validate_display!(validate_factor, Term::Factor(10), "10");
    validate_display!(
        validate_factor_exponent,
        Term::FE {
            factor: 10,
            exponent: 10,
        },
        "10m"
    );
    validate_display!(
        validate_factor_atom,
        Term::FA {
            factor: 10,
            atom: Atom::Meter,
        },
        "10m"
    );
    validate_display!(
        validate_factor_atom_annotation,
        Term::FAA {
            factor: 5,
            atom: Atom::Meter,
            annotation: "taco".to_string()
        },
        "5m-1"
    );
    validate_display!(
        validate_factor_atom_exponent,
        Term::FAE {
            factor: 5,
            atom: Atom::Meter,
            exponent: -1,
        },
        "5m-1"
    );
    validate_display!(
        validate_factor_atom_exponent_annotation,
        Term::FAEA {
            factor: 5,
            atom: Atom::Meter,
            exponent: -1,
            annotation: "seed".to_string()
        },
        "5m-1{seed}"
    );
    validate_display!(
        validate_factor_prefix_atom,
        Term::FPA {
            factor: 10,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
        },
        "10km"
    );
    validate_display!(
        validate_factor_prefix_atom_annotation,
        Term::FPAA {
            factor: 10,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            annotation: "meow".to_string()
        },
        "10km{meow}"
    );
    validate_display!(
        validate_factor_prefix_atom_exponent,
        Term::FPAE {
            factor: 10,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -1
        },
        "10km-1"
    );
    validate_display!(
        validate_factor_prefix_atom_exponent_annotation,
        Term::FPAEA {
            factor: 10,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -1,
            annotation: "seed".to_string()
        },
        "10km-1{seed}"
    );
}
