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
pub struct Hebe;

/// Similar format as mars..
///
/// 
/// From:
/// 
/// Elevation map (in km) of (6) Hebe, with respect to,
/// a volume-equivalent ellipsoid best fitting our 3D-shape model.
/// The five major depressions are identified by numbers. 
/// 
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Hebeian {
    #[strum(props(Code = "FIT", Name = "First Depression Indigo Time", Offset = "-4.38", East = "0", West = "30"))]
    /// Hebe Coordinated Time - 6
    HeTC6,
    #[strum(props(Code = "FMT", Name = "First Depression Mount Time", Offset = "-3.65", East = "30", West = "60"))]
    /// Hebe Coordinated Time - 5
    HeTCn5,
    #[strum(props(Code = "SDIT", Name = "Second Depression Indigo Time", Offset = "-2.92", East = "60", West = "90"))]
    /// Hebe Coordinated Time - 4
    HeTCn4,
    #[strum(props(Code = "SDMT", Name = "Second Depression Mount Time", Offset = "-2.19", East = "90", West = "120"))]
    /// Hebe Coordinated Time - 3
    HeTCn3,
    #[strum(props(Code = "FPT", Name = "First Plain Time", Offset = "-1.46", East = "120", West = "150"))]
    /// Hebe Coordinated Time - 2
    HeTCn2,
    #[strum(props(Code = "TDIT", Name = "Third Depression Indigo Time", Offset = "-0.73", East = "150", West = "180"))]
    /// Hebe Coordinated Time - 1
    HeTCn1,
    #[default]
    #[strum(props(Code = "TDMT", Name = "Third Depression Mount Time", Offset = "0.0", East = "180", West = "210"))]
    /// Hebe Coordinated Time
    HeTC,
    #[strum(props(Code = "SPT", Name = "Second Plain Time", Offset = "0.73", East = "210", West = "240"))]
    /// Hebe Coordinated Time + 1
    HeTCp1,
    #[strum(props(Code = "DNT", Name = "Fourth Depression Noir Time", Offset = "1.46", East = "240", West = "270"))]
    /// Hebe Coordinated Time + 2
    HeTCp2,
    #[strum(props(Code = "CNAT", Name = "Converged Depression Azure Time", Offset = "2.19", East = "270", West = "300"))]
    /// Hebe Coordinated Time + 3
    HeTCp3,
    #[strum(props(Code = "DAT", Name = "Fifth Depression Azure Time", Offset = "2.92", East = "300", West = "330"))]
    /// Hebe Coordinated Time + 4
    HeTCp4,
    #[strum(props(Code = "DMT", Name = "Fifth Depression Mount Time", Offset = "3.65", East = "330", West = "360"))]
    /// Hebe Coordinated Time + 5
    HeTCp5,
}
