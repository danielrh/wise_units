//-----------------------------------------------------------------------------
// DO NOT EDIT THIS FILE!
// This is generated by wise_units-atom_generator.
//-----------------------------------------------------------------------------

use crate::parser::Atom;
use std::fmt;

/// Property categorizes the unit by use. Not much mention of it in the UCUM
/// HTML spec, but is used throughout the
/// [XML description](http://unitsofmeasure.org/ucum-essence.xml).
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    Acceleration,
    Acidity,
    Action,
    ActionArea,
    AmountOfAProliferatingOrganism,
    AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters,
    AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod,
    AmountOfAnInfectiousAgent,
    AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers,
    AmountOfInformation,
    AmountOfSubstance,
    AmountOfSubstanceDissolvedParticles,
    AmplitudeSpectralDensity,
    Arbitrary,
    ArbitraryBiologicActivity,
    ArbitraryElisaUnit,
    Area,
    BiologicActivityAntistreptolysinO,
    BiologicActivityInfectivityOfAnInfectiousAgentPreparation,
    BiologicActivityOfAmylase,
    BiologicActivityOfAnticardiolipinIgA,
    BiologicActivityOfAnticardiolipinIgG,
    BiologicActivityOfAnticardiolipinIgM,
    BiologicActivityOfFactorViiiInhibitor,
    BiologicActivityOfFactorXaInhibitorHeparin,
    BiologicActivityOfPhosphatase,
    BiologicActivityOfTuberculin,
    Brightness,
    CatalyticActivity,
    DepthOfWater,
    DoseEquivalent,
    DryVolume,
    DynamicViscosity,
    EhrlichUnit,
    ElectricCapacitance,
    ElectricCharge,
    ElectricConductance,
    ElectricCurrent,
    ElectricPermittivity,
    ElectricPotential,
    ElectricPotentialLevel,
    ElectricResistance,
    Energy,
    EnergyDose,
    FluidResistance,
    FluidVolume,
    FluxOfMagneticInduction,
    Force,
    Fraction,
    Frequency,
    GaugeOfCatheters,
    HeightOfHorses,
    HomeopathicPotencyHahnemann,
    HomeopathicPotencyKorsakov,
    HomeopathicPotencyRetired,
    Illuminance,
    Inductance,
    IonDose,
    KinematicViscosity,
    Length,
    Level,
    LinearMassDensityOfTextileThread,
    LineicNumber,
    LumIntensityDensity,
    LuminousFlux,
    LuminousIntensity,
    MagneticFieldIntensity,
    MagneticFlux,
    MagneticFluxDensity,
    MagneticPermeability,
    MagneticTension,
    Mass,
    MassConcentration,
    MassFraction,
    MetabolicCostOfPhysicalActivity,
    Number,
    PlaneAngle,
    Power,
    PowerLevel,
    Pressure,
    PressureLevel,
    ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance,
    ProcedureDefinedAmountOfAProteinSubstance,
    ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard,
    ProcedureDefinedAmountOfAnAntigenSubstance,
    ProcedureDefinedAmountOfTheMajorAllergenOfRagweed,
    Radioactivity,
    RefractionOfALens,
    RefractionOfAPrism,
    SedimentationCoefficient,
    SignalTransmissionRate,
    Slope,
    SolidAngle,
    Temperature,
    Time,
    Unclassified,
    Velocity,
    ViewAreaInMicroscope,
    Volume,
    XRayAttenuation,
}

impl Property {
    /// All `Atom`s that match the `Property` variant.
    ///
    /// ```
    /// extern crate wise_units;
    /// use wise_units::{Atom, Property};
    ///
    /// assert_eq!(Property::Acidity.atoms(), vec![Atom::PH]);
    /// ```
    ///
    #[must_use]
    pub fn atoms(self) -> Vec<Atom> {
        match self {
            Self::Acceleration => {
                vec![
                    Atom::StandardAccelerationOfFreeFall,
                    Atom::Gal,
                ]
            },
            Self::Acidity => {
                vec![
                    Atom::PH,
                ]
            },
            Self::Action => {
                vec![
                    Atom::PlanckConstant,
                ]
            },
            Self::ActionArea => {
                vec![
                    Atom::Barn,
                ]
            },
            Self::AmountOfAProliferatingOrganism => {
                vec![
                    Atom::ColonyFormingUnits,
                ]
            },
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters => {
                vec![
                    Atom::BioequivalentAllergenUnit,
                ]
            },
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod => {
                vec![
                    Atom::IndexOfReactivity,
                ]
            },
            Self::AmountOfAnInfectiousAgent => {
                vec![
                    Atom::PlaqueFormingUnits,
                    Atom::FocusFormingUnits,
                ]
            },
            Self::AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers => {
                vec![
                    Atom::FibrinogenEquivalentUnit,
                ]
            },
            Self::AmountOfInformation => {
                vec![
                    Atom::BitLogarithmusDualis,
                    Atom::Bit,
                    Atom::Byte,
                ]
            },
            Self::AmountOfSubstance => {
                vec![
                    Atom::Mole,
                    Atom::Equivalents,
                ]
            },
            Self::AmountOfSubstanceDissolvedParticles => {
                vec![
                    Atom::Osmole,
                ]
            },
            Self::AmplitudeSpectralDensity => {
                vec![
                    Atom::MeterPerSquareSecondsPerSquareRootOfHertz,
                ]
            },
            Self::Arbitrary => {
                vec![
                    Atom::InternationalUnit,
                    Atom::InternationalUnitSecondary,
                    Atom::ArbitaryUnit,
                    Atom::UnitedStatesPharmacopeiaUnit,
                ]
            },
            Self::ArbitraryBiologicActivity => {
                vec![
                    Atom::KunkelUnit,
                    Atom::MacLaganUnit,
                ]
            },
            Self::ArbitraryElisaUnit => {
                vec![
                    Atom::ElisaUnit,
                ]
            },
            Self::Area => {
                vec![
                    Atom::Are,
                    Atom::SquareInchInternational,
                    Atom::SquareFootInternational,
                    Atom::SquareYardInternational,
                    Atom::CircularMilInternational,
                    Atom::AcreUS,
                    Atom::SquareRodUS,
                    Atom::SquareMileUS,
                    Atom::Section,
                    Atom::Township,
                    Atom::AcreBritish,
                ]
            },
            Self::BiologicActivityAntistreptolysinO => {
                vec![
                    Atom::ToddUnit,
                ]
            },
            Self::BiologicActivityInfectivityOfAnInfectiousAgentPreparation => {
                vec![
                    Atom::CellCultureInfectiousDose,
                    Atom::TissueCultureInfectiousDose,
                    Atom::EmbryoInfectiousDose,
                ]
            },
            Self::BiologicActivityOfAmylase => {
                vec![
                    Atom::DyeUnit,
                    Atom::SomogyiUnit,
                ]
            },
            Self::BiologicActivityOfAnticardiolipinIgA => {
                vec![
                    Atom::AplUnit,
                ]
            },
            Self::BiologicActivityOfAnticardiolipinIgG => {
                vec![
                    Atom::GplUnit,
                ]
            },
            Self::BiologicActivityOfAnticardiolipinIgM => {
                vec![
                    Atom::MplUnit,
                ]
            },
            Self::BiologicActivityOfFactorViiiInhibitor => {
                vec![
                    Atom::BethesdaUnit,
                ]
            },
            Self::BiologicActivityOfFactorXaInhibitorHeparin => {
                vec![
                    Atom::AntiFactorXaUnit,
                ]
            },
            Self::BiologicActivityOfPhosphatase => {
                vec![
                    Atom::BodanskyUnit,
                    Atom::KingArmstrongUnit,
                ]
            },
            Self::BiologicActivityOfTuberculin => {
                vec![
                    Atom::TuberculinUnit,
                ]
            },
            Self::Brightness => {
                vec![
                    Atom::Lambert,
                ]
            },
            Self::CatalyticActivity => {
                vec![
                    Atom::Katal,
                    Atom::Unit,
                ]
            },
            Self::DepthOfWater => {
                vec![
                    Atom::FathomInternational,
                ]
            },
            Self::DoseEquivalent => {
                vec![
                    Atom::Sievert,
                    Atom::RadiationEquivalentMan,
                ]
            },
            Self::DryVolume => {
                vec![
                    Atom::BushelUS,
                    Atom::HistoricalWinchesterGallon,
                    Atom::PeckUS,
                    Atom::DryQuartUS,
                    Atom::DryPintUS,
                ]
            },
            Self::DynamicViscosity => {
                vec![
                    Atom::Poise,
                ]
            },
            Self::EhrlichUnit => {
                vec![
                    Atom::EhrlichUnit,
                ]
            },
            Self::ElectricCapacitance => {
                vec![
                    Atom::Farad,
                ]
            },
            Self::ElectricCharge => {
                vec![
                    Atom::Coulomb,
                    Atom::ElementaryCharge,
                ]
            },
            Self::ElectricConductance => {
                vec![
                    Atom::Siemens,
                    Atom::Mho,
                ]
            },
            Self::ElectricCurrent => {
                vec![
                    Atom::Ampere,
                    Atom::Biot,
                ]
            },
            Self::ElectricPermittivity => {
                vec![
                    Atom::PermittivityOfVacuum,
                ]
            },
            Self::ElectricPotential => {
                vec![
                    Atom::Volt,
                ]
            },
            Self::ElectricPotentialLevel => {
                vec![
                    Atom::BelVolt,
                    Atom::BelMillivolt,
                    Atom::BelMicrovolt,
                    Atom::Bel10Nanovolt,
                ]
            },
            Self::ElectricResistance => {
                vec![
                    Atom::Ohm,
                ]
            },
            Self::Energy => {
                vec![
                    Atom::Joule,
                    Atom::Electronvolt,
                    Atom::Erg,
                    Atom::CalorieAt15C,
                    Atom::CalorieAt20C,
                    Atom::MeanCalorie,
                    Atom::InternationalTableCalorie,
                    Atom::ThermochemicalCalorie,
                    Atom::Calorie,
                    Atom::NutritionLabelCalories,
                    Atom::BritishThermalUnitAt39F,
                    Atom::BritishThermalUnitAt59F,
                    Atom::BritishThermalUnitAt60F,
                    Atom::MeanBritishThermalUnit,
                    Atom::InternationalTableBritishThermalUnit,
                    Atom::ThermochemicalBritishThermalUnit,
                    Atom::BritishThermalUnit,
                ]
            },
            Self::EnergyDose => {
                vec![
                    Atom::Gray,
                    Atom::RadiationAbsorbedDose,
                ]
            },
            Self::FluidResistance => {
                vec![
                    Atom::PeripheralVascularResistanceUnit,
                    Atom::WoodUnit,
                ]
            },
            Self::FluidVolume => {
                vec![
                    Atom::QueenAnnesWineGallonUS,
                    Atom::BarrelUS,
                    Atom::QuartUS,
                    Atom::PintUS,
                    Atom::GillUS,
                    Atom::FluidOunceUS,
                    Atom::FluidDramUS,
                    Atom::MinimUS,
                    Atom::CordUS,
                    Atom::MetricFluidOunce,
                ]
            },
            Self::FluxOfMagneticInduction => {
                vec![
                    Atom::Maxwell,
                ]
            },
            Self::Force => {
                vec![
                    Atom::Newton,
                    Atom::GramForce,
                    Atom::PoundForceAvoirdupois,
                    Atom::Dyne,
                ]
            },
            Self::Fraction => {
                vec![
                    Atom::Percent,
                    Atom::PartsPerThousand,
                    Atom::PartsPerMillion,
                    Atom::PartsPerBillion,
                    Atom::PartsPerTrillion,
                ]
            },
            Self::Frequency => {
                vec![
                    Atom::Hertz,
                ]
            },
            Self::GaugeOfCatheters => {
                vec![
                    Atom::Charriere,
                ]
            },
            Self::HeightOfHorses => {
                vec![
                    Atom::HandInternational,
                ]
            },
            Self::HomeopathicPotencyHahnemann => {
                vec![
                    Atom::HomeopathicPotencyOfDecimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfCentesimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfMillesimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries,
                ]
            },
            Self::HomeopathicPotencyKorsakov => {
                vec![
                    Atom::HomeopathicPotencyOfDecimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfCentesimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfMillesimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries,
                ]
            },
            Self::HomeopathicPotencyRetired => {
                vec![
                    Atom::HomeopathicPotencyOfDecimalSeriesRetired,
                    Atom::HomeopathicPotencyOfCentesimalSeriesRetired,
                    Atom::HomeopathicPotencyOfMillesimalSeriesRetired,
                    Atom::HomeopathicPotencyOfQuintamillesimalSeriesRetired,
                ]
            },
            Self::Illuminance => {
                vec![
                    Atom::Lux,
                    Atom::Phot,
                ]
            },
            Self::Inductance => {
                vec![
                    Atom::Henry,
                ]
            },
            Self::IonDose => {
                vec![
                    Atom::Roentgen,
                ]
            },
            Self::KinematicViscosity => {
                vec![
                    Atom::Stokes,
                ]
            },
            Self::Length => {
                vec![
                    Atom::Meter,
                    Atom::AstronomicUnit,
                    Atom::Parsec,
                    Atom::LightYear,
                    Atom::InchInternational,
                    Atom::FootInternational,
                    Atom::YardInternational,
                    Atom::MileInternational,
                    Atom::NauticalMileInternational,
                    Atom::MilInternational,
                    Atom::FootUS,
                    Atom::YardUS,
                    Atom::InchUS,
                    Atom::RodUS,
                    Atom::GuntersChainUS,
                    Atom::LinkForGuntersChainUS,
                    Atom::RamdensChainUS,
                    Atom::LinkForRamdensChainUS,
                    Atom::FathomUS,
                    Atom::FurlongUS,
                    Atom::MileUS,
                    Atom::MilUS,
                    Atom::InchBritish,
                    Atom::FootBritish,
                    Atom::RodBritish,
                    Atom::GuntersChainBritish,
                    Atom::LinkForGuntersChainBritish,
                    Atom::FathomBritish,
                    Atom::PaceBritish,
                    Atom::YardBritish,
                    Atom::MileBritish,
                    Atom::NauticalMileBritish,
                    Atom::Line,
                    Atom::Point,
                    Atom::Pica,
                    Atom::PrintersPoint,
                    Atom::PrintersPica,
                    Atom::Pied,
                    Atom::Pouce,
                    Atom::Ligne,
                    Atom::Didot,
                    Atom::Cicero,
                    Atom::Angstrom,
                    Atom::Smoot,
                ]
            },
            Self::Level => {
                vec![
                    Atom::Neper,
                    Atom::Bel,
                ]
            },
            Self::LinearMassDensityOfTextileThread => {
                vec![
                    Atom::Tex,
                    Atom::Denier,
                ]
            },
            Self::LineicNumber => {
                vec![
                    Atom::Kayser,
                    Atom::MeshInternational,
                ]
            },
            Self::LumIntensityDensity => {
                vec![
                    Atom::Stilb,
                ]
            },
            Self::LuminousFlux => {
                vec![
                    Atom::Lumen,
                ]
            },
            Self::LuminousIntensity => {
                vec![
                    Atom::Candela,
                ]
            },
            Self::MagneticFieldIntensity => {
                vec![
                    Atom::Oersted,
                ]
            },
            Self::MagneticFlux => {
                vec![
                    Atom::Weber,
                ]
            },
            Self::MagneticFluxDensity => {
                vec![
                    Atom::Tesla,
                    Atom::Gauss,
                ]
            },
            Self::MagneticPermeability => {
                vec![
                    Atom::PermeabilityOfVacuum,
                ]
            },
            Self::MagneticTension => {
                vec![
                    Atom::Gilbert,
                ]
            },
            Self::Mass => {
                vec![
                    Atom::Gram,
                    Atom::Tonne,
                    Atom::UnifiedAtomicMassUnit,
                    Atom::ElectronMass,
                    Atom::ProtonMass,
                    Atom::Grain,
                    Atom::PoundAvoirdupois,
                    Atom::OunceAvoirdupois,
                    Atom::DramAvoirdupois,
                    Atom::ShortHundredweightAvoirdupois,
                    Atom::LongHunderdweightAvoirdupois,
                    Atom::ShortTonAvoirdupois,
                    Atom::LongTonAvoirdupois,
                    Atom::StoneAvoirdupois,
                    Atom::PennyweightTroy,
                    Atom::OunceTroy,
                    Atom::PoundTroy,
                    Atom::ScrupleApothecaries,
                    Atom::DramApothecaries,
                    Atom::OunceApothecaries,
                    Atom::PoundApothecaries,
                    Atom::MetricOunce,
                    Atom::MetricCarat,
                ]
            },
            Self::MassConcentration => {
                vec![
                    Atom::GramPercent,
                ]
            },
            Self::MassFraction => {
                vec![
                    Atom::CaratOfGoldAlloys,
                ]
            },
            Self::MetabolicCostOfPhysicalActivity => {
                vec![
                    Atom::MetabolicEquivalent,
                ]
            },
            Self::Number => {
                vec![
                    Atom::TheNumberTenForArbitraryPowersStar,
                    Atom::TheNumberTenForArbitraryPowersCaret,
                    Atom::TheNumberPi,
                ]
            },
            Self::PlaneAngle => {
                vec![
                    Atom::Radian,
                    Atom::Gon,
                    Atom::Degree,
                    Atom::MinuteAngle,
                    Atom::SecondAngle,
                    Atom::Circle,
                ]
            },
            Self::Power => {
                vec![
                    Atom::Watt,
                    Atom::Horsepower,
                ]
            },
            Self::PowerLevel => {
                vec![
                    Atom::BelWatt,
                    Atom::BelKilowatt,
                ]
            },
            Self::Pressure => {
                vec![
                    Atom::Pascal,
                    Atom::Bar,
                    Atom::StandardAtmosphere,
                    Atom::MeterOfWaterColumn,
                    Atom::MeterOfMercuryColumn,
                    Atom::InchOfWaterColumn,
                    Atom::InchOfMercuryColumn,
                    Atom::TechnicalAtmosphere,
                    Atom::PoundPerSqareInch,
                ]
            },
            Self::PressureLevel => {
                vec![
                    Atom::BelSoundPressure,
                ]
            },
            Self::ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance => {
                vec![
                    Atom::DAntigenUnit,
                ]
            },
            Self::ProcedureDefinedAmountOfAProteinSubstance => {
                vec![
                    Atom::ProteinNitrogenUnit,
                ]
            },
            Self::ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard => {
                vec![
                    Atom::AllergenUnit,
                ]
            },
            Self::ProcedureDefinedAmountOfAnAntigenSubstance => {
                vec![
                    Atom::LimitOfFlocculation,
                ]
            },
            Self::ProcedureDefinedAmountOfTheMajorAllergenOfRagweed => {
                vec![
                    Atom::AllergenUnitForAmbrosiaArtemisiifolia,
                ]
            },
            Self::Radioactivity => {
                vec![
                    Atom::Becquerel,
                    Atom::Curie,
                ]
            },
            Self::RefractionOfALens => {
                vec![
                    Atom::Diopter,
                ]
            },
            Self::RefractionOfAPrism => {
                vec![
                    Atom::PrismDiopter,
                ]
            },
            Self::SedimentationCoefficient => {
                vec![
                    Atom::SvedbergUnit,
                ]
            },
            Self::SignalTransmissionRate => {
                vec![
                    Atom::Baud,
                ]
            },
            Self::Slope => {
                vec![
                    Atom::PercentOfSlope,
                ]
            },
            Self::SolidAngle => {
                vec![
                    Atom::Steradian,
                    Atom::Spere,
                ]
            },
            Self::Temperature => {
                vec![
                    Atom::Kelvin,
                    Atom::DegreeCelsius,
                    Atom::DegreeFahrenheit,
                    Atom::DegreeRankine,
                    Atom::DegreeReaumur,
                ]
            },
            Self::Time => {
                vec![
                    Atom::Second,
                    Atom::Minute,
                    Atom::Hour,
                    Atom::Day,
                    Atom::TropicalYear,
                    Atom::MeanJulianYear,
                    Atom::MeanGregorianYear,
                    Atom::Year,
                    Atom::Week,
                    Atom::SynodalMonth,
                    Atom::MeanJulianMonth,
                    Atom::MeanGregorianMonth,
                    Atom::Month,
                ]
            },
            Self::Unclassified => {
                vec![
                    Atom::BoltzmannConstant,
                    Atom::NewtonianConstantOfGravitation,
                ]
            },
            Self::Velocity => {
                vec![
                    Atom::VelocityOfLight,
                    Atom::KnotInternational,
                    Atom::KnotBritish,
                ]
            },
            Self::ViewAreaInMicroscope => {
                vec![
                    Atom::HighPowerField,
                    Atom::LowPowerField,
                ]
            },
            Self::Volume => {
                vec![
                    Atom::Liter,
                    Atom::LiterSecondary,
                    Atom::CubicInchInternational,
                    Atom::CubicFootInternational,
                    Atom::CubicYardInternational,
                    Atom::BoardFootInternational,
                    Atom::CordInternational,
                    Atom::TablespoonUS,
                    Atom::TeaspoonUS,
                    Atom::CupUS,
                    Atom::MetricCup,
                    Atom::MetricTeaspoon,
                    Atom::MetricTablespoon,
                    Atom::GallonBritish,
                    Atom::PeckBritish,
                    Atom::BushelBritish,
                    Atom::QuartBritish,
                    Atom::PintBritish,
                    Atom::GillBritish,
                    Atom::FluidOunceBritish,
                    Atom::FluidDramBritish,
                    Atom::MinimBritish,
                    Atom::Drop,
                    Atom::Stere,
                ]
            },
            Self::XRayAttenuation => {
                vec![
                    Atom::HounsfieldUnit,
                ]
            },
        }
    }
}

impl Default for Property {
    fn default() -> Self {
        Self::Unclassified
    }
}

impl fmt::Display for Property {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Acceleration => {
                "Acceleration"
            },
            Self::Acidity => {
                "Acidity"
            },
            Self::Action => {
                "Action"
            },
            Self::ActionArea => {
                "ActionArea"
            },
            Self::AmountOfAProliferatingOrganism => {
                "AmountOfAProliferatingOrganism"
            },
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters => {
                "AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters"
            },
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod => {
                "AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod"
            },
            Self::AmountOfAnInfectiousAgent => {
                "AmountOfAnInfectiousAgent"
            },
            Self::AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers => {
                "AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers"
            },
            Self::AmountOfInformation => {
                "AmountOfInformation"
            },
            Self::AmountOfSubstance => {
                "AmountOfSubstance"
            },
            Self::AmountOfSubstanceDissolvedParticles => {
                "AmountOfSubstanceDissolvedParticles"
            },
            Self::AmplitudeSpectralDensity => {
                "AmplitudeSpectralDensity"
            },
            Self::Arbitrary => {
                "Arbitrary"
            },
            Self::ArbitraryBiologicActivity => {
                "ArbitraryBiologicActivity"
            },
            Self::ArbitraryElisaUnit => {
                "ArbitraryElisaUnit"
            },
            Self::Area => {
                "Area"
            },
            Self::BiologicActivityAntistreptolysinO => {
                "BiologicActivityAntistreptolysinO"
            },
            Self::BiologicActivityInfectivityOfAnInfectiousAgentPreparation => {
                "BiologicActivityInfectivityOfAnInfectiousAgentPreparation"
            },
            Self::BiologicActivityOfAmylase => {
                "BiologicActivityOfAmylase"
            },
            Self::BiologicActivityOfAnticardiolipinIgA => {
                "BiologicActivityOfAnticardiolipinIgA"
            },
            Self::BiologicActivityOfAnticardiolipinIgG => {
                "BiologicActivityOfAnticardiolipinIgG"
            },
            Self::BiologicActivityOfAnticardiolipinIgM => {
                "BiologicActivityOfAnticardiolipinIgM"
            },
            Self::BiologicActivityOfFactorViiiInhibitor => {
                "BiologicActivityOfFactorViiiInhibitor"
            },
            Self::BiologicActivityOfFactorXaInhibitorHeparin => {
                "BiologicActivityOfFactorXaInhibitorHeparin"
            },
            Self::BiologicActivityOfPhosphatase => {
                "BiologicActivityOfPhosphatase"
            },
            Self::BiologicActivityOfTuberculin => {
                "BiologicActivityOfTuberculin"
            },
            Self::Brightness => {
                "Brightness"
            },
            Self::CatalyticActivity => {
                "CatalyticActivity"
            },
            Self::DepthOfWater => {
                "DepthOfWater"
            },
            Self::DoseEquivalent => {
                "DoseEquivalent"
            },
            Self::DryVolume => {
                "DryVolume"
            },
            Self::DynamicViscosity => {
                "DynamicViscosity"
            },
            Self::EhrlichUnit => {
                "EhrlichUnit"
            },
            Self::ElectricCapacitance => {
                "ElectricCapacitance"
            },
            Self::ElectricCharge => {
                "ElectricCharge"
            },
            Self::ElectricConductance => {
                "ElectricConductance"
            },
            Self::ElectricCurrent => {
                "ElectricCurrent"
            },
            Self::ElectricPermittivity => {
                "ElectricPermittivity"
            },
            Self::ElectricPotential => {
                "ElectricPotential"
            },
            Self::ElectricPotentialLevel => {
                "ElectricPotentialLevel"
            },
            Self::ElectricResistance => {
                "ElectricResistance"
            },
            Self::Energy => {
                "Energy"
            },
            Self::EnergyDose => {
                "EnergyDose"
            },
            Self::FluidResistance => {
                "FluidResistance"
            },
            Self::FluidVolume => {
                "FluidVolume"
            },
            Self::FluxOfMagneticInduction => {
                "FluxOfMagneticInduction"
            },
            Self::Force => {
                "Force"
            },
            Self::Fraction => {
                "Fraction"
            },
            Self::Frequency => {
                "Frequency"
            },
            Self::GaugeOfCatheters => {
                "GaugeOfCatheters"
            },
            Self::HeightOfHorses => {
                "HeightOfHorses"
            },
            Self::HomeopathicPotencyHahnemann => {
                "HomeopathicPotencyHahnemann"
            },
            Self::HomeopathicPotencyKorsakov => {
                "HomeopathicPotencyKorsakov"
            },
            Self::HomeopathicPotencyRetired => {
                "HomeopathicPotencyRetired"
            },
            Self::Illuminance => {
                "Illuminance"
            },
            Self::Inductance => {
                "Inductance"
            },
            Self::IonDose => {
                "IonDose"
            },
            Self::KinematicViscosity => {
                "KinematicViscosity"
            },
            Self::Length => {
                "Length"
            },
            Self::Level => {
                "Level"
            },
            Self::LinearMassDensityOfTextileThread => {
                "LinearMassDensityOfTextileThread"
            },
            Self::LineicNumber => {
                "LineicNumber"
            },
            Self::LumIntensityDensity => {
                "LumIntensityDensity"
            },
            Self::LuminousFlux => {
                "LuminousFlux"
            },
            Self::LuminousIntensity => {
                "LuminousIntensity"
            },
            Self::MagneticFieldIntensity => {
                "MagneticFieldIntensity"
            },
            Self::MagneticFlux => {
                "MagneticFlux"
            },
            Self::MagneticFluxDensity => {
                "MagneticFluxDensity"
            },
            Self::MagneticPermeability => {
                "MagneticPermeability"
            },
            Self::MagneticTension => {
                "MagneticTension"
            },
            Self::Mass => {
                "Mass"
            },
            Self::MassConcentration => {
                "MassConcentration"
            },
            Self::MassFraction => {
                "MassFraction"
            },
            Self::MetabolicCostOfPhysicalActivity => {
                "MetabolicCostOfPhysicalActivity"
            },
            Self::Number => {
                "Number"
            },
            Self::PlaneAngle => {
                "PlaneAngle"
            },
            Self::Power => {
                "Power"
            },
            Self::PowerLevel => {
                "PowerLevel"
            },
            Self::Pressure => {
                "Pressure"
            },
            Self::PressureLevel => {
                "PressureLevel"
            },
            Self::ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance => {
                "ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance"
            },
            Self::ProcedureDefinedAmountOfAProteinSubstance => {
                "ProcedureDefinedAmountOfAProteinSubstance"
            },
            Self::ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard => {
                "ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard"
            },
            Self::ProcedureDefinedAmountOfAnAntigenSubstance => {
                "ProcedureDefinedAmountOfAnAntigenSubstance"
            },
            Self::ProcedureDefinedAmountOfTheMajorAllergenOfRagweed => {
                "ProcedureDefinedAmountOfTheMajorAllergenOfRagweed"
            },
            Self::Radioactivity => {
                "Radioactivity"
            },
            Self::RefractionOfALens => {
                "RefractionOfALens"
            },
            Self::RefractionOfAPrism => {
                "RefractionOfAPrism"
            },
            Self::SedimentationCoefficient => {
                "SedimentationCoefficient"
            },
            Self::SignalTransmissionRate => {
                "SignalTransmissionRate"
            },
            Self::Slope => {
                "Slope"
            },
            Self::SolidAngle => {
                "SolidAngle"
            },
            Self::Temperature => {
                "Temperature"
            },
            Self::Time => {
                "Time"
            },
            Self::Unclassified => {
                "Unclassified"
            },
            Self::Velocity => {
                "Velocity"
            },
            Self::ViewAreaInMicroscope => {
                "ViewAreaInMicroscope"
            },
            Self::Volume => {
                "Volume"
            },
            Self::XRayAttenuation => {
                "XRayAttenuation"
            },
        };

        write!(formatter, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_display() {
        let a = format!("{}", Property::Acceleration);
        assert_eq!(&a, "Acceleration");
    }
}
