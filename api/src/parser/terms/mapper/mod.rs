// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
pub(self) mod annotatable;
pub(self) mod ast_term;
pub(self) mod basic_component;
pub(self) mod component;
pub(self) mod finishable;
pub(self) mod main_term;
pub(self) mod simple_unit;

use self::annotatable::Annotatable;
use self::ast_term::AstTerm;
use self::basic_component::BasicComponent;
use self::component::Component;
use self::finishable::Finishable;
use self::main_term::MainTerm;
use self::simple_unit::SimpleUnit;
use crate::invert::Invert;
use crate::parser::atom::Atom;
use crate::parser::error::Error;
use crate::parser::prefix::Prefix;
// use crate::parser::symbols::mapper as symbol_mapper;
// use crate::parser::symbols::symbol_parser::Rule as SymbolRule;
// use crate::parser::symbols::symbol_parser::SymbolParser;
use crate::parser::term::Term;
use crate::parser::terms::term_parser::Rule;
use pest::iterators::{Pair, Pairs};
// use pest::Parser;

pub(crate) fn map(mut pairs: Pairs<'_, Rule>) -> Result<Vec<Term>, Error> {
    fn visit_pairs(pair: Pair<'_, Rule>) -> Result<Vec<Term>, Error> {
        let main_term = match pair.as_rule() {
            Rule::main_term => visit_main_term(pair)?,
            _ => {
                let e = Error::UnableToParse {
                    expression: pair.as_str().to_string(),
                };
                return Err(e);
            }
        };

        let mut terms: Vec<Term> = main_term.into();
        terms.shrink_to_fit();

        Ok(terms)
    }

    match pairs.next() {
        Some(pair) => Ok(visit_pairs(pair)?),
        None => Ok(vec![]),
    }
}

fn visit_digits(pair: Pair<'_, Rule>) -> Result<i32, Error> {
    let span = pair.into_span();
    let string = span.as_str();

    string.parse::<i32>().map_err(|e| Error::UnableToParse {
        expression: e.to_string(),
    })
}

fn visit_exponent(pair: Pair<'_, Rule>) -> Result<Option<i32>, Error> {
    let mut e: Option<i32> = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::sign => {
                let span = inner_pair.into_span();
                let string = span.as_str();

                match string {
                    "+" => {}
                    "-" => {
                        e = match e {
                            Some(exponent) => Some(-exponent),
                            None => Some(-1),
                        };
                    }
                    _ => {
                        let error = Error::UnableToParse {
                            expression: string.to_string(),
                        };
                        return Err(error);
                    }
                }
            }
            Rule::digits => {
                let new_digits = visit_digits(inner_pair)?;

                match e {
                    Some(exponent) => {
                        e = Some(exponent * new_digits);
                    }
                    None => e = Some(new_digits),
                }
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(e)
}

fn visit_simple_unit(pair: Pair<'_, Rule>) -> Result<SimpleUnit, Error> {
    let mut prefix: Option<Prefix> = None;
    let mut atom: Option<Atom> = None;

    if pair.as_str() == "1" {
        return Ok(SimpleUnit { prefix, atom });
    }

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::prefix_symbol => {
                prefix = Some(visit_prefix_symbol(inner_pair)?);
            }
            Rule::atom_symbol => {
                atom = Some(visit_atom_symbol(inner_pair)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(SimpleUnit { prefix, atom })
}

fn visit_prefix_symbol(pair: Pair<'_, Rule>) -> Result<Prefix, Error> {
    let prefix = match pair.as_str() {
        "a" => Prefix::Atto,
        "c" => Prefix::Centi,
        "d" => Prefix::Deci,
        "da" => Prefix::Deka,
        "E" => Prefix::Exa,
        "f" => Prefix::Femto,
        "Gi" => Prefix::Gibi,
        "G" => Prefix::Giga,
        "h" => Prefix::Hecto,
        "k" => Prefix::Kilo,
        "Mi" => Prefix::Mebi,
        "M" => Prefix::Mega,
        "u" => Prefix::Micro,
        "m" => Prefix::Milli,
        "n" => Prefix::Nano,
        "P" => Prefix::Peta,
        "Ti" => Prefix::Tebi,
        "T" => Prefix::Tera,
        "y" => Prefix::Yocto,
        "Y" => Prefix::Yotta,
        "z" => Prefix::Zepto,
        "Z" => Prefix::Zetta,
        _ => {
            match visit_secondary_prefix_symbol(pair.as_str()) {
                Some(p) => p,
                None => return Err(Error::UnknownUnitString(pair.as_str().to_string()))
            }
        },
    };

    Ok(prefix)
}

fn visit_secondary_prefix_symbol(prefix_str: &str) -> Option<Prefix> {
    let prefix = match prefix_str {
        "A" => Prefix::Atto,
        "C" => Prefix::Centi,
        "D" => Prefix::Deci,
        "DA" => Prefix::Deka,
        "EX" => Prefix::Exa,
        "F" => Prefix::Femto,
        "GIB" => Prefix::Gibi,
        "GA" => Prefix::Giga,
        "H" => Prefix::Hecto,
        "K" => Prefix::Kilo,
        "MIB" => Prefix::Mebi,
        "MA" => Prefix::Mega,
        "U" => Prefix::Micro,
        "M" => Prefix::Milli,
        "N" => Prefix::Nano,
        "PT" => Prefix::Peta,
        "TIB" => Prefix::Tebi,
        "TR" => Prefix::Tera,
        "YO" => Prefix::Yocto,
        "YA" => Prefix::Yotta,
        "ZO" => Prefix::Zepto,
        "ZA" => Prefix::Zetta,
        _ => return None
    };

    Some(prefix)
}

fn visit_atom_symbol(pair: Pair<'_, Rule>) -> Result<Atom, Error> {
    let pair_str = pair.into_span();

    let atom = match pair_str.as_str() {
        // Base units first.
        "cd" => Atom::Candela,
        "C"  => Atom::Coulomb,
        "g"           => Atom::Gram,
        "K"           => Atom::Kelvin,
        "m"=> Atom::Meter,
        "rad"         => Atom::Radian,
        "s"           => Atom::Second,

        // Derived units last.
        "[acr_br]" => Atom::AcreBritish,
        "[acr_us]" => Atom::AcreUS,
        "A"                                => Atom::Ampere,
        "ar"       => Atom::Are,
        "AU"       => Atom::AstronomicUnit,
        "u"        => Atom::UnifiedAtomicMassUnit,
        "bar"      => Atom::Bar,
        "[bbl_us]" => Atom::BarrelUS,
        "Bq"       => Atom::Becquerel,
        "Bi"       => Atom::Biot,
        "[bf_i]"   => Atom::BoardFootInternational,
        "[k]"      => Atom::BoltzmannConstant,
        "[bu_br]"  => Atom::BushelBritish,
        "[bu_us]"  => Atom::BushelUS,
        "[cml_i]"  => Atom::CircularMilInternational,
        "[cr_i]"   => Atom::CordInternational,
        "[crd_us]" => Atom::CordUS,
        "[cft_i]"  => Atom::CubicFootInternational,
        "[cin_i]"  => Atom::CubicInchInternational,
        "[cyd_i]"  => Atom::CubicYardInternational,
        "[cup_us]" => Atom::CupUS,
        "Ci"       => Atom::Curie,

        "d"        => Atom::Day,
        "Cel"      => Atom::DegreeCelsius,
        "[degF]"   => Atom::DegreeFahrenheit,
        "'"        => Atom::MinuteAngle,
        "''"       => Atom::Degree,
        "[degRe]"  => Atom::DegreeReaumur,
        "deg"      => Atom::Degree,
        "[dr_av]"  => Atom::DramAvoirdupois,
        "[dpt_us]" => Atom::DryPintUS,
        "[dqt_us]" => Atom::DryQuartUS,
        "dyn"      => Atom::Dyne,
        "eV"       => Atom::Electronvolt,
        "[m_e]"    => Atom::ElectronMass,
        "[e]"      => Atom::ElementaryCharge,
        "eq"       => Atom::Equivalents,
        "F"        => Atom::Farad,
        "[fth_br]" => Atom::FathomBritish,
        "[fth_i]"  => Atom::FathomInternational,
        "[fth_us]" => Atom::FathomUS,
        "[fdr_br]" => Atom::FluidDramBritish,
        "[fdr_us]" => Atom::FluidDramUS,
        "[foz_br]" => Atom::FluidOunceBritish,
        "[foz_us]" => Atom::FluidOunceUS,
        "[ft_br]"  => Atom::FootBritish,
        "[ft_i]"   => Atom::FootInternational,
        "[ft_us]"  => Atom::FootUS,
        "[fur_us]" => Atom::FurlongUS,

        "Gal"      => Atom::Gal,
        "[gal_br]" => Atom::GallonBritish,
        "G"        => Atom::Gauss,
        "Gb"       => Atom::Gilbert,
        "[gil_br]" => Atom::GillBritish,
        "[gil_us]" => Atom::GillUS,
        "gon"      => Atom::Gon,
        "gf"       => Atom::GramForce,
        "g%"       => Atom::GramPercent,
        "[gr]"     => Atom::Grain,
        "Gy"       => Atom::Gray,
        "[ch_br]"  => Atom::GuntersChainBritish,
        "[ch_us]"  => Atom::GuntersChainUS,
        "[hd_i]"   => Atom::HandInternational,
        "Hz"       => Atom::Hertz,
        "H"        => Atom::Henry,
        "[gal_wi]" => Atom::HistoricalWinchesterGallon,
        "[HP]"     => Atom::Horsepower,
        "h"        => Atom::Hour,
        "[in_br]"  => Atom::InchBritish,
        "[in_i]"   => Atom::InchInternational,
        "[in_us]"  => Atom::InchUS,

        "J"        => Atom::Joule,
        "Ky"       => Atom::Kayser,
        "[kn_br]"  => Atom::KnotBritish,
        "[kn_i]"   => Atom::KnotInternational,
        "Lmb"      => Atom::Lambert,
        "[lcwt_av]"=> Atom::LongHunderdweightAvoirdupois,
        "[lton_av]"=> Atom::LongTonAvoirdupois,
        "[ly]"     => Atom::LightYear,
        "[lk_br]"  => Atom::LinkForGuntersChainBritish,
        "[lk_us]"  => Atom::LinkForGuntersChainUS,
        "[rlk_us]" => Atom::LinkForRamdensChainUS,
        "l"        => Atom::Liter,
        "lm"       => Atom::Lumen,
        "lx"       => Atom::Lux,

        "Mx"       => Atom::Maxwell,
        "mo_g"     => Atom::MeanGregorianMonth,
        "a_g"      => Atom::MeanGregorianYear,
        "mo_j"     => Atom::MeanJulianMonth,
        "a_j"      => Atom::MeanJulianYear,
        "[cup_m]"  => Atom::MetricCup,
        "[foz_m]"  => Atom::MetricFluidOunce,
        "[tbs_m]"  => Atom::MetricTablespoon,
        "[tsp_m]"  => Atom::MetricTeaspoon,
        "[mil_i]"  => Atom::MilInternational,
        "[mil_us]" => Atom::MilUS,
        "[mi_br]"  => Atom::MileBritish,
        "[mi_i]"   => Atom::MileInternational,
        "[mi_us]"  => Atom::MileUS,
        "[min_br]" => Atom::MinimBritish,
        "[min_us]" => Atom::MinimUS,
        "mol"      => Atom::Mole,
        "mo"       => Atom::Month,
        "[nmi_br]" => Atom::NauticalMileBritish,
        "[nmi_i]"  => Atom::NauticalMileInternational,
        "N"        => Atom::Newton,
        "Ohm"      => Atom::Ohm,
        "Oe"       => Atom::Oersted,
        "[oz_av]"  => Atom::OunceAvoirdupois,
        "[oz_tr]"  => Atom::OunceTroy,

        "pc"       => Atom::Parsec,
        "Pa"       => Atom::Pascal,
        "%"        => Atom::Percent,
        "[mu_0]"   => Atom::PermeabilityOfVacuum,
        "[eps_0]"  => Atom::PermittivityOfVacuum,
        "[pH]"     => Atom::PH,
        "ph"       => Atom::Phot,
        "[pc_br]"  => Atom::PaceBritish,
        "[ppb]"    => Atom::PartsPerBillion,
        "[ppm]"    => Atom::PartsPerMillion,
        "[ppth]"   => Atom::PartsPerThousand,
        "[pk_br]"  => Atom::PeckBritish,
        "[pk_us]"  => Atom::PeckUS,
        "[pwt_tr]" => Atom::PennyweightTroy,
        "[pt_br]"  => Atom::PintBritish,
        "[pt_us]"  => Atom::PintUS,
        "[h]"      => Atom::PlanckConstant,
        "[lb_av]"  => Atom::PoundAvoirdupois,
        "[lb_tr]"  => Atom::PoundTroy,
        "[lbf_av]" => Atom::PoundForceAvoirdupois,
        "P"        => Atom::Poise,
        "[p'diop]" => Atom::PrismDiopter,
        "[PNU]"    => Atom::ProteinNitrogenUnit,
        "[m_p]"    => Atom::ProtonMass,
        "[qt_br]"  => Atom::QuartBritish,
        "[qt_us]"  => Atom::QuartUS,
        "[gal_us]" => Atom::QueenAnnesWineGallonUS,
        "[rch_us]" => Atom::RamdensChainUS,
        "RAD"      => Atom::RadiationAbsorbedDose,
        "REM"      => Atom::RadiationEquivalentMan,
        "[rd_br]"  => Atom::RodBritish,
        "[rd_us]"  => Atom::RodUS,
        "R"        => Atom::Roentgen,

        "[sct]"    => Atom::Section,
        "[scwt_av]"=> Atom::ShortHundredweightAvoirdupois,
        "[ston_av]"=> Atom::ShortTonAvoirdupois,
        "S"        => Atom::Siemens,
        "Sv"       => Atom::Sievert,
        "[sft_i]"  => Atom::SquareFootInternational,
        "[sin_i]"  => Atom::SquareInchInternational,
        "[smi_us]" => Atom::SquareMileUS,
        "[srd_us]" => Atom::SquareRodUS,
        "[syd_i]"  => Atom::SquareYardInternational,
        "[g]"      => Atom::StandardAccelerationOfFreeFall,
        "atm"      => Atom::StandardAtmosphere,
        "sr"       => Atom::Steradian,
        "sb"       => Atom::Stilb,
        "[stone_av]"=> Atom::StoneAvoirdupois,
        "mo_s"     => Atom::SynodalMonth,
        "[tbs_us]" => Atom::TablespoonUS,
        "[tsp_us]" => Atom::TeaspoonUS,
        "T"        => Atom::Tesla,
        "[pi]"     => Atom::TheNumberPi,
        "10*"      => Atom::TheNumberTenForArbitraryPowersStar,
        "10^"      => Atom::TheNumberTenForArbitraryPowersCaret,
        "t"        => Atom::Tonne,
        "[twp]"    => Atom::Township,
        "a_t"      => Atom::TropicalYear,

        "[c]"      => Atom::VelocityOfLight,
        "V"        => Atom::Volt,
        "W"        => Atom::Watt,
        "Wb"       => Atom::Weber,
        "wk"       => Atom::Week,
        "[yd_i]"   => Atom::YardInternational,
        "[yd_br]"  => Atom::YardBritish,
        "[yd_us]"  => Atom::YardUS,
        "a"        => Atom::Year,

        _ => {
            match visit_secondary_atom_symbol(pair_str.as_str()) {
                Some(atom) => atom,
                None => return Err(Error::UnknownUnitString(pair_str.as_str().to_string()))
            }
        }
    };

    Ok(atom)
}

fn visit_secondary_atom_symbol(atom_str: &str) -> Option<Atom> {
    let atom = match atom_str {
        // Base units first.
        "CD"  => Atom::Candela,
        "C"           => Atom::Coulomb,
        "G"           => Atom::Gram,
        "K"           => Atom::Kelvin,
        "M"   => Atom::Meter,
        "RAD"         => Atom::Radian,
        "S"           => Atom::Second,

        // Derived units last.
         "[ACR_BR]"           => Atom::AcreBritish,
         "[ACR_US]"           => Atom::AcreUS,
        "A"                                => Atom::Ampere,
         "AR"                 => Atom::Are,
         "ASU"                => Atom::AstronomicUnit,
         "AMU"                => Atom::UnifiedAtomicMassUnit,
         "BAR"                => Atom::Bar,
         "[BBL_US]"           => Atom::BarrelUS,
         "BQ"                 => Atom::Becquerel,
         "BI"                 => Atom::Biot,
         "[BF_I]"             => Atom::BoardFootInternational,
         "[K]"      | "𝑘"     => Atom::BoltzmannConstant,
         "[BU_BR]"            => Atom::BushelBritish,
         "[BU_US]"            => Atom::BushelUS,
         "[CML_I]"            => Atom::CircularMilInternational,
         "[CR_I]"             => Atom::CordInternational,
         "[CRD_US]"           => Atom::CordUS,
         "[CFT_I]"            => Atom::CubicFootInternational,
         "[CIN_I]"            => Atom::CubicInchInternational,
         "[CYD_I]"            => Atom::CubicYardInternational,
         "[CUP_US]"           => Atom::CupUS,
         "CI"                 => Atom::Curie,

         "D"                  => Atom::Day,
         "CEL"       | "°C"                 => Atom::DegreeCelsius,
         "[DEGF]"   | "°F"    => Atom::DegreeFahrenheit,
        "'"                                => Atom::MinuteAngle,
        "''"                               => Atom::Degree,
        "[degRe]"   | "°Ré"                => Atom::DegreeReaumur,
         "DEG"      | "°"     => Atom::Degree,
         "[DR_AV]"            => Atom::DramAvoirdupois,
         "[DPT_US]"           => Atom::DryPintUS,
         "[DQT_US]"           => Atom::DryQuartUS,
         "DYN"                => Atom::Dyne,
         "EV"                 => Atom::Electronvolt,
         "[M_E]"              => Atom::ElectronMass,
         "[E]"      | "𝑒"     => Atom::ElementaryCharge,
         "EQ"                 => Atom::Equivalents,
        "F"                                => Atom::Farad,
         "[FTH_BR]"           => Atom::FathomBritish,
         "[FTH_I]"            => Atom::FathomInternational,
         "[FTH_US]"           => Atom::FathomUS,
         "[FDR_BR]"           => Atom::FluidDramBritish,
         "[FDR_US]"           => Atom::FluidDramUS,
         "[FOZ_BR]"           => Atom::FluidOunceBritish,
         "[FOZ_US]"           => Atom::FluidOunceUS,
         "[FT_BR]"            => Atom::FootBritish,
         "[FT_I]"   | "ft"    => Atom::FootInternational,
         "[FT_US]"            => Atom::FootUS,
         "[FUR_US]"           => Atom::FurlongUS,

         "GL"                 => Atom::Gal,
         "[GAL_BR]"           => Atom::GallonBritish,
         "GS"                 => Atom::Gauss,
         "GB"                 => Atom::Gilbert,
         "[GIL_BR]"           => Atom::GillBritish,
         "[GIL_US]"           => Atom::GillUS,
         "GON"      | "grade" => Atom::Gon,
         "GF"                 => Atom::GramForce,
         "G%"                 => Atom::GramPercent,
         "[GR]"               => Atom::Grain,
         "GY"                 => Atom::Gray,
         "[CH_BR]"            => Atom::GuntersChainBritish,
         "[CH_US]"            => Atom::GuntersChainUS,
         "[HD_I]"             => Atom::HandInternational,
         "HZ"                 => Atom::Hertz,
        "H"                                => Atom::Henry,
         "[GAL_WI]"           => Atom::HistoricalWinchesterGallon,
        "[HP]"                             => Atom::Horsepower,
         "HR"                 => Atom::Hour,
         "[IN_BR]"            => Atom::InchBritish,
         "[IN_I]"   | "in"    => Atom::InchInternational,
         "[IN_US]"            => Atom::InchUS,

        "J"                                => Atom::Joule,
         "KY"                 => Atom::Kayser,
         "[KN_BR]"            => Atom::KnotBritish,
         "[KN_I]"             => Atom::KnotInternational,
         "LMB"                => Atom::Lambert,
         "[LCWT_AV]"          => Atom::LongHunderdweightAvoirdupois,
         "[LTON_AV]"          => Atom::LongTonAvoirdupois,
         "[LY]"     | "l.y."  => Atom::LightYear,
         "[LK_BR]"            => Atom::LinkForGuntersChainBritish,
         "[LK_US]"            => Atom::LinkForGuntersChainUS,
         "[RLK_US]"           => Atom::LinkForRamdensChainUS,
         "L"                  => Atom::Liter,
         "LM"                 => Atom::Lumen,
         "LX"                 => Atom::Lux,

         "MX"                 => Atom::Maxwell,
         "MO_G"               => Atom::MeanGregorianMonth,
         "ANN_G"              => Atom::MeanGregorianYear,
         "MO_J"     | "moⱼ"   => Atom::MeanJulianMonth,
         "ANN_J"    | "aⱼ"    => Atom::MeanJulianYear,
         "[CUP_M]"            => Atom::MetricCup,
         "[FOZ_M]"            => Atom::MetricFluidOunce,
         "[TBS_M]"            => Atom::MetricTablespoon,
         "[TSP_M]"            => Atom::MetricTeaspoon,
        "[MIL_I]"            => Atom::MilInternational,
        "[MIL_US]"           => Atom::MilUS,
        "[MI_BR]"            => Atom::MileBritish,
        "[MI_I]"             => Atom::MileInternational,
        "[MI_US]"            => Atom::MileUS,
        "[MIN_BR]"           => Atom::MinimBritish,
        "[MIN_US]"           => Atom::MinimUS,
        "MOL"                => Atom::Mole,
        "MO"                 => Atom::Month,
        "[NMI_BR]"           => Atom::NauticalMileBritish,
        "[NMI_I]"            => Atom::NauticalMileInternational,
        "N"                                => Atom::Newton,
        "OHM"                => Atom::Ohm,
        "OE"                 => Atom::Oersted,
        "[OZ_AV]"            => Atom::OunceAvoirdupois,
        "[OZ_TR]"            => Atom::OunceTroy,

        "PRS"                => Atom::Parsec,
        "PAL"                => Atom::Pascal,
        "%"                                => Atom::Percent,
        "[MU_0]"    | "μ₀"   => Atom::PermeabilityOfVacuum,
        "[EPS_0]"   | "ε₀"   => Atom::PermittivityOfVacuum,
        "[PH]"               => Atom::PH,
        "PHT"                => Atom::Phot,
        "[PC_BR]"            => Atom::PaceBritish,
        "[PPB]"     | "ppb"  => Atom::PartsPerBillion,
        "[PPM]"     | "ppm"  => Atom::PartsPerMillion,
        "[PPTH]"    | "ppt"  => Atom::PartsPerThousand,
        "[PK_BR]"            => Atom::PeckBritish,
        "[PK_US]"            => Atom::PeckUS,
        "[PWT_TR]"           => Atom::PennyweightTroy,
        "[PT_BR]"            => Atom::PintBritish,
        "[PT_US]"            => Atom::PintUS,
        "[H]"                => Atom::PlanckConstant,
        "[LB_AV]"            => Atom::PoundAvoirdupois,
        "[LB_TR]"            => Atom::PoundTroy,
        "[LBF_AV]"  | "lbf"  => Atom::PoundForceAvoirdupois,
        "P"                                => Atom::Poise,
        "[P'DIOP]"  | "PD"   => Atom::PrismDiopter,
        "[PNU]"                            => Atom::ProteinNitrogenUnit,
        "[M_P]"     | "𝑚ₚ"   => Atom::ProtonMass,
        "[QT_BR]"            => Atom::QuartBritish,
        "[QT_US]"            => Atom::QuartUS,
        "[GAL_US]"           => Atom::QueenAnnesWineGallonUS,
        "[RCH_US]"           => Atom::RamdensChainUS,
        "[RAD]"              => Atom::RadiationAbsorbedDose,
        "[REM]"              => Atom::RadiationEquivalentMan,
        "[RD_BR]"            => Atom::RodBritish,
        "[RD_US]"            => Atom::RodUS,
        "ROE"                => Atom::Roentgen,

        "[SCT]"              => Atom::Section,
        "[SCWT_AV]"          => Atom::ShortHundredweightAvoirdupois,
        "[STON_AV]"          => Atom::ShortTonAvoirdupois,
        "SIE"                => Atom::Siemens,
        "SV"                 => Atom::Sievert,
        "[SFT_I]"            => Atom::SquareFootInternational,
        "[SIN_I]"            => Atom::SquareInchInternational,
        "[SMI_US]"           => Atom::SquareMileUS,
        "[SRD_US]"           => Atom::SquareRodUS,
        "[SYD_I]"            => Atom::SquareYardInternational,
        "[G]"        | "𝑔"                  => Atom::StandardAccelerationOfFreeFall,
        "ATM"                => Atom::StandardAtmosphere,
        "SR"                 => Atom::Steradian,
        "SB"                 => Atom::Stilb,
        "[STONE_AV]"         => Atom::StoneAvoirdupois,
        "MO_S"       | "moₛ" => Atom::SynodalMonth,
        "[TBS_US]"           => Atom::TablespoonUS,
        "[TSP_US]"           => Atom::TeaspoonUS,
        "T"                                 => Atom::Tesla,
        "[PI]"      | "π"     => Atom::TheNumberPi,
        "10*"                               => Atom::TheNumberTenForArbitraryPowersStar,
        "10^"                               => Atom::TheNumberTenForArbitraryPowersCaret,
        "TNE"                 => Atom::Tonne,
        "[TWP]"               => Atom::Township,
        "ANN_T"     | "aₜ"    => Atom::TropicalYear,

        "[C]"       | "𝑐"    => Atom::VelocityOfLight,
        "V"                                => Atom::Volt,
        "W"                                => Atom::Watt,
        "WB"                 => Atom::Weber,
        "WK"                 => Atom::Week,
        "[YD_I]"             => Atom::YardInternational,
        "[YD_BR]"            => Atom::YardBritish,
        "[YD_US]"            => Atom::YardUS,
        "ANN"                => Atom::Year,

        _ => return None
    };

    Some(atom)
}

fn visit_annotatable(pair: Pair<'_, Rule>) -> Result<Annotatable, Error> {
    let mut prefix: Option<Prefix> = None;
    let mut atom: Option<Atom> = None;
    let mut exponent: Option<i32> = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::simple_unit => {
                let simple_unit = visit_simple_unit(inner_pair)?;

                atom = simple_unit.atom;
                prefix = simple_unit.prefix;
            }
            Rule::exponent => {
                exponent = visit_exponent(inner_pair)?;
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(Annotatable {
        prefix,
        atom,
        exponent,
    })
}

fn visit_annotation(pair: Pair<'_, Rule>) -> Result<String, Error> {
    let string = pair.into_span().as_str().to_string();

    Ok(string)
}

fn visit_factor(pair: Pair<'_, Rule>) -> Result<u32, Error> {
    let span = pair.into_span();
    let string = span.as_str();

    string.parse::<u32>().map_err(|e| Error::UnableToParse {
        expression: e.to_string(),
    })
}

fn visit_basic_component(pair: Pair<'_, Rule>) -> Result<BasicComponent, Error> {
    let mut bc = BasicComponent::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::annotatable => {
                let annotatable = visit_annotatable(inner_pair)?;

                bc.prefix = annotatable.prefix;
                bc.atom = annotatable.atom;
                bc.exponent = annotatable.exponent;
            }
            Rule::annotation => {
                let annotation = visit_annotation(inner_pair)?;

                bc.annotation = Some(annotation);
            }
            Rule::factor => {
                bc.factor = Some(visit_factor(inner_pair)?);
            }
            Rule::term => {
                let ast_term = visit_term(inner_pair)?;

                bc.terms = ast_term.finish();
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(bc)
}

fn visit_component(pair: Pair<'_, Rule>) -> Result<Component, Error> {
    let mut factor: Option<u32> = None;
    let mut terms: Vec<Term> = vec![];

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::factor => {
                factor = Some(visit_factor(inner_pair)?);
            }
            Rule::basic_component => {
                let bc = visit_basic_component(inner_pair)?;

                terms = bc.finish();
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(Component { factor, terms })
}

fn visit_term(pair: Pair<'_, Rule>) -> Result<AstTerm, Error> {
    let mut has_slash = false;
    let mut component: Option<Component> = None;
    let mut terms: Vec<Term> = vec![];

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::term => {
                let mut new_terms: Vec<Term> = visit_term(inner_pair)?.finish();

                if has_slash {
                    new_terms.invert();
                    has_slash = false;
                }

                terms = new_terms;
            }
            Rule::slash => {
                has_slash = true;
            }
            Rule::component => {
                component = Some(visit_component(inner_pair)?);
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(AstTerm { component, terms })
}

fn visit_main_term(pair: Pair<'_, Rule>) -> Result<MainTerm, Error> {
    let mut terms: Vec<Term> = vec![];
    let mut has_slash = false;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::slash => {
                has_slash = true;
            }
            Rule::term => {
                let mut new_terms: Vec<Term> = visit_term(inner_pair)?.finish();

                if has_slash {
                    new_terms.invert();
                    has_slash = false;
                }

                terms = new_terms;
            }
            _ => {
                let error = Error::UnableToParse {
                    expression: inner_pair.as_str().to_string(),
                };
                return Err(error);
            }
        }
    }

    Ok(MainTerm { terms })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::terms::term_parser::{Rule, TermParser};
    use crate::parser::{Atom, Prefix, Term};
    use pest::Parser;

    macro_rules! validate_interpret {
        ($test_name:ident, $input:expr, $($terms:expr),+) => {
            #[test]
            fn $test_name() {
                let pairs = TermParser::parse(Rule::main_term, $input).unwrap();
                let actual = map(pairs).unwrap();
                let expected = vec![$($terms),+];

                assert_eq!(actual, expected);
            }
        };
    }

    #[test]
    fn validate_exponent() {
        let pairs = TermParser::parse(Rule::main_term, "m-3").unwrap();
        let actual = map(pairs).unwrap();

        let expected_term = term!(Meter, exponent: -3);
        let expected = vec![expected_term];

        assert_eq!(actual, expected);

        let pairs = TermParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = map(pairs).unwrap();

        let term1 = term!(Kilo, Meter, exponent: 2);
        let term2 = term!(Meter, exponent: 3);
        let expected = vec![term1, term2];

        assert_eq!(actual, expected);
    }

    validate_interpret!(validate_interpret_meter, "m", term!(Meter));
    validate_interpret!(
        validate_interpret_meter_positive_exponent,
        "m2",
        term!(Meter, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter_negative_exponent,
        "m-2",
        term!(Meter, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter_factor,
        "2m",
        term!(Meter, factor: 2)
    );
    validate_interpret!(validate_interpret_kilometer, "km", term!(Kilo, Meter));

    // Slash terms
    validate_interpret!(
        validate_interpret_meter_per_second,
        "m/s",
        term!(Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_kilometer_per_second,
        "km/s",
        term!(Kilo, Meter),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_kilosecond,
        "m/ks",
        term!(Meter),
        term!(Kilo, Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second,
        "m2/s",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_second2,
        "m/s2",
        term!(Meter),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_meter2_per_second2,
        "m2/s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_2meter_per_second,
        "2m/s",
        term!(Meter, factor: 2),
        term!(Second, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_meter_per_2second,
        "m/2s",
        term!(Meter),
        term!(Second, exponent: -1, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_per_2second2,
        "2m2/2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: -2)
    );
    validate_interpret!(
        validate_interpret_foot_per_factor,
        "[ft_i]/12",
        term!(FootInternational),
        term!(factor: 12, exponent: -1)
    );

    // Dot terms
    validate_interpret!(
        validate_interpret_meter_second,
        "m.s",
        term!(Meter),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter2_second,
        "m2.s",
        term!(Meter, exponent: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_second2,
        "m.s2",
        term!(Meter),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_meter2_second2,
        "m2.s2",
        term!(Meter, exponent: 2),
        term!(Second, exponent: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_second,
        "2m.s",
        term!(Meter, factor: 2),
        term!(Second)
    );
    validate_interpret!(
        validate_interpret_meter_2second,
        "m.2s",
        term!(Meter),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter_2second,
        "2m.2s",
        term!(Meter, factor: 2),
        term!(Second, factor: 2)
    );
    validate_interpret!(
        validate_interpret_2meter2_2second2,
        "2m2.2s2",
        term!(Meter, factor: 2, exponent: 2),
        term!(Second, factor: 2, exponent: 2)
    );

    // Dot and slash combined terms
    validate_interpret!(
        validate_interpret_acre_inch_per_acre,
        "[acr_us].[in_i]/[acr_us]",
        term!(AcreUS),
        term!(InchInternational),
        term!(AcreUS, exponent: -1)
    );
    validate_interpret!(
        validate_interpret_2acre3_3inch4_per_4acre5,
        "2[acr_us]3.3[in_i]4/4[acr_us]5",
        term!(AcreUS, factor: 2, exponent: 3),
        term!(InchInternational, factor: 3, exponent: 4),
        term!(AcreUS, factor: 4, exponent: -5)
    );

    #[test]
    #[ignore]
    fn validate_custom_atom() {
        let pairs = TermParser::parse(Rule::main_term, "[meow]").unwrap();

        let actual = map(pairs).unwrap();
        let acre_term = term!(AcreUS);
        let inch_term = term!(InchInternational);
        let acre2_term = term!(AcreUS, exponent: -1);

        let expected = vec![acre_term, inch_term, acre2_term];

        assert_eq!(actual, expected);
    }
}
