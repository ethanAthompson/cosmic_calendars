
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