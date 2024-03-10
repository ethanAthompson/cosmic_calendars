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
pub struct Eros;

/// Similar format as mars..
///

#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Erosian {
    #[strum(props(Code = "DT", Name = "Don Time", Offset = "-3.162", East = "360", West = "330"))]
    /// Eros Coordinated Time - 6
    ESTCn6,
    #[strum(props(Code = "JT", Name = "Juan Time", Offset = "-2.635", East = "330", West = "300"))]
    /// Eros Coordinated Time - 5
    ESTCn5,
    #[strum(props(Code = "HST", Name = "Himeros Time", Offset = "-2.108", East = "300", West = "270"))]
    /// Eros Coordinated Time - 4
    ESTCn4,
    #[strum(props(Code = "CDT", Name = "Cupid Time", Offset = "-1.581", East = "270", West = "240"))]
    /// Eros Coordinated Time - 3
    ESTCn3,
    #[strum(props(Code = "ADT", Name = "Avtandi Time", Offset = "-1.054", East = "240", West = "210"))]
    /// Eros Coordinated Time - 2
    ESTCn2,
    #[strum(props(Code = "LRT", Name = "Leander Time", Offset = "-0.527", East = "210", West = "180"))]
    /// Eros Coordinated Time - 1
    ESTCn1,
    #[default]
    #[strum(props(Code = "OST", Name = "Orpheus Time", Offset = "0.0", East = "180", West = "150"))]
    /// Eros Coordinated Time
    ESTC,
    #[strum(props(Code = "AAT", Name = "Aida Time", Offset = "0.527", East = "150", West = "120"))]
    /// Eros Coordinated Time + 1
    ESTCp1,
    #[strum(props(Code = "GIT", Name = "Genji Time", Offset = "1.054", East = "120", West = "90"))]
    /// Eros Coordinated Time + 2
    ESTCp2,
    #[strum(props(Code = "FST", Name = "Fujitsubo Time", Offset = "1.581", East = "90", West = "60"))]
    /// Eros Coordinated Time + 3
    ESTCp3,
    #[strum(props(Code = "GT", Name = "Gamba Time", Offset = "2.108", East = "60", West = "30"))]
    /// Eros Coordinated Time + 4
    ESTCp4,
    #[strum(props(Code = "ST", Name = "Selene Time", Offset = "2.635", East = "30", West = "0"))]
    /// Eros Coordinated Time + 5
    ESTCp5,
}
