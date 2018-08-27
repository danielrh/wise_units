//! These tests are for the dynamically generated `symbol_parser` module.
//!
#[cfg(test)]
mod atom_test {
    use num_help::{BR_0, BR_1, BR_PI, ToFloat};
    use parser::{
        Atom, Classification, Composable, Composition, Dimension, Prefix, Term, UcumSymbol,
    };
    use ucum_unit::UcumUnit;

    macro_rules! validate_definition {
        (
            $test_name:ident,
            $atom_name:ident,
            $expected_value:expr,
            $($expected_term:expr),+
        ) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$atom_name;
                let expected = vec![$($expected_term),+];

                assert_eq!(atom.definition().value().clone(), $expected_value);
                assert_eq!(atom.definition().terms(), expected.as_slice());
            }
        };
    }

    macro_rules! validate_scalar {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$variant;
                assert_eq!(atom.scalar(), $value);
            }
        };
    }

    macro_rules! validate_scalar_as_float {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$variant;
                assert_eq!(atom.scalar().to_float().expect("Unable to convert to float"), $value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name:ident, $variant:ident, $value:expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $variant, $value);
             )+
        };
    }

    macro_rules! validate_scalars_as_float {
        ($($test_name:ident, $variant:ident, $value:expr);+ $(;)*) => {
            $(
                validate_scalar_as_float!($test_name, $variant, $value);
             )+
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$variant;
                assert_eq!(atom.magnitude(), $value);
            }
        };
    }

    macro_rules! validate_magnitudes {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_magnitude!($test_name, $variant, $value);
             )+
        };
    }

    macro_rules! validate_composition {
        (
            @insert
            $composition:expr,
            $dimension_variant:ident: $value:expr
        ) => {
            $composition.insert(Dimension::$dimension_variant, $value);
        };

        (
            $test_name:ident,
            $atom_variant:ident,
            $($dimension_variant:ident: $value:expr),+
        ) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$atom_variant;
                let mut composition = Composition::default();
                $(
                    validate_composition!(@insert composition, $dimension_variant: $value);
                 )+
                    assert_eq!(atom.composition(), composition);
            }
        };
    }

    #[test]
    fn validate_classification_si() {
        let atoms = vec![
            Atom::Candela,
            Atom::Coulomb,
            Atom::DegreeCelsius,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
            Atom::Mole,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Si);
        }
    }

    #[test]
    fn validate_classification_us_lengths() {
        let atoms = vec![Atom::AcreUS, Atom::FootUS, Atom::RodUS];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::UsLengths);
        }
    }

    #[test]
    fn validate_classification_iso1000() {
        let atoms = vec![Atom::Are, Atom::Degree, Atom::Liter];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Iso1000);
        }
    }

    #[test]
    fn validate_classification_heat() {
        let atoms = vec![Atom::DegreeFahrenheit, Atom::DegreeReaumur];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Heat);
        }
    }

    #[test]
    fn validate_classification_us_volumes() {
        let atoms = vec![
            Atom::FluidOunceUS,
            Atom::GillUS,
            Atom::QuartUS,
            Atom::QueenAnnesWineGallonUS,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::UsVolumes);
        }
    }

    #[test]
    fn validate_classification_intcust() {
        let atoms = vec![Atom::InchInternational, Atom::FootInternational];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Intcust);
        }
    }

    #[test]
    fn validate_classification_dimless() {
        let atoms = vec![
            Atom::PartsPerBillion,
            Atom::PartsPerMillion,
            Atom::PartsPerThousand,
            Atom::PartsPerTrillion,
            Atom::Percent,
            Atom::TheNumberPi,
            Atom::TheNumberTenForArbitraryPowersCaret,
            Atom::TheNumberTenForArbitraryPowersStar,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Dimless);
        }
    }

    #[test]
    fn validate_classification_chemical() {
        let atoms = vec![Atom::PH];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Chemical);
        }
    }

    #[test]
    fn validate_classification_clinical() {
        let atoms = vec![Atom::PrismDiopter];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Clinical);
        }
    }

    #[test]
    fn validate_definitions_base_atoms() {
        let base_atoms = vec![
            Atom::Candela,
            Atom::Coulomb,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
        ];
        let terms = vec![term!()];

        for base_atom in base_atoms {
            assert_eq!(base_atom.definition().value(), &*BR_1);
            assert_eq!(base_atom.definition().terms(), terms.as_slice());
        }
    }

    validate_definition!(
        validate_definition_acre_us,
        AcreUS,
        big_rational_raw!(160),
        term!(RodUS, exponent: 2)
    );

    validate_definition!(
        validate_definition_are,
        Are,
        big_rational_raw!(100),
        term!(Meter, exponent: 2)
    );

    validate_definition!(
        validate_definition_degree,
        Degree,
        big_rational_raw!(2),
        term!(TheNumberPi),
        term!(Radian),
        term!(factor: 360, exponent: -1)
    );
    validate_definition!(
        validate_definition_degree_celsius,
        DegreeCelsius,
        BR_1.clone(),
        term!(Kelvin)
    );
    validate_definition!(
        validate_definition_degree_fahrenheit,
        DegreeFahrenheit,
        big_rational_raw!(5),
        term!(Kelvin),
        term!(factor: 9, exponent: -1)
    );
    validate_definition!(
        validate_definition_degree_reaumur,
        DegreeReaumur,
        big_rational_raw!(5),
        term!(Kelvin),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_fluid_ounce_us,
        FluidOunceUS,
        BR_1.clone(),
        term!(GillUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_foot_international,
        FootInternational,
        big_rational_raw!(12),
        term!(InchInternational)
    );
    validate_definition!(
        validate_definition_inch_international,
        InchInternational,
        big_rational_raw!(254, 100),
        term!(Centi, Meter)
    );
    validate_definition!(
        validate_definition_foot_us,
        FootUS,
        big_rational_raw!(1200),
        term!(Meter),
        term!(factor: 3937, exponent: -1)
    );
    validate_definition!(
        validate_definition_gill_us,
        GillUS,
        BR_1.clone(),
        term!(PintUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_liter,
        Liter,
        BR_1.clone(),
        term!(Deci, Meter, exponent: 3)
    );
    validate_definition!(
        validate_definition_mole,
        Mole,
        big_rational_raw!(bytes "602213670000000000000000"),
        term!(TheNumberTenForArbitraryPowersStar, exponent: 23)
    );
    validate_definition!(
        validate_definition_parts_per_billion,
        PartsPerBillion,
        BR_1.clone(),
        term!(TheNumberTenForArbitraryPowersStar, exponent: -9)
    );
    validate_definition!(
        validate_definition_parts_per_million,
        PartsPerMillion,
        BR_1.clone(),
        term!(TheNumberTenForArbitraryPowersStar, exponent: -6)
    );
    validate_definition!(
        validate_definition_parts_per_thousand,
        PartsPerThousand,
        BR_1.clone(),
        term!(TheNumberTenForArbitraryPowersStar, exponent: -3)
    );
    validate_definition!(
        validate_definition_ph,
        PH,
        BR_1.clone(),
        term!(Mole),
        term!(Liter, exponent: -1)
    );
    validate_definition!(
        validate_definition_pint_us,
        PintUS,
        BR_1.clone(),
        term!(QuartUS),
        term!(factor: 2, exponent: -1)
    );
    validate_definition!(
        validate_definition_prism_diopter,
        PrismDiopter,
        BR_1.clone(),
        term!(Radian)
    );
    validate_definition!(validate_definition_bel_watt, BelWatt, BR_1.clone(), term!(Watt));
    validate_definition!(
        validate_definition_quart_us,
        QuartUS,
        BR_1.clone(),
        term!(QueenAnnesWineGallonUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_metric_cup,
        MetricCup,
        big_rational_raw!(240),
        term!(Milli, LiterSecondary)
    );
    validate_definition!(
        validate_definition_queen_annes_wine_gallon_us,
        QueenAnnesWineGallonUS,
        big_rational_raw!(231),
        term!(InchInternational, exponent: 3)
    );
    validate_definition!(validate_definition_rod_us, RodUS, big_rational_raw!(33, 2), term!(FootUS));
    validate_definition!(
        validate_definition_the_number_pi,
        TheNumberPi,
        BR_PI.clone(),
        term!()
    );
    validate_definition!(
        validate_definition_the_number_ten_for_arbitrary_powers_caret,
        TheNumberTenForArbitraryPowersCaret,
        big_rational_raw!(10),
        term!()
    );
    validate_definition!(
        validate_definition_the_number_ten_for_arbitrary_powers_star,
        TheNumberTenForArbitraryPowersStar,
        big_rational_raw!(10),
        term!()
    );

    // Composition tests
    validate_composition!(validate_composition_candela, Candela, LuminousIntensity: 1);
    validate_composition!(validate_composition_coulomb, Coulomb, ElectricCharge: 1);
    validate_composition!(validate_composition_gram, Gram, Mass: 1);
    validate_composition!(validate_composition_kelvin, Kelvin, Temperature: 1);
    validate_composition!(validate_composition_meter, Meter, Length: 1);
    validate_composition!(validate_composition_radian, Radian, PlaneAngle: 1);
    validate_composition!(validate_composition_second, Second, Time: 1);

    validate_composition!(validate_composition_acre_british, AcreBritish, Length: 2);
    validate_composition!(validate_composition_acre_us, AcreUS, Length: 2);
    validate_composition!(validate_composition_are, Are, Length: 2);
    validate_composition!(validate_composition_astronomic_unit, AstronomicUnit, Length: 1);
    validate_composition!(validate_composition_bar, Bar, Length: -1, Mass: 1, Time: -2);
    validate_composition!(validate_composition_barrel_us, BarrelUS, Length: 3);
    validate_composition!(validate_composition_becquerel, Becquerel, Time: -1);
    validate_composition!(validate_composition_biot, Biot, ElectricCharge: 1, Time: -1);
    validate_composition!(validate_composition_board_foot_international, BoardFootInternational, Length: 3);
    validate_composition!(
        validate_composition_boltzmann_constant,
        BoltzmannConstant,
        Length: 2, Mass: 1, Temperature: -1, Time: -2
    );
    validate_composition!(
        validate_composition_british_thermal_unit_at_39f,
        BritishThermalUnitAt39F,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_bushel_british,
        BushelBritish,
        Length: 3
    );
    validate_composition!(
        validate_composition_bushel_us,
        BushelUS,
        Length: 3
    );
    validate_composition!(
        validate_composition_calorie_at_15c,
        CalorieAt15C,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_calorie_at_20c,
        CalorieAt20C,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_international_table_calorie,
        InternationalTableCalorie,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_pound_av,
        PoundAvoirdupois,
        Mass: 1
    );

    // Scalar tests
    validate_scalars!(
        validate_scalar_candela, Candela, BR_1.clone();
        validate_scalar_coulomb, Coulomb, BR_1.clone();
        validate_scalar_gram, Gram, BR_1.clone();
        validate_scalar_kelvin, Kelvin, BR_1.clone();
        validate_scalar_meter, Meter, BR_1.clone();
        validate_scalar_radian, Radian, BR_1.clone();
        validate_scalar_second, Second, BR_1.clone();

        validate_scalar_are, Are, big_rational_raw!(100);
        validate_scalar_bar, Bar, big_rational_raw!(100_000_000);
        validate_scalar_becquerel, Becquerel, BR_1.clone();
        validate_scalar_biot, Biot, big_rational_raw!(10);
        validate_scalar_btu_at_39f, BritishThermalUnitAt39F, big_rational_raw!(1_059_670);

        validate_scalar_calorie_th, ThermochemicalCalorie, big_rational_raw!(4184);
        validate_scalar_calorie, Calorie, big_rational_raw!(4184);
        validate_scalar_calorie_nutrition_label, NutritionLabelCalories, big_rational_raw!(4_184_000);
        validate_scalar_curie, Curie, big_rational_raw!(37_000_000_000);

        validate_scalar_day, Day, big_rational_raw!(86_400);
        validate_scalar_degree_rankine, DegreeRankine, big_rational_raw!(5, 9);
        validate_scalar_dyne, Dyne, big_rational_raw!(1, 100);

        validate_scalar_erg, Erg, big_rational_raw!(1, 1000);

        validate_scalar_farad, Farad, big_rational_raw!(1, 100);
        validate_scalar_fathom_international, FathomInternational, big_rational_raw!(1143, 625);
        validate_scalar_foot_international, FootInternational, big_rational_raw!(381, 1250);
        validate_scalar_foot_us, FootUS, big_rational_raw!(1200, 3937);

        validate_scalar_gal, Gal, big_rational_raw!(1, 100);
        validate_scalar_gauss, Gauss, big_rational_raw!(1, 10);
        validate_scalar_gram_percent, GramPercent, big_rational_raw!(10_000);
        validate_scalar_gray, Gray, big_rational_raw!(1);

        validate_scalar_hand_international, HandInternational, big_rational_raw!(127, 1250);
        validate_scalar_hertz, Hertz, big_rational_raw!(1);
        validate_scalar_henry, Henry, big_rational_raw!(1_000);
        validate_scalar_hour, Hour, big_rational_raw!(3600);

        validate_scalar_inch_international, InchInternational, big_rational_raw!(127, 5000);
        validate_scalar_inch_us, InchUS, big_rational_raw!(100, 3937);

        validate_scalar_joule, Joule, big_rational_raw!(1000);

        validate_scalar_kayser, Kayser, big_rational_raw!(100);
        validate_scalar_knot_international, KnotInternational, big_rational_raw!(463, 900);

        validate_scalar_light_year, LightYear, big_rational_raw!(94_607_304_725_808);
        validate_scalar_liter, Liter, big_rational_raw!(1, 100);
        validate_scalar_lumen, Lumen, big_rational_raw!(1);
        validate_scalar_lux, Lux, big_rational_raw!(1);

        validate_scalar_maxwell, Maxwell, big_rational_raw!(1, 100_000);
        validate_scalar_mean_gregorian_month, MeanGregorianMonth, big_rational_raw!(2_629_746);
        validate_scalar_mean_gregorian_year, MeanGregorianYear, big_rational_raw!(31_556_952);
        validate_scalar_mean_julian_month, MeanJulianMonth, big_rational_raw!(2_629_800);
        validate_scalar_mean_julian_year, MeanJulianYear, big_rational_raw!(31_557_600);
        validate_scalar_metric_cup, MetricCup, big_rational_raw!(3, 12_500);
        validate_scalar_metric_fluid_ounce, MetricFluidOunce, big_rational_raw!(3, 100_000);
        validate_scalar_metric_tablespoon, MetricTablespoon, big_rational_raw!(3, 200_000);
        validate_scalar_metric_teaspoon, MetricTeaspoon, big_rational_raw!(1, 200_000);
        validate_scalar_mole, Mole, big_rational_raw!(bytes "602213670000000000000000");
        validate_scalar_month, Month, big_rational_raw!(2_629_800);

        validate_scalar_nautical_mile_internationa, NauticalMileInternational, big_rational_raw!(1852);
        validate_scalar_newton, Newton, big_rational_raw!(1000);
        validate_scalar_ohm, Ohm, big_rational_raw!(1000);
        validate_scalar_ounce_m, MetricOunce, big_rational_raw!(28);

        validate_scalar_parsec, Parsec, big_rational_raw!(bytes "30_856_780_000_000_000");
        validate_scalar_parts_per_billion, PartsPerBillion, big_rational_raw!(1, 1_000_000_00);
        validate_scalar_parts_per_million, PartsPerMillion, big_rational_raw!(1, 1_000_000);
        validate_scalar_parts_per_thousand, PartsPerThousand, big_rational_raw!(1, 1_000);
        validate_scalar_parts_per_trillion, PartsPerTrillion, big_rational_raw!(1, bytes "1_000_000_000_000");
        validate_scalar_pascal, Pascal, big_rational_raw!(1_000);
        validate_scalar_percent, Percent, big_rational_raw!(1, 100);
        validate_scalar_phot, Phot, big_rational_raw!(1, 1000);
        validate_scalar_pied, Pied, big_rational_raw!(203, 625);
        validate_scalar_pouce, Pouce, big_rational_raw!(203, 7500);
        validate_scalar_poise, Poise, big_rational_raw!(100);
        validate_scalar_printers_pica, PrintersPica, big_rational_raw!(127, 30_000);
        validate_scalar_protein_nitrogen_unit, ProteinNitrogenUnit, big_rational_raw!(1);

        validate_scalar_ramdens_chain_us, RamdensChainUS, big_rational_raw!(120_000, 3937);
        validate_scalar_radiation_absorbed_dose, RadiationAbsorbedDose, big_rational_raw!(1, 100);
        validate_scalar_radiation_equivalent_man, RadiationEquivalentMan, big_rational_raw!(1, 100);
        validate_scalar_rod_us, RodUS, big_rational_raw!(19_800, 3937);

        validate_scalar_siemens, Siemens, big_rational_raw!(1, 1000);
        validate_scalar_sievert, Sievert, BR_1.clone();
        validate_scalar_standard_atomsphere, StandardAtmosphere, big_rational_raw!(101_325_000);
        validate_scalar_steradian, Steradian, BR_1.clone();
        validate_scalar_stilb, Stilb, big_rational_raw!(10_000);
        validate_scalar_stokes, Stokes, big_rational_raw!(1, 10000);

        validate_scalar_tesla, Tesla, big_rational_raw!(1000);
        validate_scalar_the_number_ten_for_arbitrary_powers_caret, TheNumberTenForArbitraryPowersCaret, big_rational_raw!(10);
        validate_scalar_the_number_ten_for_arbitrary_powers_star, TheNumberTenForArbitraryPowersStar, big_rational_raw!(10);
        validate_scalar_tonne, Tonne, big_rational_raw!(1_000_000);

        validate_scalar_velocity_of_light, VelocityOfLight, big_rational_raw!(299_792_458);
        validate_scalar_volt, Volt, big_rational_raw!(1000);
        validate_scalar_watt, Watt, big_rational_raw!(1000);
        validate_scalar_weber, Weber, big_rational_raw!(1000);
        validate_scalar_week, Week, big_rational_raw!(604_800);
        validate_scalar_yard_international, YardInternational, big_rational_raw!(1143, 1250);
        validate_scalar_yard_us, YardUS, big_rational_raw!(3600, 3937);
        validate_scalar_year, Year, big_rational_raw!(31_557_600);

        validate_special_scalar_ph, PH, BR_0.clone();
    );

    validate_scalars_as_float!(
        validate_scalar_acre_br, AcreBritish, 4046.850_049_400_268_7;
        validate_scalar_acre_us, AcreUS, 4046.872_609_874_252;

        validate_scalar_astronomic_unit, AstronomicUnit, 149_597_870_691.0;
        validate_scalar_barrel_us, BarrelUS, 0.158_987_294_928;
        validate_scalar_board_foot_international, BoardFootInternational, 0.002_359_737_216;
        validate_scalar_boltzmann_constant, BoltzmannConstant, 1.380_658e-20;
        validate_scalar_bushel_br, BushelBritish, 0.036_368_72;
        validate_scalar_bushel_us, BushelUS, 0.035_239_070_166_88;

        validate_scalar_calorie_at_20c, CalorieAt20C, 4181.9;
        validate_scalar_calorie_at_15c, CalorieAt15C, 4185.8;
        validate_scalar_calorie_it, InternationalTableCalorie, 4186.8;
        validate_scalar_calorie_m, MeanCalorie, 4190.02;
        validate_scalar_cicero, Cicero, 0.004_511_111_111_111_111;
        validate_scalar_circular_mil_international, CircularMilInternational, 1_217_369_588.005_220_4;
        validate_scalar_cord_international, CordInternational, 3.624_556_363_776;
        validate_scalar_cord_us, CordUS, 3.624_556_363_776;
        validate_scalar_cubic_foot_international, CubicFootInternational, 0.028_316_846_592;
        validate_scalar_cubic_inch_international, CubicInchInternational, 1.638_706_4e-05;
        validate_scalar_cubic_yard_international, CubicYardInternational, 0.764_554_857_984;
        validate_scalar_cup_us, CupUS, 0.000_236_588_236_5;

        validate_scalar_degree, Degree, 0.0174_532_925_199_432_95;
        validate_scalar_degree_minute, MinuteAngle, 0.000_290_888_208_665_721_6;
        validate_scalar_degree_second, SecondAngle, 4.848_136_811_095_36e-06;
        validate_scalar_didot, Didot, 0.000_375_925_925_925_925_93;
        validate_scalar_dram_ap, DramApothecaries, 3.887_934_6;
        validate_scalar_dram_av, DramAvoirdupois, 1.771_845_195_312_5;
        validate_scalar_dry_pint_us, DryPintUS, 0.000_550_610_471_357_5;
        validate_scalar_dry_quart_us, DryQuartUS, 0.001_101_220_942_715;

        validate_scalar_electron_mass, ElectronMass, 9.109_389_7e-28;
        validate_scalar_electron_vold, Electronvolt, 1.602_177_33e-16;
        validate_scalar_elementary_charge, ElementaryCharge, 1.60217733e-19;
        validate_scalar_equivalents, Equivalents, 6.0221367e+23;

        validate_scalar_fathom_br, FathomBritish, 1.828_798_56;
        validate_scalar_fathom_us, FathomUS, 1.828_803_657_607_315_2;
        validate_scalar_fluid_dram_br, FluidDramBritish, 3.551_632_812_5e-06;
        validate_scalar_fluid_dram_us, FluidDramUS, 3.696_691_195_312_5e-06;
        validate_scalar_fluid_ounce_br, FluidOunceBritish, 2.841_306_25e-05;
        validate_scalar_fluid_ounce_us, FluidOunceUS, 2.95735295625e-05;
        validate_scalar_foot_br, FootBritish, 0.304_799_76;
        validate_scalar_furlong_us, FurlongUS, 201.16840233680466;

        validate_scalar_gallon_br, GallonBritish, 0.004_546_09;
        validate_scalar_gilbert, Gilbert, 0.795_774_715_459_476_8;
        validate_scalar_gill_br, GillBritish, 0.000_142_065_312_5;
        validate_scalar_gill_us, GillUS, 0.000_118_294_118_25;
        validate_scalar_gon, Gon, 0.015_707_963_267_948_967;
        validate_scalar_gram_force, GramForce, 9.806_65;
        validate_scalar_grain, Grain, 0.064_798_91;
        validate_scalar_gunters_chain_br, GuntersChainBritish, 20.116_784_16;
        validate_scalar_gunters_chain_us, GuntersChainUS, 20.116_840_233_680_467;

        validate_scalar_historical_winchester_gallon, HistoricalWinchesterGallon, 0.004_404_883_770_86;
        validate_scalar_horsepower, Horsepower, 745_699.871_582_270_3;

        validate_scalar_inch_br, InchBritish, 0.025_399_98;

        validate_scalar_knot_br, KnotBritish, 0.514_772_928;

        validate_scalar_lambert, Lambert, 31_415.926_535_897_932;
        validate_scalar_long_hundredweight_av, LongHunderdweightAvoirdupois, 50_802.345_44;
        validate_scalar_long_ton_av, LongTonAvoirdupois, 1_016_046.908_8;
        validate_scalar_ligne, Ligne, 0.002_255_555_555_555_555_4;
        validate_scalar_line, Line, 0.002_116_666_666_666_667;
        validate_scalar_link_for_gunters_chain_br, LinkForGuntersChainBritish, 0.201_167_841_6;
        validate_scalar_link_for_gunters_chain_us, LinkForGuntersChainUS, 0.201_168_402_336_804_66;

        validate_scalar_mil_international, MilInternational, 2.54e-05;
        validate_scalar_mil_us, MilUS, 2.540_005_080_010_16e-05;
        validate_scalar_mile_br, MileBritish, 1_609.342_732_8;
        validate_scalar_mile_international, MileInternational, 1_609.344;
        validate_scalar_mile_us, MileUS, 1_609.347_218_694_437_3;
        validate_scalar_minim_br, MinimBritish, 5.919_388_020_833_333_4e-08;
        validate_scalar_minim_us, MinimUS, 6.161_151_992_187_5e-08;

        validate_scalar_nautical_mile_br, NauticalMileBritish, 1_853.182_540_8;

        validate_scalar_oersted, Oersted, 79.577_471_545_947_67;
        validate_scalar_ounce_ap, OunceApothecaries, 31.103_476_8;
        validate_scalar_ounce_av, OunceAvoirdupois, 28.349_523_125;
        validate_scalar_ounce_tr, OunceTroy, 31.103_476_8;

        validate_scalar_pace_br, PaceBritish, 0.761_999_4;
        validate_scalar_peck_br, PeckBritish, 0.009_092_18;
        validate_scalar_peck_us, PeckUS, 0.008_809_767_541_72;
        validate_scalar_permeability_of_vacuum, PermeabilityOfVacuum, 0.001_256_637_061_435_917_5;
        validate_scalar_permittivity_of_vacuum, PermittivityOfVacuum, 8.854_187_817e-15;
        validate_scalar_pica, Pica, 0.004_233_333_333_333_334;
        validate_scalar_pint_br, PintBritish, 0.000_568_261_25;
        validate_scalar_pint_us, PintUS, 0.000_473_176_473;
        validate_scalar_planck_constant, PlanckConstant, 6.626_075_5e-31;
        validate_scalar_point, Point, 0.000_352_777_777_777_777_76;
        validate_scalar_pound_ap, PoundApothecaries, 373.241_721_6;
        validate_scalar_pound_av, PoundAvoirdupois, 453.592_37;
        validate_scalar_pound_tr, PoundTroy, 373.241_721_6;
        validate_scalar_pound_force, PoundForceAvoirdupois, 4448.221_615_260_5;
        validate_scalar_printers_point, PrintersPoint, 0.000_351_459_8;
        validate_scalar_proton_mass, ProtonMass, 1.672_623_100_000_000_2e-24;

        validate_scalar_quart_br, QuartBritish, 0.001_136_522_5;
        validate_scalar_quart_us, QuartUS, 0.000_946_352_946;
        validate_scalar_queen_annes_wine_gallon, QueenAnnesWineGallonUS, 0.003_785_411_784;

        validate_scalar_rod_br, RodBritish, 5.029_196_04;
        validate_scalar_roentgen, Roentgen, 2.58e-07;

        validate_scalar_scruple_ap, ScrupleApothecaries, 1.295_978_2;
        validate_scalar_section, Section, 2_589_998.470_319_521;
        validate_scalar_short_hundredweight_av, ShortHundredweightAvoirdupois, 45_359.237;
        validate_scalar_short_ton_av, ShortTonAvoirdupois, 907_184.74;
        validate_scalar_square_foot_international, SquareFootInternational, 0.092_903_04;
        validate_scalar_square_mile_us, SquareMileUS, 2_589_998.470_319_521;
        validate_scalar_square_rod_us, SquareRodUS, 25.292_953_811_714_074;
        validate_scalar_square_yard_international, SquareYardInternational, 0.836_127_36;
        validate_scalar_standard_acceleration_of_free_fall, StandardAccelerationOfFreeFall, 9.80665;
        validate_scalar_stone_av, StoneAvoirdupois, 6_350.293_18;
        validate_scalar_synodal_month, SynodalMonth, 2_551_442.976;

        validate_scalar_tablespoon_us, TablespoonUS, 1.478_676_478_125e-05;
        validate_scalar_teaspoon_us, TeaspoonUS, 4.928_921_593_75e-06;
        validate_scalar_the_number_pi, TheNumberPi, ::std::f64::consts::PI;
        validate_scalar_township, Township, 93_239_944.931_502_76;
        validate_scalar_tropical_year, TropicalYear, 31_556_925.216;

        validate_scalar_unified_atomic_mass_unit, UnifiedAtomicMassUnit, 1.660_540_2e-24;

        validate_scalar_yard_br, YardBritish, 0.914_399_28;

        validate_special_scalar_degree_celsius, DegreeCelsius, 274.15;
        validate_special_scalar_degree_fahrenheit, DegreeFahrenheit, 255.927_777_777_777_8;
        validate_special_scalar_degree_reaumur, DegreeReaumur, 274.400_000_000_000_03;
    );

    validate_magnitudes!(
        validate_magnitude_candela, Candela, BR_1.clone();
        validate_magnitude_coulomb, Coulomb, BR_1.clone();
        validate_magnitude_gram, Gram, BR_1.clone();
        validate_magnitude_kelvin, Kelvin, BR_1.clone();
        validate_magnitude_radian, Radian, BR_1.clone();
        validate_magnitude_second, Second, BR_1.clone();
        validate_magnitude_acre_us, AcreUS, BR_1.clone();
        validate_magnitude_are, Are, BR_1.clone();
        validate_magnitude_degree, Degree, BR_1.clone();
        validate_magnitude_degree_celsius, DegreeCelsius, BR_1.clone();
        validate_magnitude_degree_fahrenheit, DegreeFahrenheit, BR_1.clone();
        validate_magnitude_degree_reaumur, DegreeReaumur, BR_1.clone();
        validate_magnitude_fluid_ounce_us, FluidOunceUS, BR_1.clone();
        validate_magnitude_foot_us, FootUS, BR_1.clone();
        validate_magnitude_foot_international, FootInternational, BR_1.clone();
        validate_magnitude_gill_us, GillUS, BR_1.clone();
        validate_magnitude_inch_international, InchInternational, BR_1.clone();
        validate_magnitude_liter, Liter, BR_1.clone();
        validate_magnitude_mole, Mole, BR_1.clone();
        validate_magnitude_parts_per_billion, PartsPerBillion, BR_1.clone();
        validate_magnitude_parts_per_million, PartsPerMillion, BR_1.clone();
        validate_magnitude_parts_per_thousand, PartsPerThousand, BR_1.clone();
        validate_magnitude_percent, Percent, BR_1.clone();
        validate_magnitude_ph, PH, BR_1.clone();
        validate_magnitude_pint_us, PintUS, BR_1.clone();
        validate_magnitude_prism_diopter, PrismDiopter, BR_1.clone();
        validate_magnitude_quart_us, QuartUS, BR_1.clone();
        validate_magnitude_queen_annes_wine_gallon, QueenAnnesWineGallonUS, BR_1.clone();
        validate_magnitude_rod_us, RodUS, BR_1.clone();
        validate_magnitude_the_number_pi, TheNumberPi, BR_1.clone();
    );

    #[test]
    fn validate_display() {
        let atom = Atom::TheNumberPi;
        assert_eq!(&atom.to_string(), "[pi]")
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use parser::Atom;
        use serde_json;

        #[test]
        fn validate_serialization() {
            let j = serde_json::to_string(&Atom::BushelUS)
                .expect("Couldn't convert Atom to JSON String");

            assert_eq!("\"BushelUS\"", j);
        }

        #[test]
        fn validate_deserialization() {
            let k =
                serde_json::from_str("\"BushelUS\"").expect("Couldn't convert JSON String to Atom");

            assert_eq!(Atom::BushelUS, k);
        }
    }
}
