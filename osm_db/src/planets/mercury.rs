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