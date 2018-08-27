extern crate num_bigint;
extern crate num_rational;

#[macro_use]
extern crate wise_units;

use num_rational::BigRational;
use wise_units::{Convertible, Measurement};
use wise_units::num_help::{BR_0, BR_1, BR_PI};

macro_rules! validate_conversion {
    (
        $convert_from_test_name:ident,
        $convert_to_test_name:ident,
        $measurement_value:expr,
        $measurement_unit:expr,
        $convert_to_unit:expr,
        $expected_value:expr
    ) => {
        #[test]
        fn $convert_from_test_name() {
            let subject = Measurement::new_try_unit($measurement_value, $measurement_unit).unwrap();
            let converted = subject.convert_to($convert_to_unit).unwrap();
            assert_eq!(converted.value, $expected_value);
        }

        #[test]
        fn $convert_to_test_name() {
            let subject = Measurement::new_try_unit($expected_value, $convert_to_unit).unwrap();
            let converted = subject.convert_to($measurement_unit).unwrap();
            assert_eq!(converted.value, BigRational::from($measurement_value));
        }
    };
}

validate_conversion!(
    validate_conversion_meters_per_second_to_miles_per_hour_forward,
    validate_conversion_meters_per_second_to_miles_per_hour_back,
    1,
    "m/s",
    "[mi_i]/h",
    big_rational_raw!(3125, 1397)
);

validate_conversion!(
    validate_conversion_lb_per_acre_to_lb_per_m2_forward,
    validate_conversion_lb_per_acre_to_lb_per_m2_back,
    1,
    "[lb_av]/[acr_us]",
    "[lb_av]/m2",
    big_rational_raw!(bytes "15499969", bytes "62726400000")
);

validate_conversion!(
    validate_conversion_lb_per_acre_to_lb_per_ft2_forward,
    validate_conversion_lb_per_acre_to_lb_per_ft2_back,
    1,
    "[lb_av]/[acr_us]",
    "[lb_av]/[ft_i]2",
    big_rational_raw!(bytes "249999000001", bytes "10890000000000000")
);

validate_conversion!(
    validate_conversion_lb_to_ston_forward,
    validate_conversion_lb_to_ston_back,
    1,
    "[lb_av]",
    "[ston_av]",
    big_rational_raw!(1, 2000)
);

validate_conversion!(
    validate_conversion_m_per_s_to_ft_per_hour_forward,
    validate_conversion_m_per_s_to_ft_per_hour_back,
    1,
    "m/s",
    "[ft_i]/h",
    big_rational_raw!(1500000, 127)
);

validate_conversion!(
    validate_conversion_lb_to_oz_forward,
    validate_conversion_lb_to_oz_back,
    1,
    "[lb_av]",
    "[oz_av]",
    big_rational_raw!(16)
);

validate_conversion!(
    validate_conversion_g_per_cm3_to_mg_per_l_forward,
    validate_conversion_g_per_cm3_to_mg_per_l_back,
    1,
    "g/cm3",
    "mg/L",
    big_rational_raw!(1_000_000)
);

validate_conversion!(
    validate_conversion_meq_per_100cm3_to_meq_per_l_forward,
    validate_conversion_meq_per_100cm3_to_meq_per_l_back,
    1,
    "meq/100cm3",
    "meq/L",
    big_rational_raw!(1, 1000)
);

validate_conversion!(
    validate_conversion_percent_to_g_per_kg_forward,
    validate_conversion_percent_to_g_per_kg_back,
    1,
    "%",
    "g/kg",
    big_rational_raw!(10)
);

validate_conversion!(
    validate_conversion_percent_to_mg_per_kg_forward,
    validate_conversion_percent_to_mg_per_kg_back,
    1,
    "%",
    "mg/kg",
    big_rational_raw!(10_000)
);

validate_conversion!(
    validate_conversion_percent_to_ug_per_kg_forward,
    validate_conversion_percent_to_ug_per_kg_back,
    1,
    "%",
    "ug/kg",
    big_rational_raw!(10_000_000)
);

validate_conversion!(
    validate_conversion_ppm_to_in_per_ft_forward,
    validate_conversion_ppm_to_in_per_ft_back,
    1,
    "[ppm]",
    "[in_i]/[ft_i]",
    big_rational_raw!(3, 250_000)
);

validate_conversion!(
    validate_conversion_ppm_to_mg_per_kg_forward,
    validate_conversion_ppm_to_mg_per_kg_back,
    1,
    "[ppm]",
    "mg/kg",
    BR_1.clone()
);

validate_conversion!(
    validate_conversion_ppm_to_ug_per_kg_forward,
    validate_conversion_ppm_to_ug_per_kg_back,
    1,
    "[ppm]",
    "ug/kg",
    big_rational_raw!(1000)
);

validate_conversion!(
    validate_conversion_ug_per_kg_to_in_per_ft_forward,
    validate_conversion_ug_per_kg_to_in_per_ft_back,
    1,
    "ug/kg",
    "[in_i]/[ft_i]",
    big_rational_raw!(3, 250000000)
);

validate_conversion!(
    validate_conversion_lb_acre_per_har_to_kg_forward,
    validate_conversion_lb_acre_per_har_to_kg_back,
    1,
    "[lb_av].[acr_us]/har",
    "kg",
    big_rational_raw!(bytes "444565881837", bytes "2421870156250")
);

validate_conversion!(
    validate_conversion_unity_to_10power_forward,
    validate_conversion_unity_to_10power_back,
    500,
    "1",
    "10^",
    big_rational_raw!(50)
);

validate_conversion!(
    validate_conversion_unity_to_percent_forward,
    validate_conversion_unity_to_percent_back,
    500,
    "1",
    "%",
    big_rational_raw!(50_000)
);

validate_conversion!(
    validate_conversion_m_to_km_forward,
    validate_conversion_m_to_km_back,
    1,
    "m",
    "km",
    big_rational_raw!(1, 1000)
);

validate_conversion!(
    validate_conversion_m2_to_km2_forward,
    validate_conversion_m2_to_km2_back,
    1,
    "m2",
    "km2",
    big_rational_raw!(1, 1000_000)
);

validate_conversion!(
    validate_conversion_m2_per_s_to_km2_per_s_forward,
    validate_conversion_m2_per_s_to_km2_per_s_back,
    1,
    "m2/s",
    "km2/s",
    big_rational_raw!(1, 1000_000)
);

validate_conversion!(
    validate_conversion_s_per_m2_to_s_per_km2_forward,
    validate_conversion_s_per_m2_to_s_per_km2_back,
    1,
    "s/m2",
    "s/km2",
    big_rational_raw!(1_000_000)
);

validate_conversion!(
    validate_conversion_pi_m2_to_m2_forward,
    validate_conversion_pi_m2_to_m2_back,
    5,
    "[pi].m2",
    "m2",
    5 * &*BR_PI
);

validate_conversion!(
    validate_conversion_percent_to_10power_forward,
    validate_conversion_percent_to_10power_back,
    500,
    "%",
    "10^",
    big_rational_raw!(1, 2)
);

validate_conversion!(
    validate_conversion_pi_to_ppth_forward,
    validate_conversion_pi_to_ppth_back,
    1,
    "[pi]",
    "[ppth]",
    1000 * &*BR_PI
);

validate_conversion!(
    validate_conversion_l_to_m3_forward,
    validate_conversion_l_to_m3_back,
    2,
    "l",
    "m3",
    big_rational_raw!(1, 1000)
);

validate_conversion!(
    validate_special_conversion_cel_to_k_forward,
    validate_special_conversion_cel_to_k_back,
    25,
    "Cel",
    "K",
    big_rational_raw!(bytes "2622555134571315", bytes "8796093022208")
);

validate_conversion!(
    validate_special_conversion_degf_to_k_forward,
    validate_special_conversion_degf_to_k_back,
    big_rational_raw!(986, 100),
    "[degF]",
    "K",
    big_rational_raw!(bytes "2728108250837811", bytes "8796093022208")
);

validate_conversion!(
    validate_special_conversion_degf_to_cel_forward,
    validate_special_conversion_degf_to_cel_back,
    big_rational_raw!(986, 100),
    "[degF]",
    "Cel",
    big_rational_raw!(37)
);

validate_conversion!(
    validate_special_conversion_degre_to_k_forward,
    validate_special_conversion_degre_to_k_back,
    100,
    "[degRe]",
    "K",
    big_rational_raw!(bytes "3502164436792115", bytes "8796093022208")
);

validate_conversion!(
    validate_special_conversion_degre_to_cel_forward,
    validate_special_conversion_degre_to_cel_back,
    100,
    "[degRe]",
    "Cel",
    big_rational_raw!(125)
);

validate_conversion!(
    validate_special_conversion_deg_to_rad_forward,
    validate_special_conversion_deg_to_rad_back,
    180,
    "deg",
    "rad",
    BR_PI.clone()
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l1_forward,
    validate_special_conversion_ph_to_mol_per_l1_back,
    1,
    "[pH]",
    "mol/l",
    BR_0.clone()
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l2_forward,
    validate_special_conversion_ph_to_mol_per_l2_back,
    1,
    "[pH]",
    "mol/l",
    BR_0.clone()
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph1_forward,
    validate_special_conversion_mol_per_l_to_ph1_back,
    0,
    "mol/l",
    "[pH]",
    BR_1.clone()
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph2_forward,
    validate_special_conversion_mol_per_l_to_ph2_back,
    -1,
    "mol/l",
    "[pH]",
    big_rational_raw!(10)
);

// TODO: I don't think UCUM defines the conversion functions properly, based on
// https://en.wikipedia.org/wiki/Decibel_watt.
// validate_conversion!(
//     validate_special_conversion_bel_to_decibel_forward,
//     validate_special_conversion_bel_to_decibel_back,
//     2, "B", "dB",
//     big_rational_raw!(20)
// );

// validate_conversion!(
//     validate_special_conversion_10bel_watt_to_bel_kilowatt_forward,
//     validate_special_conversion_10bel_watt_to_bel_kilowatt_back,
//     2, "10B[W]", "B[kW]",
//     big_rational_raw!(3)
// );

// validate_conversion!(
//     validate_special_conversion_milliwatt_to_decibelwatt_forward,
//     validate_special_conversion_milliwatt_to_decibelwatt_back,
//     1, "mW", "dB[W]",
//     big_rational_raw!(-30)
// );

validate_conversion!(
    validate_special_conversion_bel_to_neper_forward,
    validate_special_conversion_bel_to_neper_back,
    2,
    "B",
    "Np",
    big_rational_raw!(bytes "2592480341699211", bytes "562949953421312")
);

validate_conversion!(
    validate_special_conversion_bit_s_to_bit_forward,
    validate_special_conversion_bit_s_to_bit_back,
    2,
    "bit_s",
    "bit",
    big_rational_raw!(4)
);

validate_conversion!(
    validate_special_conversion_bel_watt_to_bel_kilowatt_forward,
    validate_special_conversion_bel_watt_to_bel_kilowatt_back,
    2,
    "B[W]",
    "B[kW]",
    big_rational_raw!(2)
);

validate_conversion!(
    validate_special_conversion_bel_watt_to_watt_forward,
    validate_special_conversion_bel_watt_to_watt_back,
    0,
    "B[W]",
    "W",
    big_rational_raw!(1)
);
