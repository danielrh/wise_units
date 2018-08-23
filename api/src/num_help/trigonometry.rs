use num_rational::BigRational;
use num_help::ToFloat;

pub trait Trigonometry {
    type Output;

    fn sin(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn atan(self) -> Self::Output;
}

// TODO: These should all have native implementations instead of relying on f64 methods.
impl Trigonometry for BigRational {
    type Output = BigRational;

    fn sin(self) -> Self::Output {
        let float = self.to_float().expect("Unable to convert to f64");
        let sin = float.sin();

        BigRational::from_float(sin).expect("Unable to convert from f64")
    }

    fn tan(self) -> Self::Output {
        let float = self.to_float().expect("Unable to convert to f64");
        let tan = float.tan();

        BigRational::from_float(tan).expect("Unable to convert from f64")
    }

    fn atan(self) -> Self::Output {
        let float = self.to_float().expect("Unable to convert to f64");
        let atan = float.atan();

        BigRational::from_float(atan).expect("Unable to convert from f64")
    }
}
