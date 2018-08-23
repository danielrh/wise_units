use std::num::ParseFloatError;

#[derive(Fail, Debug)]
pub enum NumberError {
    #[fail(display = "Unable to parse float: {}", float_str)]
    FloatParse {
        float_str: String,
        #[cause] parse_float_error: ParseFloatError
    },

    #[fail(display = "Unusable float value: {}", _0)]
    UnusableFloat(f64),

    #[fail(display = "Invalidate BigRational components: {} / {}", numer, denom)]
    InvalidBigRational{
        numer: f64,
        denom: f64,
    }
}

impl NumberError {
    pub(crate) fn from_parse_float_error(string: &str, e: ParseFloatError) -> Self {
        NumberError::FloatParse {
            float_str: string.to_string(),
            parse_float_error: e,
        }
    }
}
