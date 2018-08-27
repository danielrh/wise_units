use measurement::Measurement;
use parser::Composable;
use ucum_unit::UcumUnit;

/// `Measurement`s are `PartialEq` if
///
/// a) their `Unit`s are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use wise_units::Measurement;
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement != other);
///
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
/// ```
///
impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        if !self.unit.is_compatible_with(&other.unit) {
            return false;
        }

        self.scalar() == other.scalar()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use unit::Unit;

    lazy_static! { static ref M: Unit = Unit::from_str("m").unwrap(); }
    lazy_static! { static ref KM: Unit = Unit::from_str("km").unwrap(); }
    lazy_static! { static ref HAR: Unit = Unit::from_str("har").unwrap(); }

    #[test]
    fn validate_eq_same_unit() {
        let m1 = Measurement::new(1, M.clone());
        let m2 = Measurement::new(1, M.clone());
        assert!(&m1 == &m2);

        let m2 = Measurement::new_try_value(1.1, M.clone()).unwrap();
        assert!(m1 != m2);
    }

    #[test]
    fn validate_eq_unit_with_prefix() {
        let m = Measurement::new(1000, M.clone());
        let km = Measurement::new(1, KM.clone());
        assert!(&m == &km);

        let km = Measurement::new_try_value(1.1, KM.clone()).unwrap();
        assert!(&m != &km);
    }

    #[test]
    fn validate_eq_unit_with_derived() {
        let m2 = Measurement::new_try_unit(10_000, "m2").unwrap();
        let har = Measurement::new(1, HAR.clone());
        assert!(m2 == har);

        let har = Measurement::new_try_value(1.1, HAR.clone()).unwrap();
        assert!(m2 != har);
    }

    #[test]
    fn validate_eq_incompatible_unit() {
        let m = Measurement::new(1, M.clone());
        let s = Measurement::new_try_unit(1, "s").unwrap();
        assert!(&m != &s);
    }
}
