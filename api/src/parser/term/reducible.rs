use super::Term;
use crate::parser::ucum_symbol::UcumSymbol;
use crate::reducible::Reducible;

impl Reducible for Term {
    fn reduce_value(&self, value: f64) -> f64 {
        let atom_scalar = self.atom().map_or(1.0, |a| a.reduce_value(value));
        let prefix_scalar = self.prefix().map_or(1.0, |p| p.definition_value());

        combine_term_values(atom_scalar, prefix_scalar, self.factor(), self.exponent())
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        let atom_magnitude = self.atom().map_or(1.0, |a| a.calculate_magnitude(value));
        let prefix_magnitude = self.prefix().map_or(1.0, |p| p.definition_value());

        combine_term_values(
            atom_magnitude,
            prefix_magnitude,
            self.factor(),
            self.exponent(),
        )
    }
}

impl Reducible for Vec<Term> {
    fn reduce_value(&self, value: f64) -> f64 {
        self.iter()
            .fold(1.0, |acc, term| acc * term.reduce_value(value))
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        self.iter()
            .fold(1.0, |acc, term| acc * term.calculate_magnitude(value))
    }
}

fn combine_term_values(
    calculated_atom: f64,
    calculated_prefix: f64,
    factor: Option<u32>,
    exponent: Option<i32>,
) -> f64 {
    let a_p_product = calculated_atom * calculated_prefix;

    match factor {
        Some(f) => {
            let product = a_p_product * f64::from(f);

            match exponent {
                Some(e) => product.powi(e),
                None => product,
            }
        }
        None => match exponent {
            Some(e) => a_p_product.powi(e),
            None => a_p_product,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Atom, Prefix, Term};
    use crate::reducible::Reducible;
    use approx::assert_relative_eq;

    mod reduce_value {
        use super::*;

        macro_rules! validate_reduce_value {
            ($test_name:ident, $term:expr, $expected_value:expr) => {
                #[test]
                fn $test_name() {
                    let term = $term;
                    assert_relative_eq!(term.reduce_value(1.0), $expected_value);
                }
            };
        }

        validate_reduce_value!(validate_meter, Term::Atom(Atom::Meter), 1.0);
        validate_reduce_value!(
            validate_kilometer,
            Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter
            },
            1000.0
        );
        validate_reduce_value!(
            validate_meter_minus1,
            Term::AE {
                atom: Atom::Meter,
                exponent: -1
            },
            1.0
        );
        validate_reduce_value!(
            validate_meter_factor,
            Term::FA {
                factor: 10,
                atom: Atom::Meter,
            },
            10.0
        );
        validate_reduce_value!(
            validate_kilometer_factor,
            Term::FPA {
                factor: 10,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            },
            10_000.0
        );
        validate_reduce_value!(
            validate_kilometer_factor_exponent,
            Term::FPAE {
                factor: 10,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -1,
            },
            0.0001
        );
        validate_reduce_value!(validate_liter, Term::Atom(Atom::Liter), 0.001);
        validate_reduce_value!(
            validate_pi,
            Term::Atom(Atom::TheNumberPi),
            ::std::f64::consts::PI
        );
        validate_reduce_value!(
            validate_pi_factor,
            Term::FA {
                factor: 10,
                atom: Atom::TheNumberPi,
            },
            ::std::f64::consts::PI * 10.0
        );
        validate_reduce_value!(
            validate_hectare,
            Term::PA {
                prefix: Prefix::Hecto,
                atom: Atom::Are
            },
            10_000.0
        );
        validate_reduce_value!(validate_week, Term::Atom(Atom::Week), 604_800.0);
        validate_reduce_value!(
            validate_kilogram,
            Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Gram
            },
            1000.0
        );
        validate_reduce_value!(
            validate_fahrenheit,
            Term::Atom(Atom::DegreeFahrenheit),
            255.927_777_777_777_8
        );
    }

    mod calculate_magnitude {
        use super::*;

        macro_rules! validate_calculate_magnitude {
            ($test_name:ident, $term:expr, $expected_value:expr) => {
                #[test]
                fn $test_name() {
                    let term = $term;
                    let scalar = term.reduce_value(1.0);
                    assert_relative_eq!(term.calculate_magnitude(scalar), $expected_value);
                }
            };
        }

        // magnitude tests
        validate_calculate_magnitude!(
            validate_calculate_magnitude_meter,
            Term::Atom(Atom::Meter),
            1.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_kilometer,
            Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter
            },
            1000.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_meter_minus1,
            Term::AE {
                atom: Atom::Meter,
                exponent: -1
            },
            1.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_meter_factor,
            Term::FA {
                atom: Atom::Meter,
                factor: 10
            },
            10.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_kilometer_factor,
            Term::FPA {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                factor: 10
            },
            10_000.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_kilometer_factor_exponent,
            Term::FPAE {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -1,
                factor: 10
            },
            0.000_1
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_liter,
            Term::Atom(Atom::Liter),
            1.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_pi,
            Term::Atom(Atom::TheNumberPi),
            1.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_pi_factor,
            Term::FA {
                atom: Atom::TheNumberPi,
                factor: 10
            },
            10.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_hectare,
            Term::PA {
                prefix: Prefix::Hecto,
                atom: Atom::Are
            },
            100.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_week,
            Term::Atom(Atom::Week),
            1.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_kilogram,
            Term::PA {
                prefix: Prefix::Kilo,
                atom: Atom::Gram
            },
            1000.0
        );
        validate_calculate_magnitude!(
            validate_calculate_magnitude_fahrenheit,
            Term::Atom(Atom::DegreeFahrenheit),
            1.000_000_000_000_056_8
        );
    }
}
