use num_bigint::BigInt;
use num_help::NumberError;
use num_rational::BigRational;
use std::str::FromStr;

pub trait ToFloat {
    fn to_float(&self) -> Result<f64, NumberError>;
}

impl ToFloat for BigInt {
    fn to_float(&self) -> Result<f64, NumberError> {
        let self_str = self.to_str_radix(10);
        let self_float = f64::from_str(&self_str)
            .map_err(|e| NumberError::from_parse_float_error(&self_str, e))?;

        if self_float.is_normal() || self_float == 0.0 {
            Ok(self_float)
        } else {
            Err(NumberError::UnusableFloat(self_float))
        }
    }
}

impl ToFloat for BigRational {
    fn to_float(&self) -> Result<f64, NumberError> {
        let self_numer = self.numer().to_float()?;
        let self_denom = self.denom().to_float()?;
        let self_float = self_numer / self_denom;

        if self_float.is_normal() || self_float == 0.0 {
            Ok(self_float)
        } else {
            Err(NumberError::UnusableFloat(self_float))
        }
    }
}
