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
/// This structure represents the first planet from the sun
pub struct Mercury;


impl Body for Mercury {
    /// A.D 1959 September 9, 12:00:00
    fn epoch(&self) -> f64 {
        2436821e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.2000
    }

    fn orbital_period(&self) -> f64 {
        88.00
    }

    fn rotational_period(&self) -> f64 {
        5_063_040.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        57.909
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


#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
/// Goes +15 West/East for MeTC-6 and MeTC+6
/// Goes +30 West/East for others..
/// 
pub enum Mercurian {
    #[strum(props(Code = "EYT", Name= "Empty Time", Offset = "-849.6", East = "0", West = "15"))]
    /// Mercury Coordinated Time - 6
    MeTCn6,
    #[strum(props(Code = "COT", Name= "Calypso Time", Offset = "-708", East = "15", West = "45"))]
    /// Mercury Coordinated Time - 5
    MeTCn5,
    #[strum(props(Code = "BAT", Name= "Belgica Time", Offset = "-566.4", East = "45", West = "75"))]
    /// Mercury Coordinated Time - 4
    MeTCn4,
    #[strum(props(Code = "ENT", Name= "Eltanian Time", Offset = "-424.8", East = "75", West = "105"))]
    /// Mercury Coordinated Time - 3
    MeTCn3,
    #[strum(props(Code = "RIT", Name= "Raditladi Time", Offset = "-283.2", East = "105", West = "135"))]
    /// Mercury Coordinated Time - 2
    MeTCn2,
    #[strum(props(Code = "CST", Name= "Caloris Time", Offset = "-141.6", East = "135", West = "165"))]
    /// Mercury Coordinated Time - 1
    MeTCn1,
    #[default]
    #[strum(props(Code = "HT", Name= "Hero Time", Offset = "0.0", East = "165", West = "195"))]
    /// Mercury Coordinated Time
    MeTC,
    #[strum(props(Code = "SIT", Name= "Suisei Time", Offset = "141.6", East = "195", West = "225"))]
    /// Mercury Coordinated Time + 1
    MeTCp1,
    #[strum(props(Code = "BNT", Name= "Beethoven Time", Offset = "283.2", East = "225", West = "255"))]
    /// Mercury Coordinated Time + 2
    MeTCp2,
    #[strum(props(Code = "BST", Name= "Borealis Time", Offset = "424.8", East = "255", West = "285"))]
    /// Mercury Coordinated Time + 3
    MeTCp3,
    #[strum(props(Code = "HKT", Name= "Haystack Time", Offset = "566.4", East = "285", West = "315"))]
    /// Mercury Coordinated Time + 4
    MeTCp4,
    #[strum(props(Code = "CAT", Name= "Catena Time", Offset = "708", East = "315", West = "345"))]
    /// Mercury Coordinated Time + 5
    MeTCp5,
    #[strum(props(Code = "SI", Name= "Sanai Time", Offset = "849.6", East = "345", West = "360"))]
    /// Mercury Coordinated Time + 6
    MeTCp6,
}