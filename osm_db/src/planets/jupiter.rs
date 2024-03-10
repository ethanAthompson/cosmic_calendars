use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, DateTime, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis},
};

use chrono::Datelike;
use julian_day_converter::JULIAN_DAY_UNIX_EPOCH_DAYS;
use strum::{AsRefStr, EnumProperty, VariantArray};

use super::EARTH_ROTATIONAL_PERIOD;

/* https://chat.openai.com/c/f9ec6687-7a19-4af0-9538-b449577bc954 */
/* use this maybe?  https://search.brave.com/search?q=23.933333+hours+to+secons&source=desktop*/

#[derive(Debug, Copy, Clone)]
/// This structure represents the fifth planet from the sun

pub struct Jupiter;

/// Similar format as mars..
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Jupiterian {
    #[strum(props(Code = "LYT", Name= "Luminary Time", Offset = "-5.9599998", East = "-180", West = "-150"))]
    /// Jupiter Coordinated Time - 6
    JTCn6,
    #[strum(props(Code = "SLT", Name= "Sentinel Time", Offset = "-4.9666665", East = "-150", West = "-120"))]
    /// Jupiter Coordinated Time - 5
    JTCn5,
    #[strum(props(Code = "AXT", Name= "Apex Time", Offset = "-3.9733332", East = "-120", West = "-90"))]
    /// Jupiter Coordinated Time - 4
    JTCn4,
    #[strum(props(Code = "NET", Name= "Nebulae Time", Offset = "-2.9799999", East = "-90", West = "-60"))]
    /// Jupiter Coordinated Time - 3
    JTCn3,
    #[strum(props(Code = "ET", Name= "Echo Time", Offset = "-1.9866666", East = "-60", West = "-30"))]
    /// Jupiter Coordinated Time - 2
    JTCn2,
    #[strum(props(Code = "RAT", Name= "Redstormia Time", Offset = "-0.9933333", East = "-30", West = "0"))]
    /// Jupiter Coordinated Time - 1
    JTCn1,
    #[default]
    #[strum(props(Code = "CTT", Name= "Citadel Time", Offset = "0.0", East = "0", West = "30"))]
    /// Jupiter Coordinated Time
    JTC,
    #[strum(props(Code = "JAT", Name= "Jovia Time", Offset = "0.9933333", East = "30", West = "60"))]
    /// Jupiter Coordinated Time + 1
    JTCp1,
    #[strum(props(Code = "SPT", Name= "Scapeia Time", Offset = "1.9866666", East = "60", West = "90"))]
    /// Jupiter Coordinated Time + 2
    JTCp2,
    #[strum(props(Code = "SST", Name= "Solis Time", Offset = "2.9799999", East = "90", West = "120"))]
    /// Jupiter Coordinated Time + 3
    JTCp3,
    #[strum(props(Code = "AAT", Name= "Aurora Time", Offset = "3.9733332", East = "120", West = "150"))]
    /// Jupiter Coordinated Time + 4
    JTCp4,
    #[strum(props(Code = "SAT", Name= "Solaria Time", Offset = "4.9666665", East = "150", West = "180"))]
    /// Jupiter Coordinated Time + 5
    JTCp5,
}
