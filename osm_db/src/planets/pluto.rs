
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
/// This structure represents the dwarf planet from the sun
pub struct Pluto;
impl Body for Pluto {
    /// A.D 1930 February 18, 23:00:00
    fn epoch(&self) -> f64 {
        2_426_026.00e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.2500
    }

    fn orbital_period(&self) -> f64 {
        90_560.00
    }

    fn rotational_period(&self) -> f64 {
        552_960.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        5869.656	
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


/// Similar to Mars
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Plutonian {
    #[strum(props(Code = "LYT", Name= "Plutonian End Time", Offset = "-91.8", East = "-180", West = "-150"))]
    /// Pluto Coordinated Time - 6
    PTCn6,
    #[strum(props(Code = "SLT", Name= "Simonelli Time", Offset = "-76.5", East = "-150", West = "-120"))]
    /// Pluto Coordinated Time - 5
    PTCn5,
    #[strum(props(Code = "AXT", Name= "Slelphir Time", Offset = "-61.2", East = "-120", West = "-90"))]
    /// Pluto Coordinated Time - 4
    PTCn4,
    #[strum(props(Code = "NET", Name= "Mwindo Time", Offset = "-45.9", East = "-90", West = "-60"))]
    /// Pluto Coordinated Time - 3
    PTCn3,
    #[strum(props(Code = "ET", Name= "Tartarus Time", Offset = "-30.6", East = "-60", West = "-30"))]
    /// Pluto Coordinated Time - 2
    PTCn2,
    #[strum(props(Code = "RAT", Name= "Kiladze Time", Offset = "-15.3", East = "-30", West = "0"))]
    /// Pluto Coordinated Time - 1
    PTCn1,
    #[default]
    #[strum(props(Code = "CTT", Name= "Sputnik Time", Offset = "0.0", East = "0", West = "30"))]
    /// Pluto Coordinated Time
    PTC,
    #[strum(props(Code = "JAT", Name= "Voyager Time", Offset = "15.3", East = "30", West = "60"))]
    /// Pluto Coordinated Time + 1
    PTCp1,
    #[strum(props(Code = "SPT", Name= "Burney Time", Offset = "30.6", East = "60", West = "90"))]
    /// Pluto Coordinated Time + 2
    PTCp2,
    #[strum(props(Code = "SST", Name= "Venera Time", Offset = "45.9", East = "90", West = "120"))]
    /// Pluto Coordinated Time + 3
    PTCp3,
    #[strum(props(Code = "AAT", Name= "Vega Time", Offset = "61.2", East = "120", West = "150"))]
    /// Pluto Coordinated Time + 4
    PTCp4,
    #[strum(props(Code = "SAT", Name= "Plutonian Start Time", Offset = "76.5", East = "150", West = "180"))]
    /// Pluto Coordinated Time + 5
    PTCp5,
}