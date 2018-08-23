use num_bigint::BigInt;
use num_help::{BR_0, BR_1, BR_E};
use num_rational::BigRational;

//-----------------------------------------------------------------------------
// ArbitraryLog
//-----------------------------------------------------------------------------
pub trait ArbitraryLog<T> {
    type Output;

    fn log(&self, base: T) -> Self::Output;
}

impl ArbitraryLog<usize> for BigRational {
    type Output = BigRational;

    fn log(&self, base: usize) -> Self::Output {
        if big_rational_raw!(base, 1) > self - &*BR_1 {
            let new_base = base / self;

            &*BR_1 + new_base.log(self)
        } else {
            BR_0.clone()
        }
    }
}

impl ArbitraryLog<BigInt> for BigRational {
    type Output = BigRational;

    fn log(&self, base: BigInt) -> Self::Output {
        if big_rational_raw!(base.clone(), 1) > self - &*BR_1 {
            let new_base = base / self;

            &*BR_1 + new_base.log(self)
        } else {
            BR_0.clone()
        }
    }
}

impl ArbitraryLog<BigRational> for BigRational {
    type Output = BigRational;

    fn log(&self, base: BigRational) -> Self::Output {
        if base > self - &*BR_1 {
            let new_base = base / self;

            &*BR_1 + new_base.log(self)
        } else {
            BR_0.clone()
        }
    }
}

impl<'a> ArbitraryLog<&'a BigRational> for BigRational {
    type Output = BigRational;

    fn log(&self, base: &'a BigRational) -> Self::Output {
        if base > &(self - &*BR_1) {
            let new_base = base / self;

            &*BR_1 + new_base.log(self)
        } else {
            BR_0.clone()
        }
    }
}

//-----------------------------------------------------------------------------
// NaturalLog
//-----------------------------------------------------------------------------
pub trait NaturalLog {
    type Output;

    fn ln(&self) -> Self::Output;
}

impl NaturalLog for BigRational {
    type Output = BigRational;

    #[inline]
    fn ln(&self) -> Self::Output {
        self.log(&*BR_E)
    }
}

//-----------------------------------------------------------------------------
// ArbitraryLog
//-----------------------------------------------------------------------------
pub trait Log2 {
    type Output;

    fn log2(&self) -> Self::Output;
}

impl Log2 for BigRational {
    type Output = BigRational;

    #[inline]
    fn log2(&self) -> Self::Output {
        self.log(2)
    }
}

//-----------------------------------------------------------------------------
// ArbitraryLog
//-----------------------------------------------------------------------------
pub trait Log10 {
    type Output;

    fn log10(&self) -> Self::Output;
}

impl Log10 for BigRational {
    type Output = BigRational;

    #[inline]
    fn log10(&self) -> Self::Output {
        self.log(10)
    }
}
