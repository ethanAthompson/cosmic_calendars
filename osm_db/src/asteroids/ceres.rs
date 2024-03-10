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
pub struct Ceres;

impl Body for Ceres {
    /// A.D. 1801 January 1, 12:00:00
    fn epoch(&self) -> f64 {
        2378862e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0785
    }

    fn orbital_period(&self) -> f64 {
        1_680.15
    }

    fn rotational_period(&self) -> f64 {
        32_400.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        2.768 
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
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Ceresian {
    #[strum(props(Code = "HIT", Name = "Haulani Time", Offset = "-4.55", East = "0", West = "30"))]
    /// Ceres Coordinated Time - 5
    CTCn5,
    #[strum(props(Code = "ZIT", Name = "Zadeni Time", Offset = "-3.64", East = "30", West = "60"))]
    /// Ceres Coordinated Time - 4
    CTCn4,
    #[strum(props(Code = "VST", Name = "Vintonus Time", Offset = "-2.73", East = "60", West = "90"))]
    /// Ceres Coordinated Time - 3
    CTCn3,
    #[strum(props(Code = "KNT", Name = "Kerwan Time", Offset = "-1.82", East = "90", West = "120"))]
    /// Ceres Coordinated Time - 2
    CTCn2,
    #[strum(props(Code = "DUT", Name = "Dantu Time", Offset = "-0.91", East = "120", West = "150"))]
    /// Ceres Coordinated Time - 1
    CTCn1,
    #[default]
    #[strum(props(Code = "TUT", Name = "Toharu Time", Offset = "0.0", East = "150", West = "180"))]
    /// Ceres Coordinated Time
    CTC,
    #[strum(props(Code = "EUT", Name = "Ezinu Time", Offset = "0.91", East = "180", West = "210"))]
    /// Ceres Coordinated Time + 1
    CTCp1,
    #[strum(props(Code = "ORT", Name = "Occator Time", Offset = "1.82", East = "210", West = "240"))]
    /// Ceres Coordinated Time + 2
    CTCp2,
    #[strum(props(Code = "UAT", Name = "Urvara Time", Offset = "2.73", East = "240", West = "270"))]
    /// Ceres Coordinated Time + 3
    CTCp3,
    #[strum(props(Code = "YET", Name = "Yalode Time", Offset = "3.64", East = "270", West = "300"))]
    /// Ceres Coordinated Time + 4
    CTCp4,
    #[strum(props(Code = "FOT", Name = "Fejokoo Time", Offset = "4.55", East = "300", West = "330"))]
    /// Ceres Coordinated Time + 5
    CTCp5,
    #[strum(props(Code = "ROT", Name = "Rongo Time", Offset = "5.46", East = "330", West = "360"))]
    /// Ceres Coordinated Time + 5
    CTCp6,
}
