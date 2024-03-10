use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, DateTime, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis},
};

use chrono::Datelike;
use julian_day_converter::JULIAN_DAY_UNIX_EPOCH_DAYS;
use strum::{AsRefStr, EnumProperty, VariantArray};

#[derive(Debug, Copy, Clone)]
/// This structure represents the second planet from the sun
pub struct Europa;

impl Body for Europa {
    /// A.D 1610 January 7, 12:00:00
    fn epoch(&self) -> f64 {
        2309107e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0094
    }

    fn orbital_period(&self) -> f64 {
        3.50
    }

    fn rotational_period(&self) -> f64 {
        302_400.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        671.1
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(&mut MeanMotion, day, self.perihelion(), self.orbital_period())
    }

    fn to_date(&mut self, julian_date: f64) -> Date {
        Date::default().compute(julian_date, self.epoch(), self.rotational_period(), self.perihelion(), self.semimajor(), self.orbital_eccentricity(), self.orbital_period())
    }
}

/// Similar format as mars..
///
/// The difference is its from +/- 240 instead of 180..
///
///
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Europarian {
    #[strum(props(Code = "HBT", Name = "High Band Time", Offset = "-51.00", East = "0", West = "30"))]
    /// Europa Coordinated Time - 6
    EMTCn6,
    #[strum(props(Code = "LMT", Name = "Lowed Mottled Time", Offset = "-42.50", East = "30", West = "60"))]
    /// Europa Coordinated Time - 5
    EMTCn5,
    #[strum(props(Code = "MAET", Name = "Major Ejecta Time", Offset = "-34.00", East = "60", West = "90"))]
    /// Europa Coordinated Time - 4
    EMTCn4,
    #[strum(props(Code = "MIET", Name = "Minor Ejecta Time", Offset = "-25.50", East = "90", West = "120"))]
    /// Europa Coordinated Time - 3
    EMTCn3,
    #[strum(props(Code = "DBT", Name = "Duplex Band Time", Offset = "-17.00", East = "120", West = "150"))]
    /// Europa Coordinated Time - 2
    EMTCn2,
    #[strum(props(Code = "RCT", Name = "Ridged Chaos Time", Offset = "-8.50", East = "150", West = "180"))]
    /// Europa Coordinated Time - 1
    EMTCn1,
    #[default]
    #[strum(props(Code = "MCT", Name = "Mottled Crater Time", Offset = "0.0", East = "180", West = "150"))]
    /// Europa Coordinated Time
    EMTC,
    #[strum(props(Code = "MEST", Name = "Minor Ejecta Sub Time", Offset = "8.50", East = "150", West = "120"))]
    /// Europa Coordinated Time + 1
    EMTCp1,
    #[strum(props(Code = "ABT", Name = "Albedo Bottle Time", Offset = "17.00", East = "120", West = "90"))]
    /// Europa Coordinated Time + 2
    EMTCp2,
    #[strum(props(Code = "TRT", Name = "Temporal Ridges Time", Offset = "25.50", East = "90", West = "60"))]
    /// Europa Coordinated Time + 3
    EMTCp3,
    #[strum(props(Code = "DT", Name = "Dolphin Time", Offset = "34.00", East = "60", West = "30"))]
    /// Europa Coordinated Time + 4
    EMTCp4,
    #[strum(props(Code = "MT", Name = "Metro Time", Offset = "42.50", East = "30", West = "0"))]
    /// Europa Coordinated Time + 5
    EMTCp5,
}
