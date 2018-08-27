pub mod convertible;
pub mod field_eq;
pub mod ops;
pub mod partial_eq;
pub mod partial_ord;
pub mod reducible;
pub mod ucum_unit;

use num_help::{BR_1};
use num_rational::BigRational;
use parser::Error;
use reducible::Reducible;
use std::str::FromStr;
use ucum_unit::UcumUnit;
use unit::Unit;

pub trait IntoMeasurementValue {
    fn into_value(self) -> BigRational;
}

impl<V: Into<BigRational>> IntoMeasurementValue for V {
    fn into_value(self) -> BigRational {
        self.into()
    }
}

// impl<V: Into<BigInt>> IntoMeasurementValue for V {
//     fn into_value(self) -> BigRational {
//         let numer = self.into();

//         BigRational::from(numer)
//     }
// }

pub trait TryIntoMeasurementValue {
    fn try_into_value(self) -> Result<BigRational, Error>;
}

impl TryIntoMeasurementValue for f64 {
    fn try_into_value(self) -> Result<BigRational, Error> {
        // TODO: handle error
        let value = BigRational::from_float(self).expect("valid float");

        Ok(value)
    }
}

pub trait IntoMeasurementUnit {
    fn into_unit(self) -> Unit;
}

impl<U: Into<Unit>> IntoMeasurementUnit for U {
    fn into_unit(self) -> Unit {
        self.into()
    }
}

pub trait TryIntoMeasurementUnit {
    fn try_into_unit(self) -> Result<Unit, Error>;
}

impl<'a> TryIntoMeasurementUnit for &'a str {
    fn try_into_unit(self) -> Result<Unit, Error> {
        Unit::from_str(self)
    }
}

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::{Convertible, Measurement};
///
/// let one_km = Measurement::new(1.0, "km").unwrap();
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// assert!(in_meters.value == 1000.0);
/// ```
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Measurement {
    pub value: BigRational,
    pub unit: Unit,
}

impl Measurement {
    pub fn new<V: IntoMeasurementValue, U: IntoMeasurementUnit>(value: V, unit: U) -> Measurement {
        let value = value.into_value();
        let unit = unit.into_unit();

        Measurement { value, unit }
    }

    pub fn try_new<V: TryIntoMeasurementValue, U: TryIntoMeasurementUnit>(value: V, unit: U) -> Result<Measurement, Error> {
        // TODO: Handle error
        let value = value.try_into_value().expect("TODO: HANDLE ME!");
        let unit = unit.try_into_unit()?;

        Ok(Measurement { value, unit })
    }

    pub fn new_try_unit<V: IntoMeasurementValue, U: TryIntoMeasurementUnit>(value: V, unit: U) -> Result<Measurement, Error> {
        let value = value.into_value();
        let unit = unit.try_into_unit()?;

        Ok(Measurement { value, unit })
    }

    pub fn new_try_value<V: TryIntoMeasurementValue, U: IntoMeasurementUnit>(value: V, unit: U) -> Result<Measurement, Error> {
        // TODO: Handle error
        let value = value.try_into_value().expect("TODO: HANDLE ME!");
        let unit = unit.into_unit();

        Ok(Measurement { value, unit })
    }

    /// The value of the `Measurement` in terms of `other_unit`. Only used for
    /// converting, and does not check the compatibility of units.
    ///
    fn converted_scalar(&self, other_unit: &Unit) -> BigRational {
        if self.is_special() && other_unit.is_special() {
            let ts = self.unit.reduce_value(&self.value);
            other_unit.calculate_magnitude(ts)
        } else if self.is_special() {
            self.unit.reduce_value(&self.value)
        } else if other_unit.is_special() {
            other_unit.calculate_magnitude(&self.value)
        } else {
            self.scalar() / other_unit.reduce_value(BR_1.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use num_help::BR_1;
    use std::str::FromStr;
    use super::super::parser::{Atom, Term};
    use super::*;
    use unit::Unit;

    lazy_static! {
        static ref METER: Unit = Unit::from_str("m").unwrap();
    }

    #[test]
    fn validate_new() {
        let m = Measurement::new(BR_1.clone(), METER.clone());

        let expected_unit = Unit {
            terms: vec![term!(Meter)],
        };

        assert_eq!(m.value, BR_1.clone());
        assert_eq!(m.unit, expected_unit);
    }

    #[test]
    fn validate_converted_scalar() {
        // No special units
        let m = Measurement::new(BR_1.clone(), METER.clone());
        let unit = METER.clone();
        assert_eq!(m.converted_scalar(&unit), BR_1.clone());

        let m = Measurement::new(BR_1.clone(), METER.clone());
        let unit = Unit::from_str("km").unwrap();
        assert_eq!(m.converted_scalar(&unit), big_rational_raw!(1, 1000));

        let m = Measurement::new(1000, METER.clone());
        let unit = Unit::from_str("km").unwrap();
        assert_eq!(m.converted_scalar(&unit), BR_1.clone());

        // Measurement unit is not special, but other_unit is
        let m = Measurement::new_try_unit(BR_1.clone(), "K").unwrap();
        let unit = Unit::from_str("Cel").unwrap();
        assert_eq!(m.converted_scalar(&unit), big_rational_raw!(-27_215, 100));

        // Measurement unit is special, but other_unit is not
        let m = Measurement::new_try_unit(BR_1.clone(), "Cel").unwrap();
        let unit = Unit::from_str("K").unwrap();
        assert_eq!(m.converted_scalar(&unit), big_rational_raw!(27_415, 100));

        // Measurement unit and other_unit are special
        let m = Measurement::new_try_unit(BR_1.clone(), "Cel").unwrap();
        let unit = Unit::from_str("[degF]").unwrap();
        assert_eq!(m.converted_scalar(&unit), big_rational_raw!(338, 10));
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Measurement;
        use parser::{Atom, Prefix, Term};
        use serde_json;
        use unit::Unit;

        #[test]
        fn validate_serialization_empty_terms() {
            let unit = Unit { terms: vec![] };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };
            let expected_json = r#"{"value":123.4,"unit":{"terms":[]}}"#;

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_serialization_full_terms() {
            let expected_json = r#"{
                "value":123.4,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": null,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#.replace("\n", "")
                .replace(" ", "");

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"value":1.0, "unit":{"terms": []}}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let unit = Unit { terms: vec![] };
            let expected_measurement = Measurement {
                value: 1.0,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }

        #[test]
        fn validate_deserialization_full_terms() {
            let json = r#"{
                "value":432.1,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": 1,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let expected_measurement = Measurement {
                value: 432.1,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }
    }
}
