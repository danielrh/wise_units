use num_rational::BigRational;
use parser::ucum_symbol::UcumSymbol;
use parser::{definition::Definition, Classification};
use std::fmt;
use unit::Unit;

lazy_static! { static ref YOTTA:  BigRational = big_rational_raw!(bytes 1_000_000_000_000_000_000_000_000); }
lazy_static! { static ref ZETTA:  BigRational = big_rational_raw!(bytes 1_000_000_000_000_000_000_000); }
lazy_static! { static ref EXA:    BigRational = big_rational_raw!(1_000_000_000_000_000_000u64); }
lazy_static! { static ref PETA:   BigRational = big_rational_raw!(1_000_000_000_000_000u64); }
lazy_static! { static ref TERA:   BigRational = big_rational_raw!(1_000_000_000_000u64); }
lazy_static! { static ref GIGA:   BigRational = big_rational_raw!(1_000_000_000); }
lazy_static! { static ref MEGA:   BigRational = big_rational_raw!(1_000_000); }
lazy_static! { static ref KILO:   BigRational = big_rational_raw!(1_000); }
lazy_static! { static ref HECTO:  BigRational = big_rational_raw!(100); }
lazy_static! { static ref DEKA:   BigRational = big_rational_raw!(10); }

lazy_static! { static ref TEBI:   BigRational = big_rational_raw!(1_099_511_627_776u64); }
lazy_static! { static ref GIBI:   BigRational = big_rational_raw!(1_073_741_824); }
lazy_static! { static ref MEBI:   BigRational = big_rational_raw!(1_048_576); }
lazy_static! { static ref KIBI:   BigRational = big_rational_raw!(1024); }

lazy_static! { static ref DECI:   BigRational = big_rational_raw!(1, 100); }
lazy_static! { static ref CENTI:  BigRational = big_rational_raw!(1, 100); }
lazy_static! { static ref MILLI:  BigRational = big_rational_raw!(1, 1000); }
lazy_static! { static ref MICRO:  BigRational = big_rational_raw!(1, 1_000_000); }
lazy_static! { static ref NANO:   BigRational = big_rational_raw!(1, 1_000_000_000); }
lazy_static! { static ref PICO:   BigRational = big_rational_raw!(1, 1_000_000_000_000u64); }
lazy_static! { static ref FEMTO:  BigRational = big_rational_raw!(1, 1_000_000_000_000_000u64); }
lazy_static! { static ref ATTO:   BigRational = big_rational_raw!(1, 1_000_000_000_000_000_000u64); }
lazy_static! { static ref ZEPTO:  BigRational = big_rational_raw!(1, bytes 1_000_000_000_000_000_000_000); }
lazy_static! { static ref YOCTO:  BigRational = big_rational_raw!(1, bytes 1_000_000_000_000_000_000_000_000); }

/// A `Prefix` is essentially a multiplier for an `Atom` within a `Term`; ex.
/// the "c" in "cm" modifies meter by 0.01. The UCUM spec says these should
/// only pertain to metric units, but that rule is not adhered to in
/// `wise_units`.
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefix {
    Atto,
    Centi,
    Deci,
    Deka,
    Exa,
    Femto,
    Gibi,
    Giga,
    Hecto,
    Kibi,
    Kilo,
    Mebi,
    Mega,
    Micro,
    Milli,
    Nano,
    Peta,
    Pico,
    Tebi,
    Tera,
    Yocto,
    Yotta,
    Zepto,
    Zetta,
}

impl UcumSymbol for Prefix {
    fn classification(&self) -> Classification {
        Classification::Si
    }

    fn names(&self) -> Vec<&'static str> {
        match *self {
            Prefix::Atto  => vec!["atto"],
            Prefix::Centi => vec!["centi"],
            Prefix::Deci  => vec!["deci"],
            Prefix::Deka  => vec!["deka"],
            Prefix::Exa   => vec!["exa"],
            Prefix::Femto => vec!["femto"],
            Prefix::Gibi  => vec!["gibi"],
            Prefix::Giga  => vec!["giga"],
            Prefix::Hecto => vec!["hecto"],
            Prefix::Kibi  => vec!["kibi"],
            Prefix::Kilo  => vec!["kilo"],
            Prefix::Mebi  => vec!["mebi"],
            Prefix::Mega  => vec!["mega"],
            Prefix::Micro => vec!["micro"],
            Prefix::Milli => vec!["milli"],
            Prefix::Nano  => vec!["nano"],
            Prefix::Peta  => vec!["peta"],
            Prefix::Pico  => vec!["pico"],
            Prefix::Tebi  => vec!["tebi"],
            Prefix::Tera  => vec!["tera"],
            Prefix::Yocto => vec!["yocto"],
            Prefix::Yotta => vec!["yotta"],
            Prefix::Zepto => vec!["zepto"],
            Prefix::Zetta => vec!["zetta"],
        }
    }

    fn primary_code(&self) -> &'static str {
        match *self {
            Prefix::Atto  => "a",
            Prefix::Centi => "c",
            Prefix::Deci  => "d",
            Prefix::Deka  => "da",
            Prefix::Exa   => "E",
            Prefix::Femto => "f",
            Prefix::Gibi  => "Gi",
            Prefix::Giga  => "G",
            Prefix::Hecto => "h",
            Prefix::Kibi  => "Ki",
            Prefix::Kilo  => "k",
            Prefix::Mebi  => "Mi",
            Prefix::Mega  => "M",
            Prefix::Micro => "u",
            Prefix::Milli => "m",
            Prefix::Nano  => "n",
            Prefix::Peta  => "P",
            Prefix::Pico  => "p",
            Prefix::Tebi  => "Ti",
            Prefix::Tera  => "T",
            Prefix::Yocto => "y",
            Prefix::Yotta => "Y",
            Prefix::Zepto => "z",
            Prefix::Zetta => "Z",
        }
    }

    fn print_symbol(&self) -> Option<&'static str> {
        match *self {
            Prefix::Micro => Some("μ"),
            _ => Some(self.primary_code()),
        }
    }

    fn secondary_code(&self) -> Option<&'static str> {
        let code = match *self {
            Prefix::Atto  => "A",
            Prefix::Centi => "C",
            Prefix::Deci  => "D",
            Prefix::Deka  => "DA",
            Prefix::Exa   => "EX",
            Prefix::Femto => "F",
            Prefix::Gibi  => "GIB",
            Prefix::Giga  => "GA",
            Prefix::Hecto => "H",
            Prefix::Kibi  => "KIB",
            Prefix::Kilo  => "K",
            Prefix::Mebi  => "MIB",
            Prefix::Mega  => "MA",
            Prefix::Micro => "U",
            Prefix::Milli => "M",
            Prefix::Nano  => "N",
            Prefix::Peta  => "PT",
            Prefix::Pico  => "P",
            Prefix::Tebi  => "TIB",
            Prefix::Tera  => "TR",
            Prefix::Yocto => "YO",
            Prefix::Yotta => "YA",
            Prefix::Zepto => "ZO",
            Prefix::Zetta => "ZA",
        };

        Some(code)
    }

    /// The numeric value that each `Prefix` represents.
    ///
    fn definition_value(&self) -> BigRational {
        match *self {
            Prefix::Atto  => ATTO.clone(),
            Prefix::Centi => CENTI.clone(),
            Prefix::Deci  => DECI.clone(),
            Prefix::Deka  => DEKA.clone(),
            Prefix::Exa   => EXA.clone(),
            Prefix::Femto => FEMTO.clone(),
            Prefix::Gibi  => GIBI.clone(),
            Prefix::Giga  => GIGA.clone(),
            Prefix::Hecto => HECTO.clone(),
            Prefix::Kibi  => KIBI.clone(),
            Prefix::Kilo  => KILO.clone(),
            Prefix::Mebi  => MEBI.clone(),
            Prefix::Mega  => MEGA.clone(),
            Prefix::Micro => MICRO.clone(),
            Prefix::Milli => MILLI.clone(),
            Prefix::Nano  => NANO.clone(),
            Prefix::Peta  => PETA.clone(),
            Prefix::Pico  => PICO.clone(),
            Prefix::Tebi  => TEBI.clone(),
            Prefix::Tera  => TERA.clone(),
            Prefix::Yocto => YOCTO.clone(),
            Prefix::Yotta => YOTTA.clone(),
            Prefix::Zepto => ZEPTO.clone(),
            Prefix::Zetta => ZETTA.clone(),
        }
    }

    fn definition_unit(&self) -> Unit {
        let definition = Definition::default();

        Unit::from(definition.terms().to_vec())
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::ucum_symbol::UcumSymbol;

    macro_rules! validate_value {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let prefix = Prefix::$variant;
                assert_eq!(prefix.definition_value(), $value);
            }
        };
    }

    macro_rules! validate_values {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_value!($test_name, $variant, $value);
            )+
        };
    }

    validate_values!(
        validate_value_atto,  Atto,  ATTO.clone();
        validate_value_centi, Centi, CENTI.clone();
        validate_value_deci,  Deci,  DECI.clone();
        validate_value_deka,  Deka,  DEKA.clone();
        validate_value_exa,   Exa,   EXA.clone();
        validate_value_femto, Femto, FEMTO.clone();
        validate_value_gibi,  Gibi,  GIBI.clone();
        validate_value_giga,  Giga,  GIGA.clone();
        validate_value_hecto, Hecto, HECTO.clone();
        validate_value_kibi,  Kibi,  KIBI.clone();
        validate_value_kilo,  Kilo,  KILO.clone();
        validate_value_mebi,  Mebi,  MEBI.clone();
        validate_value_mega,  Mega,  MEGA.clone();
        validate_value_micro, Micro, MICRO.clone();
        validate_value_milli, Milli, MILLI.clone();
        validate_value_nano,  Nano,  NANO.clone();
        validate_value_peta,  Peta,  PETA.clone();
        validate_value_pico,  Pico,  PICO.clone();
        validate_value_tebi,  Tebi,  TEBI.clone();
        validate_value_tera,  Tera,  TERA.clone();
        validate_value_yocto, Yocto, YOCTO.clone();
        validate_value_yotta, Yotta, YOTTA.clone();
        validate_value_zepto, Zepto, ZEPTO.clone();
        validate_value_zetta, Zetta, ZETTA.clone();
    );

    #[test]
    fn validate_display() {
        let prefix = Prefix::Kilo;
        assert_eq!(&prefix.to_string(), "k")
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Prefix;
        use serde_json;

        #[test]
        fn validate_serialization() {
            let j = serde_json::to_string(&Prefix::Kilo)
                .expect("Couldn't convert Prefix to JSON String");

            assert_eq!("\"Kilo\"", j);
        }

        #[test]
        fn validate_deserialization() {
            let k =
                serde_json::from_str("\"Kilo\"").expect("Couldn't convert JSON String to Prefix");

            assert_eq!(Prefix::Kilo, k);
        }
    }
}
