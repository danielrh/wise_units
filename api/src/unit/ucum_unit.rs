use reducible::Reducible;
use num_help::BR_1;
use num_rational::BigRational;
use ucum_unit::UcumUnit;
use unit::Unit;

impl UcumUnit for Unit {
    /// The UCUM defines "special units" as:
    ///
    /// > units that imply a measurement on a scale other than a ratio scale
    ///
    /// Each atom in `Atoms.toml` has the `isSpecial` attribute; a `Unit` is
    /// special if any of its `Term`s has an `Atom` that is special.
    ///
    fn is_special(&self) -> bool {
        self.terms.iter().any(|term| term.is_special())
    }

    /// The UCUM defines "metric units" using four points. First:
    ///
    /// > Only metric unit atoms may be combined with a prefix.
    ///
    /// Second:
    ///
    /// > To be metric or not to be metric is a predicate assigned to each unit
    /// atom where that unit > atom is defined.
    ///
    /// Third:
    ///
    /// > All base units are metric. No non-metric unit can be part of the
    /// basis.
    ///
    /// Fourth:
    ///
    /// > A unit must be a quantity on a ratio scale in order to be metric.
    ///
    /// This library laxes the first rule and allows any atom/unit to use
    /// `Prefix`es. Also this method only returns `true` when *all* of its
    /// `Term`s are metric.
    ///
    fn is_metric(&self) -> bool {
        self.terms.iter().all(|term| term.is_metric())
    }

    fn is_arbitrary(&self) -> bool {
        self.terms.iter().all(|term| term.is_arbitrary())
    }

    /// This gives the scalar value of `self` in terms of `self`'s
    /// base-unit(s). It takes account for each of `self`'s `Term`'s
    /// `factor` and `exponent`.
    ///
    /// ```rust
    /// use wise_units::{UcumUnit, Unit};
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.scalar(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.scalar(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.000_1);
    ///
    /// // A "km/h" is 0.2777777777777778 meters/second.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.scalar(), 0.277_777_777_777_777_8);
    ///
    fn scalar(&self) -> BigRational {
        self.reduce_value(&*BR_1)
    }

    /// The scalar value of `self` in terms of `self`'s actual unit(s).
    ///
    /// ```rust
    /// use wise_units::{UcumUnit, Unit};
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.magnitude(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.000_1);
    ///
    /// // A "km/h" is 1000 meters/hour.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "m3" is 1 cubic meters.
    /// let unit = Unit::from_str("m3").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "L" is 1 liter.
    /// let unit = Unit::from_str("L").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "10m/5s" is 2 "meters per second".
    /// let unit = Unit::from_str("10m/5s").unwrap();
    /// assert_eq!(unit.magnitude(), 2.0);
    ///
    /// // A "10m/5s2" is 0.4 "meters per second".
    /// let unit = Unit::from_str("10m/5s2").unwrap();
    /// assert_eq!(unit.magnitude(), 0.4);
    ///
    fn magnitude(&self) -> BigRational {
        self.calculate_magnitude(self.scalar())
    }
}

#[cfg(test)]
mod tests {
    use num_help::{BR_1, BR_PI};
    use std::str::FromStr;
    use ucum_unit::UcumUnit;
    use unit::Unit;

    macro_rules! validate_scalar {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_eq!(unit.scalar(), $expected_value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $input_string, $expected_value);
            )+
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_eq!(unit.magnitude(), $expected_value);
            }
        };
    }

    macro_rules! validate_magnitudes {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_magnitude!($test_name, $input_string, $expected_value);
            )+
        };
    }

    #[test]
    fn validate_is_special() {
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_special());
    }

    validate_scalars!(
        validate_scalar_m, "m", BR_1.clone();
        validate_scalar_km, "km", big_rational_raw!(1000);
        validate_scalar_m_minus_1, "m-1", BR_1.clone();
        validate_scalar_10m, "10m", big_rational_raw!(10);
        validate_scalar_10km, "10km", big_rational_raw!(10_000);
        validate_scalar_10km_minus_1, "10km-1", big_rational_raw!(1, 10_000);
        validate_scalar_10km_minus_1_m2, "10km-1.m2", big_rational_raw!(1, 10_000);
        validate_scalar_km_slash_m2_dot_cm, "km/m2.cm", big_rational_raw!(100_000);
        validate_scalar_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", big_rational_raw!(1, 10);
        validate_scalar_m_slash_s2, "m/s2", BR_1.clone();
        validate_scalar_slash_1, "/1", BR_1.clone();
        validate_scalar_slash_m, "/m", BR_1.clone();
        validate_scalar_slash_annotation, "/{tot}", BR_1.clone();

        validate_scalar_liter, "l", big_rational_raw!(1, 1000);
        validate_scalar_liter_caps, "L", big_rational_raw!(1, 1000);
        validate_scalar_pi, "[pi]", BR_PI.clone();
        validate_scalar_ten_pi, "10[pi]", 10 * &*BR_PI;
        validate_scalar_hectare, "har", big_rational_raw!(10_000);
        validate_scalar_week, "wk", big_rational_raw!(604_800);
        validate_scalar_kilogram, "kg", big_rational_raw!(1000);
        validate_scalar_fahrenheit, "[degF]", big_rational_raw!(2_559_277_777_777_778, 10_000_000_000_000);
    );

    validate_magnitudes!(
        validate_magnitude_m, "m", BR_1.clone();
        validate_magnitude_km, "km", big_rational_raw!(1000);
        validate_magnitude_m_minus_1, "m-1", BR_1.clone();
        validate_magnitude_10m, "10m", big_rational_raw!(10);
        validate_magnitude_10km, "10km", big_rational_raw!(10_000);
        validate_magnitude_10km_minus_1, "10km-1", big_rational_raw!(1, 1000);
        validate_magnitude_10km_minus_1_m2, "10km-1.m2", big_rational_raw!(1, 1000);
        validate_magnitude_km_slash_m2_dot_cm, "km/m2.cm", big_rational_raw!(100_000);
        validate_magnitude_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", big_rational_raw!(1, 10);
        validate_magnitude_m_slash_s2, "m/s2", BR_1.clone();
        validate_magnitude_slash_1, "/1", BR_1.clone();
        validate_magnitude_slash_m, "/m", BR_1.clone();
        validate_magnitude_slash_annotation, "/{tot}", BR_1.clone();

        validate_magnitude_m2, "m2", BR_1.clone();
        validate_magnitude_m3, "m3", BR_1.clone();
        validate_magnitude_liter, "l", BR_1.clone();
        validate_magnitude_liter_caps, "L", BR_1.clone();
        validate_magnitude_8m_slash_4s, "8m/4s", big_rational_raw!(2);
        validate_magnitude_8m_slash_4s2, "8m/4s2", big_rational_raw!(1, 2);

        validate_magnitude_pi, "[pi]", BR_1.clone();
        validate_magnitude_ten_pi, "10[pi]", big_rational_raw!(10);

        // TODO: This doesn't parse (AGDEV-33099)
        // validate_magnitude_decare, "dar", 0.1;
        validate_magnitude_dekare, "daar", big_rational_raw!(10);
        validate_magnitude_hectare, "har", big_rational_raw!(100);
        validate_magnitude_kilare, "kar", big_rational_raw!(1000);

        validate_magnitude_week, "wk", BR_1.clone();
        validate_magnitude_kilogram, "kg", big_rational_raw!(1000);
        validate_magnitude_fahrenheit, "[degF]", BR_1.clone();
    );
}
