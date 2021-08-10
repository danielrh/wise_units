// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
pub(self) mod annotatable;
pub(self) mod annotation;
pub(self) mod ast_term;
pub(self) mod component;
pub(self) mod digits;
pub(self) mod exponent;
pub(self) mod factor;
pub(self) mod finishable;
pub(self) mod main_term;
pub(self) mod simple_unit;

use self::{
    annotatable::Annotatable, annotation::Annotation, ast_term::AstTerm, component::Component,
    digits::Digits, exponent::Exponent, factor::Factor, finishable::Finishable,
    main_term::MainTerm, simple_unit::SimpleUnit,
};
use crate::parser::{terms::term_parser::Rule, Atom, Error, Prefix, Term, Visit};
use pest::iterators::{Pair, Pairs};

pub(crate) fn map(mut pairs: Pairs<'_, Rule>) -> Result<Vec<Term>, Error> {
    fn visit_pairs(pair: Pair<'_, Rule>) -> Result<Vec<Term>, Error> {
        let main_term = if let Rule::main_term = pair.as_rule() {
            MainTerm::visit(pair)?
        } else {
            return Err(Error::UnknownUnitString(
                pair.as_span().as_str().to_string(),
            ));
        };

        let mut terms: Vec<Term> = main_term.into();
        terms.shrink_to_fit();

        Ok(terms)
    }

    match pairs.next() {
        Some(pair) => Ok(visit_pairs(pair)?),
        None => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::terms::term_parser::{Rule, TermParser};
    use crate::parser::{Atom, Prefix, Term};
    use pest::Parser;

    macro_rules! validate_interpret {
        ($test_name:ident, $input:expr, $($terms:expr),+) => {
            #[test]
            fn $test_name() {
                let pairs = TermParser::parse(Rule::main_term, $input).unwrap();
                let actual = map(pairs).unwrap();
                let expected = vec![$($terms),+];

                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m-3").unwrap();
        let actual = map(pairs).unwrap();

        let expected_term = Term::AE {
            atom: Atom::Meter,
            exponent: -3,
        };
        let expected = vec![expected_term];

        assert_eq!(actual, expected);

        let pairs = TermParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = map(pairs).unwrap();

        let term1 = Term::PAE {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: 2,
        };
        let term2 = Term::AE {
            atom: Atom::Meter,
            exponent: 3,
        };
        let expected = vec![term1, term2];

        assert_eq!(actual, expected);
    }

    validate_interpret!(validate_interpret_meter, "m", Term::Atom(Atom::Meter));
    validate_interpret!(
        validate_interpret_meter_positive_exponent,
        "m2",
        Term::AE {
            atom: Atom::Meter,
            exponent: 2
        }
    );
    validate_interpret!(
        validate_interpret_meter_negative_exponent,
        "m-2",
        Term::AE {
            atom: Atom::Meter,
            exponent: -2
        }
    );
    validate_interpret!(
        validate_interpret_meter_factor,
        "2m",
        Term::FA {
            atom: Atom::Meter,
            factor: 2
        }
    );
    validate_interpret!(
        validate_interpret_kilometer,
        "km",
        Term::PA {
            prefix: Prefix::Kilo,
            atom: Atom::Meter
        }
    );

    // Slash terms
    validate_interpret!(
        validate_interpret_meter_per_second,
        "m/s",
        Term::Atom(Atom::Meter),
        Term::AE {
            atom: Atom::Second,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_kilometer_per_second,
        "km/s",
        Term::PA {
            prefix: Prefix::Kilo,
            atom: Atom::Meter
        },
        Term::AE {
            atom: Atom::Second,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_meter_per_kilosecond,
        "m/ks",
        Term::Atom(Atom::Meter),
        Term::PAE {
            prefix: Prefix::Kilo,
            atom: Atom::Second,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_meter2_per_second,
        "m2/s",
        Term::AE {
            atom: Atom::Meter,
            exponent: 2
        },
        Term::AE {
            atom: Atom::Second,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_meter_per_second2,
        "m/s2",
        Term::Atom(Atom::Meter),
        Term::AE {
            atom: Atom::Second,
            exponent: -2
        }
    );
    validate_interpret!(
        validate_interpret_meter2_per_second2,
        "m2/s2",
        Term::AE {
            atom: Atom::Meter,
            exponent: 2
        },
        Term::AE {
            atom: Atom::Second,
            exponent: -2
        }
    );
    validate_interpret!(
        validate_interpret_2meter_per_second,
        "2m/s",
        Term::FA {
            atom: Atom::Meter,
            factor: 2
        },
        Term::AE {
            atom: Atom::Second,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_meter_per_2second,
        "m/2s",
        Term::Atom(Atom::Meter),
        Term::FAE {
            atom: Atom::Second,
            exponent: -1,
            factor: 2
        }
    );
    validate_interpret!(
        validate_interpret_2meter2_per_2second2,
        "2m2/2s2",
        Term::FAE {
            atom: Atom::Meter,
            factor: 2,
            exponent: 2
        },
        Term::FAE {
            atom: Atom::Second,
            factor: 2,
            exponent: -2
        }
    );
    validate_interpret!(
        validate_interpret_foot_per_factor,
        "[ft_i]/12",
        Term::Atom(Atom::FootInternational),
        Term::FE {
            factor: 12,
            exponent: -1
        }
    );

    // Dot terms
    validate_interpret!(
        validate_interpret_meter_second,
        "m.s",
        Term::Atom(Atom::Meter),
        Term::Atom(Atom::Second)
    );
    validate_interpret!(
        validate_interpret_meter2_second,
        "m2.s",
        Term::AE {
            atom: Atom::Meter,
            exponent: 2
        },
        Term::Atom(Atom::Second)
    );
    validate_interpret!(
        validate_interpret_meter_second2,
        "m.s2",
        Term::Atom(Atom::Meter),
        Term::AE {
            atom: Atom::Second,
            exponent: 2
        }
    );
    validate_interpret!(
        validate_interpret_meter2_second2,
        "m2.s2",
        Term::AE {
            atom: Atom::Meter,
            exponent: 2
        },
        Term::AE {
            atom: Atom::Second,
            exponent: 2
        }
    );
    validate_interpret!(
        validate_interpret_2meter_second,
        "2m.s",
        Term::FA {
            factor: 2,
            atom: Atom::Meter,
        },
        Term::Atom(Atom::Second)
    );
    validate_interpret!(
        validate_interpret_meter_2second,
        "m.2s",
        Term::Atom(Atom::Meter),
        Term::FA {
            factor: 2,
            atom: Atom::Second,
        }
    );
    validate_interpret!(
        validate_interpret_2meter_2second,
        "2m.2s",
        Term::FA {
            factor: 2,
            atom: Atom::Meter,
        },
        Term::FA {
            factor: 2,
            atom: Atom::Second,
        }
    );
    validate_interpret!(
        validate_interpret_2meter2_2second2,
        "2m2.2s2",
        Term::FAE {
            factor: 2,
            atom: Atom::Meter,
            exponent: 2
        },
        Term::FAE {
            factor: 2,
            atom: Atom::Second,
            exponent: 2
        }
    );

    // Dot and slash combined terms
    validate_interpret!(
        validate_interpret_acre_inch_per_acre,
        "[acr_us].[in_i]/[acr_us]",
        Term::Atom(Atom::AcreUS),
        Term::Atom(Atom::InchInternational),
        Term::AE {
            atom: Atom::AcreUS,
            exponent: -1
        }
    );
    validate_interpret!(
        validate_interpret_2acre3_3inch4_per_4acre5,
        "2[acr_us]3.3[in_i]4/4[acr_us]5",
        Term::FAE {
            atom: Atom::AcreUS,
            factor: 2,
            exponent: 3
        },
        Term::FAE {
            atom: Atom::InchInternational,
            factor: 3,
            exponent: 4
        },
        Term::FAE {
            atom: Atom::AcreUS,
            factor: 4,
            exponent: -5
        }
    );

    #[test]
    #[ignore]
    fn validate_custom_atom() {
        let pairs = TermParser::parse(Rule::main_term, "[meow]").unwrap();

        let actual = map(pairs).unwrap();
        let acre_term = Term::Atom(Atom::AcreUS);
        let inch_term = Term::Atom(Atom::InchInternational);
        let acre2_term = Term::AE {
            atom: Atom::AcreUS,
            exponent: -1,
        };

        let expected = vec![acre_term, inch_term, acre2_term];

        assert_eq!(actual, expected);
    }
}
