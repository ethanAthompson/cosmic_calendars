
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
pub struct Luna;

impl Body for Luna {
    /// A.D 1609 November 30, 12:00:00
    fn epoch(&self) -> f64 {
        2309069e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0549
    }

    fn orbital_period(&self) -> f64 {
        29.53
    }

    fn rotational_period(&self) -> f64 {
        2_551_443.84
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        0.3844
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


/// 
/// "The Moon" 
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Lunarian {
    #[strum(props(Code = "FST", Name= "Focas Time", Offset = "-262.288", East = "-180", West = "-150"))]
    /// Luna Coordinated Time - 4
    LuTCn4,
    #[strum(props(Code = "RRT", Name= "Rumker Time", Offset = "-196.716", East = "-150", West = "-100"))]
    /// Luna Coordinated Time - 3
    LuTCn3,
    #[strum(props(Code = "HNT", Name= "Hansteen Time", Offset = "-131.144", East = "-100", West = "-50"))]
    /// Luna Coordinated Time - 2
    LuTCn2,
    #[strum(props(Code = "AST", Name= "Agnes Time", Offset = "-65.572", East = "-50", West = "0"))]
    /// Luna Coordinated Time - 1
    LuTCn1,
    #[default]
    #[strum(props(Code = "LET", Name= "Ampere Time", Offset = "0.0", East = "0", West = "50"))]
    /// Luna Coordinated Time
    LuTC,
    #[strum(props(Code = "LAT", Name= "Leda Time", Offset = "65.572", East = "50", West = "100"))]
    /// Luna Coordinated Time + 1
    LuTCp1,
    #[strum(props(Code = "ART", Name= "Ardeshir Time", Offset = "131.144", East = "100", West = "150"))]
    /// Luna Coordinated Time + 2
    LuTCp2,
    #[strum(props(Code = "KVT", Name= "Kurchatov Time", Offset = "196.716", East = "150", West = "180"))]
    /// Luna Coordinated Time + 3
    LuTCp3,
}