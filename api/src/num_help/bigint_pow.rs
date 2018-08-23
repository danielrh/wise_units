use num_bigint::BigInt;
use super::{BI_1, BI_0};
use std::cmp::Ordering;

pub trait BigIntPow<E> {
    fn pow(&self, exponent: E) -> BigInt;
}

impl BigIntPow<i32> for BigInt {
    fn pow(&self, exponent: i32) -> Self {
        match exponent.cmp(&0) {
            Ordering::Greater => {
                let next: BigInt = self.pow(exponent - 1);
                self * next
            }
            Ordering::Less => &*BI_1 / self.pow(-exponent),
            Ordering::Equal => BI_0.clone(),
        }
    }
}
