
use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, DateTime, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis},
};

use chrono::Datelike;
use julian_day_converter::JULIAN_DAY_UNIX_EPOCH_DAYS;
use strum::{AsRefStr, EnumProperty, VariantArray};

use super::EARTH_ROTATIONAL_PERIOD;

#[derive(Debug, Copy, Clone)]
/// This structure represents the second planet from the sun
pub struct Venus;

impl Body for Venus {
    /// A.D 1990 August 10, 12:00:00:0
    fn epoch(&self) -> f64 {
        2_448_114.00e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0070
    }

    fn orbital_period(&self) -> f64 {
        225.00
    }

    fn rotational_period(&self) -> f64 {
        20_995.20
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        108.210
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
pub enum Venarian {
    #[strum(props(Code = "AAT", Name= "Asteria Time", Offset = "-2332.8", East = "-240", West = "-270"))]
    /// Venus Coordinated Time - 4
    VTCn4,
    #[strum(props(Code = "TST", Name= "Themis Time", Offset = "-1749.6", East = "-270", West = "-300"))]
    /// Venus Coordinated Time - 3
    VTCn3,
    #[strum(props(Code = "GET", Name= "Guinevere Time", Offset = "-1166.4", East = "-300", West = "-330"))]
    /// Venus Coordinated Time - 2
    VTCn2,
    #[strum(props(Code = "LAT", Name= "Lavinia Time", Offset = "-583.2", East = "-330", West = "0"))]
    /// Venus Coordinated Time - 1
    VTCn1,
    #[default]
    #[strum(props(Code = "MLT", Name= "Mawell Time", Offset = "0.0", East = "0", West = "30"))]
    /// Venus Coordinated Time
    VTC,
    #[strum(props(Code = "LET", Name= "Leda Time", Offset = "583.2", East = "30", West = "60"))]
    /// Venus Coordinated Time + 1
    VTCp1,
    #[strum(props(Code = "TET", Name= "Tellus Time", Offset = "1166.4", East = "60", West = "90"))]
    /// Venus Coordinated Time + 2
    VTCp2,
    #[strum(props(Code = "OAT", Name= "Ovda Time", Offset = "1749.6", East = "90", West = "120"))]
    /// Venus Coordinated Time + 3
    VTCp3,
    #[strum(props(Code = "THT", Name= "Thetis Time", Offset = "2332.8", East = "120", West = "150"))]
    /// Venus Coordinated Time + 4
    VTCp4,
    #[strum(props(Code = "AST", Name= "Artemis Time", Offset = "2916", East = "150", West = "180"))]
    /// Venus Coordinated Time + 5
    VTCp5,
    #[strum(props(Code = "AAT", Name= "Atla Time", Offset = "3499.2", East = "180", West = "210"))]
    /// Venus Coordinated Time + 6
    VTCp6,
    #[strum(props(Code = "VNT", Name= "Venarian Time", Offset = "4082.4", East = "210", West = "240"))]
    /// Venus Coordinated Time + 7
    VTCp7,
}