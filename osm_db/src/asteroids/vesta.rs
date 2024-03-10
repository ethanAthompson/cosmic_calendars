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
pub struct Vesta;

/// Similar format as mars..
///
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Vestanian {
    #[strum(props(Code = "AQT", Name = "Aquilia Time", Offset = "-3.18", East = "0", West = "30"))]
    /// Vesta Coordinated Time - 6
    V4TCn6,
    #[strum(props(Code = "CAT", Name = "Cannutia Time", Offset = "-2.65", East = "30", West = "60"))]
    /// Vesta Coordinated Time - 5
    V4TCn5,
    #[strum(props(Code = "COAT", Name = "Caesonia Time", Offset = "-2.12", East = "60", West = "90"))]
    /// Vesta Coordinated Time - 4
    V4TCn4,
    #[strum(props(Code = "AET", Name = "Aelia Time", Offset = "-1.59", East = "90", West = "120"))]
    /// Vesta Coordinated Time - 3
    V4TCn3,
    #[strum(props(Code = "CPAT", Name = "Calpurnia Time", Offset = "-1.06", East = "120", West = "150"))]
    /// Vesta Coordinated Time - 2
    V4TCn2,
    #[strum(props(Code = "GAT", Name = "Galeria Time", Offset = "-0.53", East = "150", West = "180"))]
    /// Vesta Coordinated Time - 1,
    V4TCn1,
    #[default]
    #[strum(props(Code = "AIT", Name = "Antonia Time", Offset = "0.0", East = "180", West = "150"))]
    /// Vesta Coordinated Time
    V4TC,
    #[strum(props(Code = "CAPT", Name = "Caparronia Time", Offset = "0.53", East = "150", West = "120"))]
    /// Vesta Coordinated Time + 1
    V4TCp1,
    #[strum(props(Code = "COT", Name = "Charito Time", Offset = "1.06", East = "120", West = "90"))]
    /// Vesta Coordinated Time + 2
    V4TCp2,
    #[strum(props(Code = "CLT", Name = "Canuleia Time", Offset = "1.59", East = "90", West = "60"))]
    /// Vesta Coordinated Time + 3
    V4TCp3,
    #[strum(props(Code = "AYPT", Name = "Alypia Time", Offset = "2.12", East = "60", West = "30"))]
    /// Vesta Coordinated Time + 4
    V4TCp4,
    #[strum(props(Code = "ANAT", Name = "Angioletta Time", Offset = "2.65", East = "30", West = "0"))]
    /// Vesta Coordinated Time + 5
    V4TCp5,
}
