
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
/// This structure represents the sixth planet from the sun
pub struct Saturn;


/// 1.07
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Saturnian {
    #[strum(props(Code="DROT", Name="D-Ring Outer Time", Offset="-7.49"))]
    /// Saturn Coordinated Time - 7
    STCn7,
    #[strum(props(Code="DRIT", Name="D-Ring Inner Time", Offset="-6.42"))]
    /// Saturn Coordinated Time - 6
    STCn6,
    #[strum(props(Code="CGMPT", Name="Colombo Gap Midpoint Time", Offset="-5.35"))]
    /// Saturn Coordinated Time - 5
    STCn5,
    #[strum(props(Code="CGT", Name="Colombo Gap Time", Offset="-4.28"))]
    /// Saturn Coordinated Time - 4
    STCn4,
    #[strum(props(Code="CRT", Name="C-Ring Time", Offset="-3.21"))]
    /// Saturn Coordinated Time - 3
    STCn3,
    #[strum(props(Code="MGT", Name="Maxwell Gap Time", Offset="-2.14"))]
    /// Saturn Coordinated Time - 2
    STCn2,
    #[strum(props(Code="BRMT", Name="B-Ring Midpoint Time", Offset="-1.07"))]
    /// Saturn Coordinated Time - 1
    STCn1,
    #[default]
    #[strum(props(Code="BRT", Name="B-Ring Time", Offset="0.0"))]
    /// Saturn Coordinated Time
    STC,
    #[strum(props(Code="HP", Name="Huygens Gap Time", Offset="1.07"))]
    /// Saturn Coordinated Time + 1
    STCp1,
    #[strum(props(Code="CDP", Name="Cassini Division Time", Offset="2.14"))]
    /// Saturn Coordinated Time + 2
    STCp2,
    #[strum(props(Code="ART", Name="A-Ring Time", Offset="3.21"))]
    /// Saturn Coordinated Time + 3
    STCp3,
    #[strum(props(Code="EGT", Name="Eneke Gap Time", Offset="4.28"))]
    /// Saturn Coordinated Time + 4
    STCp4,
    #[strum(props(Code="KGT", Name="Keeler Gap Time", Offset="5.35"))]
    /// Saturn Coordinated Time + 5
    STCp5,
    #[strum(props(Code="BT", Name="Boundary Time", Offset="6.42"))]
    /// Saturn Coordinated Time + 6
    STCp6,
    #[strum(props(Code="FRT", Name="F-Ring Time", Offset="7.49"))]
    /// Saturn Coordinated Time + 7
    STCp7,
}

