use super::Unit;
use crate::invert::{Invert, ToInverse};

impl Invert for Unit {
    #[inline]
    fn invert(self) -> Self {
        Unit {
            terms: self.terms.invert(),
        }
    }
}

impl ToInverse for Unit {
    type Output = Self;

    #[inline]
    fn to_inverse(&self) -> Self::Output {
        Self::new(self.terms.to_inverse())
    }
}

#[cfg(test)]
mod tests {
    use super::Unit;
    use std::str::FromStr;

    lazy_static::lazy_static! {
        static ref PER_METER: Unit = Unit::from_str("m-1").unwrap();
    }

    mod invert {
        use super::*;
        use crate::invert::Invert;

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            assert_eq!(unit.invert(), PER_METER.clone());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            assert_eq!(unit.invert(), PER_METER.clone());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            assert_eq!(PER_METER.clone().invert(), Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            assert_eq!(unit.invert(), Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            assert_eq!(unit.invert(), Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod to_inverse {
        use super::*;
        use crate::{invert::ToInverse, Unit};

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, PER_METER.clone());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, PER_METER.clone());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            assert_eq!(PER_METER.clone().to_inverse(), Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod into_inverse {
        use super::*;
        use crate::invert::IntoInverse;

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, PER_METER.clone());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, PER_METER.clone());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            assert_eq!(
                PER_METER.clone().into_inverse(),
                Unit::from_str("m").unwrap()
            );
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }
}
