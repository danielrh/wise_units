use convertible::Convertible;
use measurement::Measurement;
use num_rational::BigRational;
use parser::Error;
use std::ops::{Add, Div, Mul, Sub};

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
fn add_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = &lhs.value + &rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Add for Measurement {
    type Output = Result<Self, Error>;

    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    fn add(self, other: &'a Self) -> Self::Output {
        add_measurements(&self, other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: Measurement) -> Self::Output {
        add_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
fn sub_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = &lhs.value - &rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Sub for Measurement {
    type Output = Result<Self, Error>;

    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    fn sub(self, other: &'a Self) -> Self::Output {
        sub_measurements(&self, other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: Measurement) -> Self::Output {
        sub_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
fn mul_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = &lhs.value * &rhs.value;
    let new_unit = &lhs.unit * &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Mul for Measurement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        mul_measurements(&self, &other)
    }
}

impl<'a> Mul<&'a Measurement> for Measurement {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        mul_measurements(&self, other)
    }
}

impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a Measurement) -> Self::Output {
        mul_measurements(self, other)
    }
}

impl<'a> Mul<Measurement> for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: Measurement) -> Self::Output {
        mul_measurements(self, &other)
    }
}

/// Multiplies the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl<'a, T> Mul<T> for Measurement
where
    T: Mul<BigRational, Output=BigRational>
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let new_value: BigRational = other * self.value;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

impl<'a, T> Mul<T> for &'a Measurement
where
    T: Mul<&'a BigRational, Output=BigRational>
{
    type Output = Measurement;

    fn mul(self, other: T) -> Self::Output {
        let new_value: BigRational = other * &self.value;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
fn div_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let new_value = &lhs.value / &rhs.value;
    let new_unit = &lhs.unit / &rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Div for Measurement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        div_measurements(&self, &other)
    }
}

impl<'a> Div for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: &'a Measurement) -> Self::Output {
        div_measurements(self, other)
    }
}

impl<'a> Div<&'a Measurement> for Measurement {
    type Output = Self;

    fn div(self, other: &'a Self) -> Self::Output {
        div_measurements(&self, other)
    }
}

impl<'a> Div<Measurement> for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: Measurement) -> Self::Output {
        div_measurements(self, &other)
    }
}

/// Divides the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl<T> Div<T> for Measurement
where
    T: Clone + ::num_integer::Integer,
    // BigRational: Div<T, Output=BigRational>,
{
    type Output = Measurement;

    fn div(self, other: T) -> Self::Output {
        let new_value: BigRational = self.value / other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

impl<'a, T> Div<T> for &'a Measurement
where
    T: Clone + ::num_integer::Integer,
    // &'a BigRational: Div<T, Output=BigRational>
{
    type Output = Measurement;

    fn div(self, other: T) -> Self::Output {
        let new_value: BigRational = &self.value / other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

impl Div<f64> for Measurement {
    type Output = Result<Measurement, Error>;

    fn div(self, other: f64) -> Self::Output {
        let new_value: BigRational = (self.value / other)?;

        Ok(Measurement {
            value: new_value,
            unit: self.unit.clone(),
        })
    }
}
// impl<T> Div<T> for Measurement
// where
//     T: ::num_traits::Float,
//     BigRational: Div<T, Output=Option<BigRational>>,
// {
//     type Output = Result<Measurement, Error>;

//     fn div(self, other: T) -> Self::Output {
//         let new_value: BigRational = (self.value / other)?;

//         Ok(Measurement {
//             value: new_value,
//             unit: self.unit.clone(),
//         })
//     }
// }


#[cfg(test)]
mod tests {
    use measurement::Measurement;
    use parser::{Atom, Term};
    use std::str::FromStr;
    use unit::Unit;

    lazy_static! { static ref M: Unit = Unit::from_str("m").unwrap(); }

    mod add {
        use super::*;

        #[test]
        fn validate_add_owned() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(3, M.clone());

            assert_eq!((m1 + m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(3, M.clone());

            assert_eq!((&m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_owned_and_borrowed() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(3, M.clone());

            assert_eq!((m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed_and_owned() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(3, M.clone());

            assert_eq!((&m1 + m2).unwrap(), expected);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn validate_sub_owned() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(-1, M.clone());

            assert_eq!((m1 - m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(-1, M.clone());

            assert_eq!((&m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_owned_and_borrowed() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(-1, M.clone());

            assert_eq!((m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed_and_owned() {
            let m1 = Measurement::new(1, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let expected = Measurement::new(-1, M.clone());

            assert_eq!((&m1 - m2).unwrap(), expected);
        }
    }

    mod mul {
        use super::*;
        use std::ops::Mul;

        #[test]
        fn validate_mul_owned() {
            let m1 = Measurement::new(2, M.clone());
            let m2 = Measurement::new(3, M.clone());
            let r = m1 * m2;

            assert_eq!(r.value, big_rational_raw!(6));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_borrowed() {
            let m1 = Measurement::new(2, M.clone());
            let m2 = Measurement::new(3, M.clone());
            let r = &m1 * &m2;

            assert_eq!(r.value, big_rational_raw!(6));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_owned_and_borrowed() {
            let m1 = Measurement::new(2, M.clone());
            let m2 = Measurement::new(3, M.clone());
            let r = m1 * &m2;

            assert_eq!(r.value, big_rational_raw!(6));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_borrowed_and_owned() {
            let m1 = Measurement::new(2, M.clone());
            let m2 = Measurement::new(3, M.clone());
            let r = &m1 * m2;

            assert_eq!(r.value, big_rational_raw!(6));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);
            assert_eq!(terms[1], first_term);
        }

        #[test]
        fn validate_mul_f64() {
            let m = Measurement::new(10, M.clone());
            let expected = Measurement::new(200, M.clone());

            assert_eq!(m.mul(20), expected);
        }
    }

    mod div {
        use super::*;
        use std::ops::Div;

        #[test]
        fn validate_div_owned() {
            let m1 = Measurement::new(10, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let r = m1 / m2;

            assert_eq!(r.value, big_rational_raw!(5));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_borrowed() {
            let m1 = Measurement::new(10, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let r = &m1 / &m2;

            assert_eq!(r.value, big_rational_raw!(5));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_owned_and_borrowed() {
            let m1 = Measurement::new(10, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let r = m1 / &m2;

            assert_eq!(r.value, big_rational_raw!(5));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_borrowed_and_owned() {
            let m1 = Measurement::new(10, M.clone());
            let m2 = Measurement::new(2, M.clone());
            let r = &m1 / m2;

            assert_eq!(r.value, big_rational_raw!(5));

            let terms = r.unit.terms;
            assert_eq!(terms.len(), 2);

            let first_term = term!(Meter);
            assert_eq!(terms[0], first_term);

            let last_term = term!(Meter, exponent: -1);
            assert_eq!(terms[1], last_term);
        }

        #[test]
        fn validate_div_f64() {
            let m = Measurement::new(10, M.clone());
            let expected = Measurement::new(2, M.clone());

            assert_eq!(m.div(5.0).unwrap(), expected);
        }
    }
}
