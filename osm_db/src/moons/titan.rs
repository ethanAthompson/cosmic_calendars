
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
pub struct Titan;


/// Similar format as mars..
/// 
/// The difference is its from +/- 240 instead of 180..
/// 
/// 
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Titanian {
    #[strum(props(Code = "TST", Name = "Tsegihi Time", Offset = "-1913.40", East = "360", West = "330"))]
    /// Titan Coordinated Time - 5
    TTCn5,
    #[strum(props(Code = "SOT", Name = "Senkyo Time", Offset = "-1530.72", East = "330", West = "300"))]
    /// Titan Coordinated Time - 4
    TTCn4,
    #[strum(props(Code = "HPT", Name = "Hetpet Time", Offset = "-1147.04", East = "300", West = "270"))]
    /// Titan Coordinated Time - 3
    TTCn3,
    #[strum(props(Code = "BTT", Name = "Belet Time", Offset = "-765.36", East = "270", West = "240"))]
    /// Titan Coordinated Time - 2
    TTCn2,
    #[strum(props(Code = "CNT", Name = "Caladan Time", Offset = "-382.68", East = "240", West = "210"))]
    /// Titan Coordinated Time - 1
    TTCn1,
    #[default]
    #[strum(props(Code = "DNT", Name = "Dilmun Time", Offset = "0.0", East = "210", West = "180"))]
    /// Titan Coordinated Time
    TTC,
    #[strum(props(Code = "SAT", Name = "Shangrila Time", Offset = "382.68", East = "180", West = "150"))]
    /// Titan Coordinated Time + 1
    TTCp1,
    #[strum(props(Code = "TT", Name = "Tui Time", Offset = "765.36", East = "150", West = "120"))]
    /// Titan Coordinated Time + 2
    TTCp2,
    #[strum(props(Code = "XUT", Name = "Xanadu Time", Offset = "1147.04", East = "120", West = "90"))]
    /// Titan Coordinated Time + 3
    TTCp3,
    #[strum(props(Code = "HIT", Name = "Hotei Time", Offset = "1530.72", East = "90", West = "60"))]
    /// Titan Coordinated Time + 4
    TTCp4,
    #[strum(props(Code = "MYT", Name = "Momoy Time", Offset = "1913.40", East = "60", West = "30"))]
    /// Titan Coordinated Time + 5
    TTCp5,
    #[strum(props(Code = "QAT", Name = "Quivira Time", Offset = "2296.08", East = "30", West = "0"))]
    /// Titan Coordinated Time + 6
    TTCp6,
}