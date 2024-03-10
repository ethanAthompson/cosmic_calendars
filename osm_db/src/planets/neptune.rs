
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
/// This structure represents the eighth planet from the sun
pub struct Neptune;

/// Similar to Jupiter
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Neptunian {
    #[strum(props(Code = "AET", Name = "Aquazure Time", Offset = "-8", East = "-180", West = "-162"))]
    /// Neptune Coordinated Time - 5
    NTCn5,
    #[strum(props(Code = "MET", Name = "Marine Time", Offset = "-6.4", East = "-162", West = "-126"))]
    /// Neptune Coordinated Time - 4
    NTCn4,
    #[strum(props(Code = "VLT", Name = "Veil Time", Offset = "-4.8", East = "-126", West = "-90"))]
    /// Neptune Coordinated Time - 3
    NTCn3,
    #[strum(props(Code = "AET", Name = "Azure Time", Offset = "-3.2", East = "-90", West = "-54"))]
    /// Neptune Coordinated Time - 2
    NTCn2,
    #[strum(props(Code = "BSAT", Name = "Bluestormia Time", Offset = "-1.6", East = "-54", West = "-18"))]
    /// Neptune Coordinated Time - 1
    NTCn1,
    #[default]
    #[strum(props(Code = "NLT", Name = "Nautical Time", Offset = "0.0", East = "-18", West = "18"))]
    /// Neptune Coordinated Time
    NTC,
    #[strum(props(Code = "EAT", Name = "Enigma Time", Offset = "1.6", East = "18", West = "54"))]
    /// Neptune Coordinated Time + 1
    NTCp1,
    #[strum(props(Code = "SYT", Name = "Symphony Time", Offset = "3.2", East = "54", West = "90"))]
    /// Neptune Coordinated Time + 2
    NTCp2,
    #[strum(props(Code = "EBT", Name = "Ebb Time", Offset = "4.8", East = "90", West = "126"))]
    /// Neptune Coordinated Time + 3
    NTCp3,
    #[strum(props(Code = "AAT", Name = "Aquatica Time", Offset = "6.4", East = "126", West = "162"))]
    /// Neptune Coordinated Time + 4
    NTCp4,
    #[strum(props(Code = "SRT", Name = "Seafarer Time", Offset = "8", East = "162", West = "180"))]
    /// Neptune Coordinated Time + 5
    NTCp5,
}
