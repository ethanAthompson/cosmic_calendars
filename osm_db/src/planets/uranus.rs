
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
/// This structure represents the seventh planet from the sun
pub struct Uranus;


/// Similar to Neptune
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Uranian {
    #[strum(props(Code = "NAT", Name = "Nova Time", Offset = "-8", East = "-180", West = "-162"))]
    /// Uranus Coordinated Time - 5
    UrTCn5,
    #[strum(props(Code = "MT", Name = "Marine Time", Offset = "-6.44", East = "-162", West = "-126"))]
    /// Uranus Coordinated Time - 4
    UrTCn4,
    #[strum(props(Code = "MET", Name = "Mirage Time", Offset = "-4.88", East = "-126", West = "-90"))]
    /// Uranus Coordinated Time - 3
    UrTCn3,
    #[strum(props(Code = "WRT", Name = "Whisper Time", Offset = "-3.32", East = "-90", West = "-54"))]
    /// Uranus Coordinated Time - 2
    UrTCn2,
    #[strum(props(Code = "NST", Name = "Nimbus Time", Offset = "-1.72", East = "-54", West = "-18"))]
    /// Uranus Coordinated Time - 1
    UrTCn1,
    #[default]
    #[strum(props(Code = "SPL", Name = "Spiral Time", Offset = "0.0", East = "-18", West = "18"))]
    /// Uranus Coordinated Time
    UrTC,
    #[strum(props(Code = "CET", Name = "Cascade Time", Offset = "1.72", East = "18", West = "54"))]
    /// Uranus Coordinated Time + 1
    UrTCp1,
    #[strum(props(Code = "SYT", Name = "Serenity Time", Offset = "3.32", East = "54", West = "90"))]
    /// Uranus Coordinated Time + 2
    UrTCp2,
    #[strum(props(Code = "TTT", Name = "Twilight Time", Offset = "4.88", East = "90", West = "126"))]
    /// Uranus Coordinated Time + 3
    UrTCp3,
    #[strum(props(Code = "LT", Name = "Legacy Time", Offset = "6.44", East = "126", West = "162"))]
    /// Uranus Coordinated Time + 4
    UrTCp4,
    #[strum(props(Code = "ZRT", Name = "Zephyr Time", Offset = "8", East = "162", West = "180"))]
    /// Uranus Coordinated Time + 5
    UrTCp5,
    
}