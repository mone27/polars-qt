#![allow(clippy::excessive_precision)]
use once_cell::sync::Lazy;

use super::conversion::*;

// #[allow(non_upper_case_globals)] // this physical constant needs to be lowercase
// const g0: f64 = 9.80665;

impl UnitRegistry {
    fn new_with_definitions() -> Self {
        let mut registry = UnitRegistry::new();
        registry.add_base_dimensions();
        registry.add_base_units();
        registry.add_angles();
        registry.add_information();
        registry.add_ratio();
        registry.add_length();
        registry.add_mass();
        registry.add_time();
        registry.add_area();
        registry.add_volume();
        registry.add_frequency();
        registry.add_wavenumber();
        registry.add_velocity();
        registry.add_volumetric_flow_rate();
        registry.add_acceleration();
        registry.add_force();
        registry.add_energy();
        registry.add_power();
        registry.add_momentum();
        registry.add_density();
        registry.add_pressure();
        registry.add_torque();
        registry.add_viscosity();
        registry.add_kinematic_viscosity();
        registry.add_fluidity();
        registry.add_amount_of_substance();
        registry.add_concentration();
        registry.add_catalytic_activity();
        registry.add_entropy();
        registry.add_molar_entropy();
        registry.add_charge();
        registry.add_radiation();
        registry.add_heat_transmission();
        registry.add_luminance();
        registry.add_luminous_flux();
        registry.add_illuminance();
        registry.add_intensity();
        registry.add_current();
        registry.add_electric_potential();
        registry.add_electric_field();
        registry.add_electric_displacement_field();
        // registry.add_reduced_electric_field(); ignore for now
        registry.add_resistance();
        registry.add_resistivity();
        registry.add_conductance();
        registry.add_capacitance();
        registry.add_magnetic_flux();
        registry.add_inductance();
        registry.add_magnetic_field();
        registry.add_magnetomotive_force();
        registry.add_magnetic_field_strength();
        registry.add_electric_dipole_moment();
        registry.add_electric_quadrupole_moment();
        registry.add_magnetic_dipole_moment();
        registry.add_refractive_index();
        registry.add_uscs_length_international();
        registry.add_uscs_length_survey();
        registry.add_uscs_dry_volume();
        registry.add_uscs_liquid_volume();
        registry.add_uscs_volume_other();
        registry.add_avoirdupois();
        registry.add_avoirdupois_uk();
        registry.add_avoirdupois_us();
        registry.add_troy();
        registry.add_apothecary();
        registry.add_imperial_volume();
        registry.add_printer();
        registry
    }

    fn add_base_dimensions(&mut self) {
        self.add_dimension_simple("length");
        self.add_dimension_simple("mass");
        self.add_dimension_simple("time");
        self.add_dimension_simple("current");
        self.add_dimension_simple("temperature");
        self.add_dimension(Dimension::new("amount", vec![]));
        self.add_dimension_simple("luminosity");
        self.add_dimension(Dimension::new("dimensionless", vec![]));
    }

    fn add_base_units(&mut self) {
        self.add_unit_simple("meter", "[length]");
        self.add_unit_simple("kilogram", "[mass]");
        self.add_unit_simple("second", "[time]");
        self.add_unit_simple("ampere", "[current]");
        self.add_unit_simple("kelvin", "[temperature]");
        self.add_unit_simple("mole", "[amount]");
        self.add_unit_simple("candela", "[luminosity]");
        self.add_unit_simple("radian", "[dimensionless]");
        self.add_unit_simple("bit", "[dimensionless]");
        self.add_unit_simple("count", "[dimensionless]");
    }
    fn add_angles(&mut self) {
        self.add_unit_deriv("turn", "[dimensionless]", 2.0 * std::f64::consts::PI, "radian");
        self.add_unit_deriv(
            "degree",
            "[dimensionless]",
            2.0 * std::f64::consts::PI / 360.0,
            "radian",
        );
        self.add_unit_deriv(
            "arcminute",
            "[dimensionless]",
            std::f64::consts::PI / (180.0 * 60.0),
            "radian",
        );
        self.add_unit_deriv(
            "arcsecond",
            "[dimensionless]",
            std::f64::consts::PI / (180.0 * 60.0 * 60.0),
            "radian",
        );
        self.add_unit_deriv(
            "milliarcsecond",
            "[dimensionless]",
            std::f64::consts::PI / (180.0 * 60.0 * 60.0 * 1000.0),
            "radian",
        );
        self.add_unit_deriv("grade", "[dimensionless]", std::f64::consts::PI / 200.0, "radian");
        self.add_unit_deriv("mil", "[dimensionless]", std::f64::consts::PI / 3200.0, "radian");
        self.add_unit_deriv("steradian", "[dimensionless]", 1.0, "radian");
        self.add_unit_deriv(
            "square_degree",
            "[dimensionless]",
            (std::f64::consts::PI / 180.0).powi(2),
            "steradian",
        );
    }
    fn add_information(&mut self) {
        self.add_unit_deriv("baud", "[dimensionless]", 1.0, "bit");
        self.add_unit_deriv("byte", "[dimensionless]", 8.0, "bit");
    }
    fn add_ratio(&mut self) {
        self.add_unit_deriv("percent", "[dimensionless]", 0.01, "count");
        self.add_unit_deriv("permille", "[dimensionless]", 0.001, "count");
        self.add_unit_deriv("ppm", "[dimensionless]", 1e-6, "count");
    }
    fn add_length(&mut self) {
        self.add_unit_deriv("angstrom", "[length]", 1e-10, "meter");
        self.add_unit_deriv("micron", "[length]", 1e-6, "meter");
        self.add_unit_deriv("fermi", "[length]", 1e-15, "meter");
        self.add_unit_deriv("light_year", "[length]", 9.4607e15, "meter");
        self.add_unit_deriv("astronomical_unit", "[length]", 1.495978707e11, "meter");
        self.add_unit_deriv("parsec", "[length]", 3.085677581e16, "meter");
        self.add_unit_deriv("nautical_mile", "[length]", 1852.0, "meter");
        self.add_unit_deriv("bohr", "[length]", 5.29177210903e-11, "meter");
        self.add_unit_deriv("planck_length", "[length]", 1.616255e-35, "meter");
    }

    fn add_mass(&mut self) {
        self.add_unit_deriv("metric_ton", "[mass]", 1e3, "kilogram");
        self.add_unit_deriv("unified_atomic_mass_unit", "[mass]", 1.66053906660e-27, "kilogram"); // 1 u = 1.66053906660e-27 kg
        self.add_unit_deriv("dalton", "[mass]", 1.66053906660e-27, "kilogram"); // 1 Da = 1.66053906660e-27 kg
        self.add_unit_deriv("grain", "[mass]", 64.79891e-6, "kilogram"); // 1 grain = 64.79891 mg
        self.add_unit_deriv("gamma_mass", "[mass]", 1e-9, "kilogram"); // 1 gamma = 1 microgram
        self.add_unit_deriv("carat", "[mass]", 200e-6, "kilogram"); // 1 carat = 200 mg
        self.add_unit_deriv("planck_mass", "[mass]", 2.176434e-8, "kilogram"); // Planck mass = 2.176434e-8 kg
    }

    fn add_time(&mut self) {
        self.add_unit_deriv("minute", "[time]", 60.0, "second");
        self.add_unit_deriv("hour", "[time]", 3600.0, "second"); // 60 * 60
        self.add_unit_deriv("day", "[time]", 86400.0, "second"); // 24 * 3600
        self.add_unit_deriv("week", "[time]", 604800.0, "second"); // 7 * 86400
        self.add_unit_deriv("fortnight", "[time]", 1209600.0, "second"); // 2 * 604800
        self.add_unit_deriv("year", "[time]", 31557600.0, "second"); // 365.25 * 86400
        self.add_unit_deriv("month", "[time]", 2629800.0, "second"); // 31557600 / 12
        self.add_unit_deriv("century", "[time]", 3155760000.0, "second"); // 100 * 31557600
        self.add_unit_deriv("millennium", "[time]", 31557600000.0, "second"); // 1000 * 31557600
        self.add_unit_deriv("eon", "[time]", 3.15576e16, "second"); // 1e9 * 31557600
        self.add_unit_deriv("shake", "[time]", 1e-8, "second");
        self.add_unit_deriv("svedberg", "[time]", 1e-13, "second");
        self.add_unit_deriv("atomic_unit_of_time", "[time]", 2.4188843265857e-17, "second"); // hbar / E_h
        self.add_unit_deriv("gregorian_year", "[time]", 31556952.0, "second"); // 365.2425 * 86400
        self.add_unit_deriv("sidereal_year", "[time]", 31558149.7632, "second"); // 365.256363004 * 86400
        self.add_unit_deriv("tropical_year", "[time]", 31556925.216, "second"); // 365.242190402 * 86400
        self.add_unit_deriv("common_year", "[time]", 31536000.0, "second"); // 365 * 86400
        self.add_unit_deriv("leap_year", "[time]", 31622400.0, "second"); // 366 * 86400
        self.add_unit_deriv("sidereal_day", "[time]", 86164.0905, "second"); // 86400 / 1.00273790935079524
        self.add_unit_deriv("sidereal_month", "[time]", 2360591.488, "second"); // 27.32166155 * 86400
        self.add_unit_deriv("tropical_month", "[time]", 2360587.328, "second"); // 27.321582 * 86400
        self.add_unit_deriv("synodic_month", "[time]", 2551442.592, "second"); // 29.530589 * 86400
        self.add_unit_deriv("planck_time", "[time]", 5.39116e-44, "second"); // (hbar * gravitational_constant / c ** 5) ** 0.5
    }

    fn add_area(&mut self) {
        self.add_dimension(self.try_get_dimension("[length]").pow(2).with_name("[area]"));
        self.add_unit(self.try_get_unit("meter").pow(2));
        self.add_unit_deriv("are", "[area]", 100.0, "meter^2");
        self.add_unit_deriv("barn", "[area]", 1e-28, "meter^2");
        self.add_unit_deriv("darcy", "[area]", 9.869233e-13, "meter^2"); // centipoise * centimeter^2 / (second * atmosphere)
        self.add_unit_deriv("hectare", "[area]", 10000.0, "meter^2"); // 100 * 100
    }

    fn add_volume(&mut self) {
        // Define m3 unit and volume dimension
        self.add_unit(self.try_get_unit("meter").pow(3));
        self.add_dimension(self.try_get_dimension("[length]").pow(3).with_name("[volume]"));

        self.add_unit_deriv("liter", "[volume]", 1e-3, "meter^3");
        self.add_unit_deriv("cubic_centimeter", "[volume]", 1e-6, "meter^3");
        self.add_unit_deriv("lambda", "[volume]", 1e-9, "meter^3");
        self.add_unit_deriv("stere", "[volume]", 1.0, "meter^3");
    }

    fn add_frequency(&mut self) {
        self.add_dimension(self.try_get_dimension("[time]").pow(-1).with_name("[frequency]"));
        self.add_unit(self.try_get_unit("second").pow(-1));
        self.add_unit_deriv("hertz", "[frequency]", 1.0, "second^-1");
        self.add_unit_deriv("revolutions_per_minute", "[frequency]", 1.0 / 60.0, "hertz");
        self.add_unit_deriv("revolutions_per_second", "[frequency]", 1.0, "hertz");
        self.add_unit_deriv("counts_per_second", "[frequency]", 1.0, "hertz");
    }

    fn add_wavenumber(&mut self) {
        self.add_dimension(self.try_get_dimension("[length]").pow(-1).with_name("[wavenumber]"));
        self.add_unit(self.try_get_unit("meter").pow(-1));
        self.add_unit_deriv("reciprocal_centimeter", "[wavenumber]", 100.0, "meter^-1");
    }

    fn add_velocity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[length]") / self.try_get_dimension("[time]")).with_name("[velocity]"),
        );
        self.add_unit(self.try_get_unit("meter") / self.try_get_unit("second"));
        self.add_unit_deriv("knot", "[velocity]", 1852.0 / 3600.0, "meter/second"); // nautical_mile / hour
        self.add_unit_deriv("mile_per_hour", "[velocity]", 1609.344 / 3600.0, "meter/second"); // mile / hour
        self.add_unit_deriv("kilometer_per_hour", "[velocity]", 1000.0 / 3600.0, "meter/second"); // kilometer / hour
        self.add_unit_deriv("kilometer_per_second", "[velocity]", 1000.0, "meter/second"); // kilometer / second
        self.add_unit_deriv("meter_per_second", "[velocity]", 1.0, "meter/second");
        self.add_unit_deriv("foot_per_second", "[velocity]", 0.3048, "meter/second");
        // foot / second
    }

    fn add_volumetric_flow_rate(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[volume]") / self.try_get_dimension("[time]")).with_name("[volumetric_flow_rate]"),
        );
        self.add_unit(self.try_get_unit("meter^3") / self.try_get_unit("second"));
        self.add_unit_deriv("sverdrup", "[volumetric_flow_rate]", 1e6, "meter^3/second");
    }

    fn add_acceleration(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[velocity]") / self.try_get_dimension("[time]")).with_name("[acceleration]"),
        );
        self.add_unit(self.try_get_unit("meter") / self.try_get_unit("second").pow(2));
        self.add_unit_deriv("galileo", "[acceleration]", 0.01, "meter/second^2");
        // centimeter / second^2
    }
    fn add_force(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[mass]") * self.try_get_dimension("[acceleration]")).with_name("[force]"),
        );
        self.add_unit(self.try_get_unit("kilogram") * self.try_get_unit("meter") / self.try_get_unit("second").pow(2));
        self.add_unit_deriv("newton", "[force]", 1.0, "kilogram*meter/second^2");
        self.add_unit_deriv("dyne", "[force]", 1e-5, "newton"); // gram * centimeter / second^2
        self.add_unit_deriv("force_kilogram", "[force]", 9.80665, "newton"); // g_0 * kilogram
        self.add_unit_deriv("force_gram", "[force]", 9.80665e-3, "newton"); // g_0 * gram
        self.add_unit_deriv("force_metric_ton", "[force]", 9.80665e3, "newton"); // g_0 * metric_ton
        self.add_unit_deriv("atomic_unit_of_force", "[force]", 8.23872206e-8, "newton");
        // E_h / a_0
    }

    fn add_energy(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[force]") * self.try_get_dimension("[length]")).with_name("[energy]"),
        );
        self.add_unit(self.try_get_unit("newton") * self.try_get_unit("meter"));
        self.add_unit_deriv("joule", "[energy]", 1.0, "newton*meter");
        self.add_unit_deriv("erg", "[energy]", 1e-7, "joule"); // dyne * centimeter
        self.add_unit_deriv("watt_hour", "[energy]", 3600.0, "joule"); // watt * hour
        self.add_unit_deriv("electron_volt", "[energy]", 1.602176634e-19, "joule"); // e * volt
        self.add_unit_deriv("rydberg", "[energy]", 2.1798723611035e-18, "joule"); // ℎ * c * R_inf
        self.add_unit_deriv("hartree", "[energy]", 4.3597447222071e-18, "joule"); // 2 * rydberg
        self.add_unit_deriv("calorie", "[energy]", 4.184, "joule"); // thermochemical_calorie
        self.add_unit_deriv("international_calorie", "[energy]", 4.1868, "joule"); // international_steam_table_calorie
        self.add_unit_deriv("fifteen_degree_calorie", "[energy]", 4.1855, "joule"); // cal_15
        self.add_unit_deriv("british_thermal_unit", "[energy]", 1055.056, "joule"); // Btu
        self.add_unit_deriv("international_british_thermal_unit", "[energy]", 1055.05585262, "joule"); // Btu_it
        self.add_unit_deriv(
            "thermochemical_british_thermal_unit",
            "[energy]",
            1054.35026444,
            "joule",
        ); // Btu_th
        self.add_unit_deriv("quadrillion_Btu", "[energy]", 1.055056e18, "joule"); // 1e15 * Btu
        self.add_unit_deriv("therm", "[energy]", 1.055056e8, "joule"); // 1e5 * Btu
        self.add_unit_deriv("US_therm", "[energy]", 1.054804e8, "joule"); // approximate
        self.add_unit_deriv("ton_TNT", "[energy]", 4.184e9, "joule"); // 1e9 * calorie
        self.add_unit_deriv("tonne_of_oil_equivalent", "[energy]", 4.1868e10, "joule"); // 1e10 * international_calorie
        self.add_unit_deriv("atmosphere_liter", "[energy]", 101.325, "joule"); // atmosphere * liter
    }

    fn add_power(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[energy]") / self.try_get_dimension("[time]")).with_name("[power]"),
        );
        self.add_dimension(
            (self.try_get_dimension("[energy]") / self.try_get_dimension("[time]")).with_name("[power]"),
        );
        self.add_unit(self.try_get_unit("joule") / self.try_get_unit("second"));
        self.add_unit_deriv("watt", "[power]", 1.0, "joule/second");
        self.add_unit_deriv("volt_ampere", "[power]", 1.0, "watt"); // volt * ampere
        self.add_unit_deriv("horsepower", "[power]", 745.69987158227022, "watt"); // 550 * foot * force_pound / second
        self.add_unit_deriv("boiler_horsepower", "[power]", 9812.5, "watt"); // 33475 * Btu / hour
        self.add_unit_deriv("metric_horsepower", "[power]", 735.49875, "watt"); // 75 * force_kilogram * meter / second
        self.add_unit_deriv("electrical_horsepower", "[power]", 746.0, "watt");
        self.add_unit_deriv("refrigeration_ton", "[power]", 3516.8528420667, "watt"); // 12e3 * Btu / hour
        self.add_unit_deriv("cooling_tower_ton", "[power]", 4396.0660525834, "watt"); // 1.25 * refrigeration_ton
        self.add_unit_deriv("standard_liter_per_minute", "[power]", 1.68875, "watt"); // atmosphere * liter / minute
        self.add_unit_deriv("conventional_watt_90", "[power]", 1.0000000000000002, "watt");
        // K_J90 ** 2 * R_K90 / (K_J ** 2 * R_K)
    }

    fn add_momentum(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[length]") * self.try_get_dimension("[mass]") / self.try_get_dimension("[time]"))
                .with_name("[momentum]"),
        );
        self.add_unit(self.try_get_unit("kilogram") * self.try_get_unit("meter") / self.try_get_unit("second"));
    }

    fn add_density(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[mass]") / self.try_get_dimension("[volume]")).with_name("[density]"),
        );
        self.add_unit(self.try_get_unit("kilogram") / self.try_get_unit("liter"));
        self.add_unit_deriv("mercury", "[density]", 13595.1, "kilogram/liter");
        self.add_unit_deriv("water", "[density]", 1.0, "kilogram/liter");
        self.add_unit_deriv("mercury_60F", "[density]", 13556.8, "kilogram/liter"); // approximate
        self.add_unit_deriv("water_39F", "[density]", 0.999972, "kilogram/liter"); // approximate
        self.add_unit_deriv("water_60F", "[density]", 0.999001, "kilogram/liter");
        // approximate
    }

    fn add_pressure(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[force]") / self.try_get_dimension("[area]")).with_name("[pressure]"),
        );
        self.add_unit(self.try_get_unit("newton") / self.try_get_unit("meter^2"));
        self.add_unit_deriv("pascal", "[pressure]", 1.0, "newton/meter^2");
        self.add_unit_deriv("barye", "[pressure]", 0.1, "pascal"); // dyne / centimeter^2
        self.add_unit_deriv("bar", "[pressure]", 1e5, "pascal");
        self.add_unit_deriv("technical_atmosphere", "[pressure]", 98066.5, "pascal"); // kilogram * g_0 / centimeter^2
        self.add_unit_deriv("torr", "[pressure]", 133.3223684211, "pascal"); // atm / 760
        self.add_unit_deriv("pound_force_per_square_inch", "[pressure]", 6894.757293168, "pascal"); // force_pound / inch^2
        self.add_unit_deriv("kip_per_square_inch", "[pressure]", 6894757.293168, "pascal"); // kip / inch^2
        self.add_unit_deriv("millimeter_Hg", "[pressure]", 133.322387415, "pascal"); // millimeter * Hg * g_0
        self.add_unit_deriv("centimeter_Hg", "[pressure]", 1333.22387415, "pascal"); // centimeter * Hg * g_0
        self.add_unit_deriv("inch_Hg", "[pressure]", 3386.389, "pascal"); // inch * Hg * g_0
        self.add_unit_deriv("inch_Hg_60F", "[pressure]", 3376.85, "pascal"); // inch * Hg_60F * g_0
        self.add_unit_deriv("inch_H2O_39F", "[pressure]", 249.082, "pascal"); // inch * water_39F * g_0
        self.add_unit_deriv("inch_H2O_60F", "[pressure]", 248.84, "pascal"); // inch * water_60F * g_0
        self.add_unit_deriv("foot_H2O", "[pressure]", 2989.06692, "pascal"); // foot * water * g_0
        self.add_unit_deriv("centimeter_H2O", "[pressure]", 98.0665, "pascal"); // centimeter * water * g_0
        self.add_unit_deriv("sound_pressure_level", "[pressure]", 20e-6, "pascal");
        // SPL
    }

    fn add_torque(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[force]") * self.try_get_dimension("[length]")).with_name("[torque]"),
        );
        self.add_unit(self.try_get_unit("newton") * self.try_get_unit("meter"));
        self.add_unit_deriv("foot_pound", "[torque]", 1.3558179483314004, "newton*meter");
        // foot * force_pound
    }

    fn add_viscosity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[pressure]") * self.try_get_dimension("[time]")).with_name("[viscosity]"),
        );
        self.add_unit(self.try_get_unit("pascal") * self.try_get_unit("second"));
        self.add_unit_deriv("poise", "[viscosity]", 0.1, "pascal*second");
        self.add_unit_deriv("reyn", "[viscosity]", 6894.757293168, "pascal*second");
        // psi * second
    }

    fn add_kinematic_viscosity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[area]") / self.try_get_dimension("[time]")).with_name("[kinematic_viscosity]"),
        );
        self.add_unit(self.try_get_unit("meter^2") / self.try_get_unit("second"));
        self.add_unit_deriv("stokes", "[kinematic_viscosity]", 1e-4, "meter^2/second");
        // centimeter^2 / second
    }

    fn add_fluidity(&mut self) {
        self.add_dimension(self.try_get_dimension("[viscosity]").pow(-1).with_name("[fluidity]"));
        self.add_unit(self.try_get_unit("poise").pow(-1));
        self.add_unit_deriv("rhe", "[fluidity]", 1.0, "poise^-1");
    }

    fn add_amount_of_substance(&mut self) {
        self.add_dimension(self.try_get_dimension("[dimensionless]").with_name("[substance]"));
        self.add_unit_deriv("particle", "[substance]", 1.0 / 6.02214076e23, "mole");
        // 1 / N_A
    }

    fn add_concentration(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[substance]") / self.try_get_dimension("[volume]")).with_name("[concentration]"),
        );
        self.add_unit(self.try_get_unit("mole") / self.try_get_unit("liter"));
        self.add_unit_deriv("molar", "[concentration]", 1.0, "mole/liter");
    }

    fn add_catalytic_activity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[substance]") / self.try_get_dimension("[time]")).with_name("[activity]"),
        );
        self.add_unit(self.try_get_unit("mole") / self.try_get_unit("second"));
        self.add_unit_deriv("katal", "[activity]", 1.0, "mole/second");
        self.add_unit_deriv("enzyme_unit", "[activity]", 1.6666666666666667e-8, "mole/second");
        // micromole / minute
    }

    fn add_entropy(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[energy]") / self.try_get_dimension("[temperature]")).with_name("[entropy]"),
        );
        self.add_unit(self.try_get_unit("joule") / self.try_get_unit("kelvin"));
        self.add_unit_deriv("clausius", "[entropy]", 4.184, "joule/kelvin"); // calorie / kelvin
    }

    fn add_molar_entropy(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[entropy]") / self.try_get_dimension("[substance]")).with_name("[molar_entropy]"),
        );
        self.add_unit(self.try_get_unit("joule") / self.try_get_unit("kelvin") / self.try_get_unit("mole"));
        self.add_unit_deriv("entropy_unit", "[molar_entropy]", 4.184, "joule/kelvin/mole");
        // calorie / kelvin / mole
    }

    fn add_charge(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[current]") * self.try_get_dimension("[time]")).with_name("[charge]"),
        );
        self.add_unit(self.try_get_unit("ampere") * self.try_get_unit("second"));
        self.add_unit_deriv("coulomb", "[charge]", 1., "ampere*second");
        self.add_unit(self.try_get_unit("coulomb"));
        self.add_unit_deriv("abcoulomb", "[charge]", 10.0, "coulomb");
        self.add_unit_deriv("faraday", "[charge]", 96485.33212, "coulomb"); // e * N_A * mole
        self.add_unit_deriv("conventional_coulomb_90", "[charge]", 1.0000000000000002, "coulomb"); // K_J90 * R_K90 / (K_J * R_K)
        self.add_unit_deriv("ampere_hour", "[charge]", 3600.0, "coulomb"); // ampere * hour
    }

    fn add_radiation(&mut self) {
        self.add_dimension(self.try_get_dimension("[time]").pow(-1).with_name("[radiation]"));
        self.add_unit(self.try_get_unit("count") / self.try_get_unit("second"));
        self.add_unit_deriv("becquerel", "[radiation]", 1., "count/second");
        self.add_unit_deriv("curie", "[radiation]", 3.7e10, "becquerel");
        self.add_unit_deriv("rutherford", "[radiation]", 1e6, "becquerel");
        self.add_unit(self.try_get_unit("joule") / self.try_get_unit("kilogram"));
        self.add_unit(self.try_get_unit("coulomb") / self.try_get_unit("kilogram"));
        self.add_dimension(self.try_get_dimension("[energy]") / self.try_get_dimension("[mass]"));
        self.add_unit_deriv("gray", "[energy]/[mass]", 1.0, "joule/kilogram");
        self.add_unit_deriv("sievert", "[energy]/[mass]", 1.0, "joule/kilogram");
        self.add_unit_deriv("rads", "[energy]/[mass]", 0.01, "gray");
        self.add_unit_deriv("rem", "[energy]/[mass]", 0.01, "sievert");
        // self.add_unit_deriv("roentgen", "[radiation]", 2.58e-4, "coulomb/kilogram");
    }

    fn add_heat_transmission(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[energy]") / self.try_get_dimension("[area]")).with_name("[heat_transmission]"),
        );
        self.add_unit(self.try_get_unit("joule") / self.try_get_unit("meter^2"));
        self.add_unit_deriv("peak_sun_hour", "[heat_transmission]", 3.6e6, "joule/meter^2"); // 1e3 * watt_hour / meter^2
        self.add_unit_deriv("langley", "[heat_transmission]", 41840.0, "joule/meter^2");
        // thermochemical_calorie / centimeter^2
    }

    fn add_luminance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[luminosity]") / self.try_get_dimension("[area]")).with_name("[luminance]"),
        );
        self.add_unit(self.try_get_unit("candela") / self.try_get_unit("meter^2"));
        self.add_unit_deriv("nit", "[luminance]", 1.0, "candela/meter^2");
        self.add_unit_deriv("stilb", "[luminance]", 1e4, "candela/meter^2"); // candela / centimeter^2
        self.add_unit_deriv("lambert", "[luminance]", 3183.098861837907, "candela/meter^2");
        // 1 / π * candela / centimeter^2
    }

    fn add_luminous_flux(&mut self) {
        self.add_dimension(self.try_get_dimension("[luminosity]").with_name("[luminous_flux]"));
        self.add_unit(self.try_get_unit("candela") * self.try_get_unit("steradian"));
        self.add_unit_deriv("lumen", "[luminous_flux]", 1.0, "candela*steradian");
    }

    fn add_illuminance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[luminous_flux]") / self.try_get_dimension("[area]")).with_name("[illuminance]"),
        );
        self.add_unit(self.try_get_unit("lumen") / self.try_get_unit("meter^2"));
        self.add_unit_deriv("lux", "[illuminance]", 1.0, "lumen/meter^2");
    }
    fn add_intensity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[power]") / self.try_get_dimension("[area]")).with_name("[intensity]"),
        );
        self.add_unit(self.try_get_unit("watt") / self.try_get_unit("meter^2"));
        self.add_unit_deriv("atomic_unit_of_intensity", "[intensity]", 3.50944758e16, "watt/meter^2");
        // 0.5 * ε_0 * c * atomic_unit_of_electric_field^2
    }

    fn add_current(&mut self) {
        self.add_unit_deriv("biot", "[current]", 10.0, "ampere");
        self.add_unit_deriv("abampere", "[current]", 10.0, "ampere");
        self.add_unit_deriv("atomic_unit_of_current", "[current]", 6.623618183e-3, "ampere"); // e / atomic_unit_of_time
        self.add_unit_deriv("mean_international_ampere", "[current]", 1.00034, "ampere"); // approximate
        self.add_unit_deriv("US_international_ampere", "[current]", 1.00033, "ampere"); // approximate
        self.add_unit_deriv("conventional_ampere_90", "[current]", 1.0000000000000002, "ampere"); // K_J90 * R_K90 / (K_J * R_K)
        self.add_unit_deriv("planck_current", "[current]", 3.4789e25, "ampere");
        // (c^6 / gravitational_constant / k_C)^0.5
    }

    fn add_electric_potential(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[energy]") / self.try_get_dimension("[charge]")).with_name("[electric_potential]"),
        );
        self.add_unit((self.try_get_unit("joule") / self.try_get_unit("coulomb")).with_name("volt"));
        self.add_unit_deriv("abvolt", "[electric_potential]", 1e-8, "volt");
        self.add_unit_deriv("mean_international_volt", "[electric_potential]", 1.00034, "volt"); // approximate
        self.add_unit_deriv("US_international_volt", "[electric_potential]", 1.00033, "volt"); // approximate
        self.add_unit_deriv(
            "conventional_volt_90",
            "[electric_potential]",
            1.0000000000000002,
            "volt",
        );
        // K_J90 / K_J
    }

    fn add_electric_field(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[electric_potential]") / self.try_get_dimension("[length]"))
                .with_name("[electric_field]"),
        );
        self.add_unit(self.try_get_unit("volt") / self.try_get_unit("meter"));
        self.add_unit_deriv(
            "atomic_unit_of_electric_field",
            "[electric_field]",
            5.14220652e11,
            "volt/meter",
        ); // e * k_C / a_0^2
    }

    fn add_electric_displacement_field(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[charge]") / self.try_get_dimension("[area]"))
                .with_name("[electric_displacement_field]"),
        );
        self.add_unit(self.try_get_unit("coulomb") / self.try_get_unit("meter^2"));
    }
    // this is an obscure unit that not even wikipedia can agree on how is defined, for now I am ignoring it as the dimensionlity don't add up
    // fn add_reduced_electric_field(&mut self) {
    //     self.add_dimension(
    //         (self.try_get_dimension("[electric_field]") * self.try_get_dimension("[area]"))
    //             .with_name("[reduced_electric_field]"),
    //     );
    //     self.add_unit(self.try_get_unit("volt") * self.try_get_unit("meter^2"));
    //     self.add_unit_deriv("townsend", "[reduced_electric_field]", 1e-21, "volt*meter^2");
    // }

    fn add_resistance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[electric_potential]") / self.try_get_dimension("[current]"))
                .with_name("[resistance]"),
        );
        self.add_unit((self.try_get_unit("volt") / self.try_get_unit("ampere")).with_name("ohm"));
        self.add_unit_deriv("abohm", "[resistance]", 1e-9, "ohm");
        self.add_unit_deriv("mean_international_ohm", "[resistance]", 1.00049, "ohm"); // approximate
        self.add_unit_deriv("US_international_ohm", "[resistance]", 1.000495, "ohm"); // approximate
        self.add_unit_deriv("conventional_ohm_90", "[resistance]", 1.0000000000000002, "ohm");
        // R_K / R_K90
    }
    fn add_resistivity(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[resistance]") * self.try_get_dimension("[length]")).with_name("[resistivity]"),
        );
        self.add_unit(self.try_get_unit("ohm") * self.try_get_unit("meter"));
    }

    fn add_conductance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[current]") / self.try_get_dimension("[electric_potential]"))
                .with_name("[conductance]"),
        );
        self.add_unit((self.try_get_unit("ampere") / self.try_get_unit("volt")).with_name("siemens"));
        self.add_unit_deriv("absiemens", "[conductance]", 1e9, "siemens");
    }

    fn add_capacitance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[charge]") / self.try_get_dimension("[electric_potential]"))
                .with_name("[capacitance]"),
        );
        self.add_unit((self.try_get_unit("coulomb") / self.try_get_unit("volt")).with_name("farad"));
        self.add_unit_deriv("abfarad", "[capacitance]", 1e9, "farad");
        self.add_unit_deriv("conventional_farad_90", "[capacitance]", 1.0000000000000002, "farad");
        // R_K90 / R_K
    }

    fn add_magnetic_flux(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[electric_potential]") * self.try_get_dimension("[time]"))
                .with_name("[magnetic_flux]"),
        );
        self.add_unit((self.try_get_unit("volt") * self.try_get_unit("second")).with_name("weber"));
        self.add_unit_deriv("unit_pole", "[magnetic_flux]", 1.2566370614359173e-6, "weber");
        // µ_0 * biot * centimeter
    }
    fn add_inductance(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[magnetic_flux]") / self.try_get_dimension("[current]")).with_name("[inductance]"),
        );
        self.add_unit((self.try_get_unit("weber") / self.try_get_unit("ampere")).with_name("henry"));
        self.add_unit_deriv("abhenry", "[inductance]", 1e-9, "henry");
        self.add_unit_deriv("conventional_henry_90", "[inductance]", 1.0000000000000002, "henry");
        // R_K / R_K90
    }

    fn add_magnetic_field(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[magnetic_flux]") / self.try_get_dimension("[area]"))
                .with_name("[magnetic_field]"),
        );
        self.add_unit((self.try_get_unit("weber") / self.try_get_unit("meter^2")).with_name("tesla"));
        self.add_unit_deriv("gamma", "[magnetic_field]", 1e-9, "tesla");
    }

    fn add_magnetomotive_force(&mut self) {
        self.add_dimension(self.try_get_dimension("[current]").with_name("[magnetomotive_force]"));
        self.add_unit_deriv("ampere_turn", "[magnetomotive_force]", 1.0, "ampere");
        self.add_unit_deriv("biot_turn", "[magnetomotive_force]", 10.0, "ampere");
        self.add_unit_deriv("gilbert", "[magnetomotive_force]", 0.7957747154594768, "ampere");
        // 1 / (4 * π) * biot_turn
    }

    fn add_magnetic_field_strength(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[current]") / self.try_get_dimension("[length]"))
                .with_name("[magnetic_field_strength]"),
        );
        self.add_unit(self.try_get_unit("ampere") / self.try_get_unit("meter"));
    }

    fn add_electric_dipole_moment(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[charge]") * self.try_get_dimension("[length]")).with_name("[electric_dipole]"),
        );
        self.add_unit(self.try_get_unit("coulomb") * self.try_get_unit("meter"));
        self.add_unit_deriv("debye", "[electric_dipole]", 3.3356409519815204e-30, "coulomb*meter");
        // 1e-9 / ζ * coulomb * angstrom
    }

    fn add_electric_quadrupole_moment(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[charge]") * self.try_get_dimension("[area]")).with_name("[electric_quadrupole]"),
        );
        self.add_unit(self.try_get_unit("coulomb") * self.try_get_unit("meter^2"));
        self.add_unit_deriv(
            "buckingham",
            "[electric_quadrupole]",
            3.3356409519815204e-40,
            "coulomb*meter^2",
        ); // debye * angstrom
    }

    fn add_magnetic_dipole_moment(&mut self) {
        self.add_dimension(
            (self.try_get_dimension("[current]") * self.try_get_dimension("[area]")).with_name("[magnetic_dipole]"),
        );
        self.add_unit(self.try_get_unit("ampere") * self.try_get_unit("meter^2"));
        self.add_unit_deriv("bohr_magneton", "[magnetic_dipole]", 9.274009994e-24, "ampere*meter^2"); // e * hbar / (2 * m_e)
        self.add_unit_deriv(
            "nuclear_magneton",
            "[magnetic_dipole]",
            5.050783699e-27,
            "ampere*meter^2",
        );
        // e * hbar / (2 * m_p)
    }

    fn add_refractive_index(&mut self) {
        self.add_dimension(Dimension::new("[refractive_index]", vec![]));
        self.add_unit_simple("refractive_index_unit", "[refractive_index]");
    }
    fn add_uscs_length_international(&mut self) {
        self.add_unit_deriv("yard", "[length]", 0.9144, "meter");
        self.add_unit_deriv("inch", "[length]", 1.0 / 36.0, "yard");
        self.add_unit_deriv("thou", "[length]", 1e-3, "inch");
        self.add_unit_deriv("hand", "[length]", 4.0, "inch");
        self.add_unit_deriv("foot", "[length]", 1.0 / 3.0, "yard");
        self.add_unit_deriv("mile", "[length]", 1760.0, "yard");
        self.add_unit(self.try_get_unit("thou").with_name("mil_length"));

        self.add_unit(self.try_get_unit("mil_length").pow(2));

        self.add_unit_deriv("circular_mil", "[area]", std::f64::consts::PI / 4.0, "mil_length^2");

        self.add_unit(self.try_get_unit("inch").pow(2).with_name("square_inch"));
        self.add_unit(self.try_get_unit("foot").pow(2).with_name("square_foot"));
        self.add_unit(self.try_get_unit("yard").pow(2).with_name("square_yard"));
        self.add_unit(self.try_get_unit("mile").pow(2).with_name("square_mile"));

        self.add_unit(self.try_get_unit("inch").pow(3).with_name("cubic_inch"));
        self.add_unit(self.try_get_unit("foot").pow(3).with_name("cubic_foot"));
        self.add_unit(self.try_get_unit("yard").pow(3).with_name("cubic_yard"));
    }

    fn add_uscs_length_survey(&mut self) {
        self.add_unit_deriv("survey_foot", "[length]", 1200.0 / 3937.0, "meter");
        self.add_unit_deriv("rod", "[length]", 16.5, "survey_foot");
        self.add_unit_deriv("chain", "[length]", 4.0, "rod");
        self.add_unit_deriv("link", "[length]", 1e-2, "chain");
        self.add_unit_deriv("fathom", "[length]", 6.0, "survey_foot");
        self.add_unit_deriv("furlong", "[length]", 40.0, "rod");
        self.add_unit_deriv("cables_length", "[length]", 120.0, "fathom");
        self.add_unit_deriv("survey_mile", "[length]", 5280.0, "survey_foot");
        self.add_unit_deriv("league", "[length]", 3.0, "survey_mile");

        self.add_unit(self.try_get_unit("rod").pow(2).with_name("square_rod"));
        self.add_unit(self.try_get_unit("chain").pow(2).with_name("square_chain"));
        self.add_unit(self.try_get_unit("survey_mile").pow(2).with_name("square_survey_mile"));
        self.add_unit(self.try_get_unit("league").pow(2).with_name("square_league"));
        self.add_unit_deriv("acre", "[area]", 10., "square_chain");
        self.add_unit(self.try_get_unit("acre") * (self.try_get_unit("survey_foot")).with_name("acre_foot"));
    }

    fn add_uscs_dry_volume(&mut self) {
        self.add_unit_deriv("bushel", "[volume]", 2150.42, "cubic_inch");
        self.add_unit_deriv("dry_pint", "[volume]", 1.0 / 64.0, "bushel");
        self.add_unit_deriv("dry_quart", "[volume]", 1.0 / 32.0, "bushel");
        self.add_unit_deriv("dry_gallon", "[volume]", 1.0 / 8.0, "bushel");
        self.add_unit_deriv("peck", "[volume]", 1.0 / 4.0, "bushel");
        self.add_unit_deriv("dry_barrel", "[volume]", 7056.0, "cubic_inch");
        self.add_unit((self.try_get_unit("foot").pow(2) * self.try_get_unit("inch")).with_name("board_foot"));
    }

    fn add_uscs_liquid_volume(&mut self) {
        self.add_unit_deriv("gallon", "[volume]", 231.0, "cubic_inch");
        self.add_unit_deriv("quart", "[volume]", 1.0 / 4.0, "gallon");
        self.add_unit_deriv("pint", "[volume]", 1.0 / 2.0, "quart");
        self.add_unit_deriv("fifth", "[volume]", 1.0 / 5.0, "gallon");
        self.add_unit_deriv("gill", "[volume]", 1.0 / 4.0, "pint");
        self.add_unit_deriv("fluid_ounce", "[volume]", 1.0 / 16.0, "pint");
        self.add_unit_deriv("fluid_dram", "[volume]", 1.0 / 128.0, "pint");
        self.add_unit_deriv("minim", "[volume]", 1.0 / 7680.0, "pint");
    }

    fn add_uscs_volume_other(&mut self) {
        self.add_unit_deriv("teaspoon", "[volume]", 1.0 / 6.0, "fluid_ounce");
        self.add_unit_deriv("tablespoon", "[volume]", 1.0 / 2.0, "fluid_ounce");
        self.add_unit_deriv("shot", "[volume]", 3.0, "tablespoon");
        self.add_unit_deriv("cup", "[volume]", 1.0 / 2.0, "pint");
        self.add_unit_deriv("barrel", "[volume]", 31.5, "gallon");
        self.add_unit_deriv("oil_barrel", "[volume]", 42.0, "gallon");
        self.add_unit_deriv("beer_barrel", "[volume]", 31.0, "gallon");
        self.add_unit_deriv("hogshead", "[volume]", 63.0, "gallon");
    }

    fn add_avoirdupois(&mut self) {
        self.add_unit_deriv("pound", "[mass]", 7000.0, "grain");
        self.add_unit_deriv("dram", "[mass]", 1.0 / 256.0, "pound");
        self.add_unit_deriv("ounce", "[mass]", 1.0 / 16.0, "pound");
        self.add_unit_deriv("stone", "[mass]", 14.0, "pound");
        self.add_unit_deriv("quarter", "[mass]", 28.0, "stone");
        self.add_unit_deriv("bag", "[mass]", 94.0, "pound");
        self.add_unit_deriv("hundredweight", "[mass]", 100.0, "pound");
        self.add_unit_deriv("long_hundredweight", "[mass]", 112.0, "pound");
        self.add_unit_deriv("ton", "[mass]", 2000.0, "pound");
        self.add_unit_deriv("long_ton", "[mass]", 2240.0, "pound");
        self.add_unit(self.try_get_unit("pound") * self.try_get_unit("second").pow(2) / self.try_get_unit("foot"));
        self.add_unit(self.try_get_unit("pound") * self.try_get_unit("second").pow(2) / self.try_get_unit("inch"));
        // Need a way to handle constants with a physical unit
        // self.add_unit_deriv("slug", "[mass]", g0, "pound*second^2/foot");
        // self.add_unit_deriv("slinch", "[mass]", g0, "pound*second^2/inch");

        // self.add_unit_deriv("force_ounce", "[force]", g0, "ounce");
        // self.add_unit_deriv("force_pound", "[force]", g0, "pound");
        // self.add_unit_deriv("force_ton", "[force]", g0, "ton");
        // self.add_unit_deriv("force_long_ton", "[force]", g0, "long_ton");
        // self.add_unit_deriv("kip", "[force]", 1000.0, "force_pound");
        self.add_unit(
            (self.try_get_unit("pound") * self.try_get_unit("foot") / self.try_get_unit("second").pow(2))
                .with_name("poundal"),
        );
    }

    fn add_avoirdupois_uk(&mut self) {
        self.add_unit_deriv("UK_hundredweight", "[mass]", 1.0, "long_hundredweight");
        self.add_unit_deriv("UK_ton", "[mass]", 1.0, "long_ton");
        // self.add_unit_deriv("UK_force_ton", "[force]", 1.0, "force_long_ton");
    }

    fn add_avoirdupois_us(&mut self) {
        self.add_unit_deriv("US_hundredweight", "[mass]", 1.0, "hundredweight");
        self.add_unit_deriv("US_ton", "[mass]", 1.0, "ton");
        // self.add_unit_deriv("US_force_ton", "[force]", 1.0, "force_ton");
    }

    fn add_troy(&mut self) {
        self.add_unit_deriv("pennyweight", "[mass]", 24.0, "grain");
        self.add_unit_deriv("troy_ounce", "[mass]", 480.0, "grain");
        self.add_unit_deriv("troy_pound", "[mass]", 12.0, "troy_ounce");
    }

    fn add_apothecary(&mut self) {
        self.add_unit_deriv("scruple", "[mass]", 20.0, "grain");
        self.add_unit_deriv("apothecary_dram", "[mass]", 3.0, "scruple");
        self.add_unit_deriv("apothecary_ounce", "[mass]", 8.0, "apothecary_dram");
        self.add_unit_deriv("apothecary_pound", "[mass]", 12.0, "apothecary_ounce");
    }

    fn add_imperial_volume(&mut self) {
        self.add_unit_deriv("imperial_gallon", "[volume]", 4.54609, "liter");
        self.add_unit_deriv("imperial_pint", "[volume]", 1.0 / 8.0, "imperial_gallon");
        self.add_unit_deriv("imperial_quart", "[volume]", 1.0 / 4.0, "imperial_gallon");
        self.add_unit_deriv("imperial_peck", "[volume]", 2.0, "imperial_gallon");
        self.add_unit_deriv("imperial_bushel", "[volume]", 8.0, "imperial_gallon");
        self.add_unit_deriv("imperial_barrel", "[volume]", 36.0, "imperial_gallon");
        self.add_unit_deriv("imperial_fluid_ounce", "[volume]", 1.0 / 20.0, "imperial_pint");
        self.add_unit_deriv("imperial_minim", "[volume]", 1.0 / 480.0, "imperial_fluid_ounce");
        self.add_unit_deriv("imperial_fluid_scruple", "[volume]", 1.0 / 24.0, "imperial_fluid_ounce");
        self.add_unit_deriv("imperial_fluid_drachm", "[volume]", 1.0 / 8.0, "imperial_fluid_ounce");
        self.add_unit_deriv("imperial_gill", "[volume]", 1.0 / 4.0, "imperial_pint");
        self.add_unit_deriv("imperial_cup", "[volume]", 1.0 / 2.0, "imperial_pint");
    }
    fn add_printer(&mut self) {
        self.add_unit_deriv("pica", "[length]", 1.0 / 6.0, "inch");
        self.add_unit_deriv("point", "[length]", 1.0 / 12.0, "pica");
        self.add_unit_deriv("didot", "[length]", 1.0 / 2660.0, "meter");
        self.add_unit_deriv("cicero", "[length]", 12.0, "didot");
        self.add_unit_deriv("tex_point", "[length]", 1.0 / 72.27, "inch");
        self.add_unit_deriv("tex_pica", "[length]", 12.0, "tex_point");
        self.add_unit_deriv("tex_didot", "[length]", 1238.0 / 1157.0, "tex_point");
        self.add_unit_deriv("tex_cicero", "[length]", 12.0, "tex_didot");
        self.add_unit_deriv("scaled_point", "[length]", 1.0 / 65536.0, "tex_point");
    }
}

#[allow(clippy::redundant_closure)]
pub static REGISTRY: Lazy<UnitRegistry> = Lazy::new(|| UnitRegistry::new_with_definitions());

#[cfg(test)]
mod test {
    use is_close::is_close;

    use super::REGISTRY;

    #[test]
    fn test_new_definitions() {
        assert_eq!(REGISTRY.try_get_unit("meter").name(), "meter");
    }

    #[test]
    fn test_meter_to_foot() {
        let conv_factor = REGISTRY.convert("foot".to_string(), "meter".to_string()).unwrap();
        assert!(is_close!(conv_factor, 0.3048));
    }
}
