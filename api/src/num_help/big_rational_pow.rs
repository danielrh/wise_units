use num_rational::BigRational;
use std::cmp::Ordering;
use super::{BR_0, BR_1};

pub trait BigRationalPow<E> {
    fn pow(&self, exponent: E) -> BigRational;
}

impl BigRationalPow<i32> for BigRational {
    #[inline]
    fn pow(&self, exponent: i32) -> BigRational {
        match exponent.cmp(&0) {
            Ordering::Greater => {
                let next: BigRational = self.pow(exponent - 1);
                self * next
            }
            Ordering::Less => &*BR_1 / self.pow(-exponent),
            Ordering::Equal => BR_1.clone(),
        }
    }
}

impl BigRationalPow<BigRational> for BigRational {
    #[inline]
    fn pow(&self, exponent: BigRational) -> BigRational {
        match exponent.cmp(&BR_0) {
            Ordering::Greater => {
                let next: BigRational = self.pow(exponent - &*BR_1);
                self * next
            }
            Ordering::Less => &*BR_1 / self.pow(-exponent),
            Ordering::Equal => BR_1.clone(),
        }
    }
}

impl<'a> BigRationalPow<&'a BigRational> for BigRational {
    #[inline]
    fn pow(&self, exponent: &'a BigRational) -> BigRational {
        match exponent.cmp(&BR_0) {
            Ordering::Greater => {
                let next: BigRational = self.pow(exponent - &*BR_1);
                self * next
            }
            Ordering::Less => &*BR_1 / self.pow(-exponent),
            Ordering::Equal => BR_1.clone(),
        }
    }
}

pub trait BigRationalSqrt {
    fn sqrt(&self) -> BigRational;
}

impl BigRationalSqrt for BigRational {
    #[inline]
    fn sqrt(&self) -> BigRational {
        self.pow(big_rational_raw!(1, 2))
    }
}
