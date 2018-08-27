use num_rational::BigRational;
use reducible::Reducible;
use unit::Unit;

//-----------------------------------------------------------------------------
// impl Reducible
//-----------------------------------------------------------------------------
impl Reducible for Unit {
    fn reduce_value(&self, value: BigRational) -> BigRational {
        self.reduce_value(&value)
    }

    /// Calculates `value` count of `self` in terms of `self`'s unit.
    ///
    fn calculate_magnitude(&self, value: BigRational) -> BigRational {
        self.calculate_magnitude(&value)
    }
}

impl<'a> Reducible<&'a BigRational> for Unit {
    fn reduce_value(&self, value: &'a BigRational) -> BigRational {
        self.terms.reduce_value(value)
    }

    /// Calculates `value` count of `self` in terms of `self`'s unit.
    ///
    fn calculate_magnitude(&self, value: &'a BigRational) -> BigRational {
        self.terms.calculate_magnitude(value)
    }
}

#[cfg(test)]
mod tests {
    use num_help::{BR_1, BR_PI};
    use reducible::Reducible;
    use std::str::FromStr;
    use unit::Unit;

    macro_rules! validate_reduce_value {
        ($test_name:ident, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($unit_str).unwrap();
                assert_eq!(unit.reduce_value(&*BR_1), $expected_value);
            }
        };
    }

    macro_rules! validate_calculate_magnitude {
        ($test_name:ident, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($unit_str).unwrap();
                let scalar = unit.reduce_value(&*BR_1);
                assert_eq!(unit.calculate_magnitude(scalar), $expected_value);
            }
        };
    }

    // reduce_value tests
    validate_reduce_value!(validate_reduce_value_m, "m", BR_1.clone());
    validate_reduce_value!(validate_reduce_value_km, "km", big_rational_raw!(1000));
    validate_reduce_value!(
        validate_reduce_value_meter_minus1,
        "m-1",
        BR_1.clone()
    );
    validate_reduce_value!(
        validate_reduce_value_meter_factor,
        "10m",
        big_rational_raw!(10)
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor,
        "10km",
        big_rational_raw!(10_000)
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor_exponent,
        "10km-1",
        big_rational_raw!(1, 100_000)
    );
    validate_reduce_value!(validate_reduce_value_liter, "L", big_rational_raw!(1, 1000));
    validate_reduce_value!(
        validate_reduce_value_pi,
        "[pi]",
        BR_PI.clone()
    );
    validate_reduce_value!(
        validate_reduce_value_pi_factor,
        "10[pi]",
        &*BR_PI * 10
    );
    validate_reduce_value!(validate_reduce_value_hectare, "har", big_rational_raw!(10_000));
    validate_reduce_value!(validate_reduce_value_week, "wk", big_rational_raw!(604_800));
    validate_reduce_value!(validate_reduce_value_kilogram, "kg", big_rational_raw!(1000));
    validate_reduce_value!(
        validate_reduce_value_fahrenheit,
        "[degF]",
        big_rational_raw!(2_559_277_777_777_778, 10000000000000)
    );

    // magnitude tests
    validate_calculate_magnitude!(validate_calculate_magnitude_meter, "m", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer,
        "km",
        big_rational_raw!(1000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_minus1,
        "m-1",
        BR_1.clone()
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_factor,
        "10m",
        big_rational_raw!(10)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor,
        "10km",
        big_rational_raw!(10_000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor_exponent,
        "10km-1",
        big_rational_raw!(1, 10_00)
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_liter, "L", BR_1.clone());
    validate_calculate_magnitude!(validate_calculate_magnitude_pi, "[pi]", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_pi_factor,
        "10[pi]",
        big_rational_raw!(10)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_hectare,
        "har",
        big_rational_raw!(100)
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_week, "wk", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilogram,
        "kg",
        big_rational_raw!(1000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_fahrenheit,
        "[degF]",
        BR_1.clone()
    );
}
