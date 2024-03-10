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
pub struct Io;

/// Similar format as mars..
///
/// The difference is its names comes from lots of volcanos.. :-| 
///
///
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Ioan {
    #[strum(props(Code = "STT", Name = "Surt Time", Offset = "-1913.40", East = "360", West = "330"))]
    /// Io Coordinated Time - 5
    IoTCn5,
    #[strum(props(Code = "LIT", Name = "Loki Time", Offset = "-1530.72", East = "330", West = "300"))]
    /// Io Coordinated Time - 4 
    IoTCn4,
    #[strum(props(Code = "DBT", Name = "Dazhbog Time", Offset = "-1147.04", East = "300", West = "270"))]
    /// Io Coordinated Time - 3,
    IoTCn3,
    #[strum(props(Code = "DET", Name = "Danube Time", Offset = "-765.36", East = "270", West = "240"))]
    /// Io Coordinated Time - 2
    IoTCn2,
    #[strum(props(Code = "RNT", Name = "Reiden Time", Offset = "-382.68", East = "240", West = "210"))]
    /// Io Coordinated Time - 1
    IoTCn1,
    #[default]
    #[strum(props(Code = "ZAT", Name = "Zamama Time", Offset = "0.0", East = "210", West = "180"))]
    /// Io Coordinated Time
    IoTC,
    #[strum(props(Code = "PST", Name = "Prometheus Time", Offset = "382.68", East = "180", West = "150"))]
    /// Io Coordinated Time + 1
    IoTCp1,
    #[strum(props(Code = "TT", Name = "Thor Time", Offset = "765.36", East = "150", West = "120"))]
    /// Io Coordinated Time + 2
    IoTCp2,
    #[strum(props(Code = "AIT", Name = "Amirani Time", Offset = "1147.04", East = "120", West = "90"))]
    /// Io Coordinated Time + 3
    IoTCp3,
    #[strum(props(Code = "GBT", Name = "Gish Bar Time", Offset = "1530.72", East = "90", West = "60"))]
    /// Io Coordinated Time + 4 
    IoTCp4,
    #[strum(props(Code = "MIT", Name = "Masubi Time", Offset = "1913.40", East = "60", West = "30"))]
    /// Io Coordinated Time + 5
    IoTCp5,
    #[strum(props(Code = "KT", Name = "Kane Time", Offset = "2296.08", East = "30", West = "0"))]
    /// Io Coordinated Time + 6
    IoTCp6,
}
