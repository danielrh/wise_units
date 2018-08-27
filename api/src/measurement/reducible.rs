use measurement::Measurement;
use num_help::BR_1;
use num_rational::BigRational;
use reducible::Reducible;
use ucum_unit::UcumUnit;

impl Reducible for Measurement {
    fn reduce_value(&self, value: BigRational) -> BigRational {
        self.reduce_value(&value)
    }

    fn calculate_magnitude(&self, value: BigRational) -> BigRational {
        self.calculate_magnitude(&value)
    }
}

impl<'a> Reducible<&'a BigRational> for Measurement {
    fn reduce_value(&self, value: &'a BigRational) -> BigRational {
        if self.is_special() {
            self.unit.reduce_value(value)
        } else {
            value * self.unit.reduce_value(&*BR_1)
        }
    }

    fn calculate_magnitude(&self, value: &'a BigRational) -> BigRational {
        if self.is_special() {
            let scalar = self.scalar();
            self.unit.calculate_magnitude(scalar)
        } else {
            value * self.unit.calculate_magnitude(&*BR_1)
        }
    }
}

#[cfg(test)]
mod tests {
    use num_help::{BR_1, BR_PI};
    use reducible::Reducible;
    use measurement::Measurement;

    macro_rules! validate_reduce_value {
        ($test_name:ident, $measurement_value:expr, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let measurement = Measurement::new_try_unit($measurement_value, $unit_str).unwrap();
                assert_eq!(measurement.reduce_value(&*BR_1), $expected_value);
            }
        };
    }

    macro_rules! validate_calculate_magnitude {
        ($test_name:ident, $measurement_value:expr, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let measurement = Measurement::new_try_unit($measurement_value, $unit_str).unwrap();
                assert_eq!(measurement.calculate_magnitude(&*BR_1), $expected_value);
            }
        };
    }

    // reduce_value tests
    validate_reduce_value!(validate_reduce_value_m, 1, "m", BR_1.clone());
    validate_reduce_value!(validate_reduce_value_km, 1, "km", big_rational_raw!(1000));
    validate_reduce_value!(
        validate_reduce_value_meter_minus1,
        1,
        "m-1",
        BR_1.clone()
    );
    validate_reduce_value!(
        validate_reduce_value_meter_factor,
        1,
        "10m",
        big_rational_raw!(10)
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor,
        1,
        "10km",
        big_rational_raw!(10_000)
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor_exponent,
        1,
        "10km-1",
        big_rational_raw!(1, 10_000)
    );
    validate_reduce_value!(validate_reduce_value_liter, 1, "L", big_rational_raw!(1, 1000));
    validate_reduce_value!(
        validate_reduce_value_pi,
        1,
        "[pi]",
        BR_PI.clone()
    );
    validate_reduce_value!(
        validate_reduce_value_pi_factor,
        1,
        "10[pi]",
        &*BR_PI * 10
    );
    validate_reduce_value!(validate_reduce_value_hectare, 1, "har", big_rational_raw!(10_000));
    validate_reduce_value!(validate_reduce_value_week, 1, "wk", big_rational_raw!(604_800));
    validate_reduce_value!(validate_reduce_value_kilogram, 1, "kg", big_rational_raw!(1000));
    validate_reduce_value!(
        validate_reduce_value_fahrenheit,
        1,
        "[degF]",
        big_rational_raw!(255_927_777_777_777_8, 10000000000000)
    );

    // magnitude tests
    validate_calculate_magnitude!(validate_calculate_magnitude_meter, 1, "m", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer,
        1,
        "km",
        big_rational_raw!(1000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_minus1,
        1,
        "m-1",
        BR_1.clone()
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_factor,
        1,
        "10m",
        big_rational_raw!(10)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor,
        1,
        "10km",
        big_rational_raw!(10_000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor_exponent,
        1,
        "10km-1",
        big_rational_raw!(1, 10_000)
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_liter, 1, "L", BR_1.clone());
    validate_calculate_magnitude!(validate_calculate_magnitude_pi, 1, "[pi]", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_pi_factor,
        1,
        "10[pi]",
        big_rational_raw!(10)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_hectare,
        1,
        "har",
        big_rational_raw!(100)
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_week, 1, "wk", BR_1.clone());
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilogram,
        1,
        "kg",
        big_rational_raw!(1000)
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_fahrenheit,
        1,
        "[degF]",
        BR_1.clone()
    );
}
