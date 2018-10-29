use crate::parser::Error;
use std::str::FromStr;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
impl FromStr for Unit {
    type Err = Error;

    #[inline]
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let terms = crate::parser::parse(expression)?;

        Ok(Self::from(terms))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::unit::Unit;

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }

    #[test]
    fn validate_annotation() {
        let unit = Unit::from_str("{foo}").unwrap();
        let term = unit.terms.first().unwrap();
        let annotation = &term.annotation;

        assert_eq!(annotation, &Some("foo".to_string()));
    }
}
