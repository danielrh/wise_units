pub mod bigint_pow;
pub mod big_rational_pow;
pub mod logarithmic;
pub mod number_error;
pub mod to_float;
pub mod trigonometry;

pub use self::bigint_pow::*;
pub use self::big_rational_pow::*;
pub use self::logarithmic::*;
pub use self::number_error::NumberError;
pub use self::to_float::ToFloat;
pub use self::trigonometry::Trigonometry;

use num_bigint::BigInt;
use num_rational::BigRational;

lazy_static! { pub static ref BI_1: BigInt = BigInt::from(1); }
lazy_static! { pub static ref BI_0: BigInt = BigInt::from(0); }

lazy_static! { pub static ref BR_0: BigRational = big_rational_raw!(0, 1); }
lazy_static! { pub static ref BR_1: BigRational = big_rational_raw!(1, 1); }
lazy_static! { pub static ref BR_E: BigRational = BigRational::from_float(::std::f64::consts::E).unwrap(); }
lazy_static! { pub static ref BR_PI: BigRational = BigRational::from_float(::std::f64::consts::PI).unwrap(); }
lazy_static! { pub static ref BR_EPSILON: BigRational = BigRational::from_float(::std::f64::EPSILON).unwrap(); }
