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
pub struct Ganymede;

/// Similar format as mars..
///
/// The difference is its from +/- 240 instead of 180..
///
///
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Ganymedian {
    #[strum(props(Code = "KIT", Name = "Kadi Time", Offset = "-153.90", East = "180", West = "-170"))]
    /// Ganymede Coordinated Time - 9
    GTCn9,
    #[strum(props(Code = "LUT", Name = "Lakhmu Time", Offset = "-136.80", East = "-170", West = "-150"))]
    /// Ganymede Coordinated Time - 8
    GTCn8,
    #[strum(props(Code = "GOT", Name = "Galileo Time", Offset = "-119.70", East = "-150", West = "-130"))]
    /// Ganymede Coordinated Time - 7
    GTCn7,
    #[strum(props(Code = "NAT", Name = "Nidaba Time", Offset = "-102.60", East = "-130", West = "-110"))]
    /// Ganymede Coordinated Time - 6
    GTCn6,
    #[strum(props(Code = "SKT", Name = "Selket Time", Offset = "-85.50", East = "-110", West = "-90"))]
    /// Ganymede Coordinated Time - 5
    GTCn5,
    #[strum(props(Code = "XAT", Name = "Xibalba Time", Offset = "-68.40", East = "-90", West = "-70"))]
    /// Ganymede Coordinated Time - 4
    GTCn4,
    #[strum(props(Code = "NHT", Name = "Nineveh Time", Offset = "-51.30", East = "-70", West = "-50"))]
    /// Ganymede Coordinated Time - 3
    GTCn3,
    #[strum(props(Code = "PET", Name = "Perrine Time", Offset = "-34.20", East = "-50", West = "-30"))]
    /// Ganymede Coordinated Time - 2
    GTCn2,
    #[strum(props(Code = "SNT", Name = "Sicyon Time", Offset = "-17.10", East = "-30", West = "-10"))]
    /// Ganymede Coordinated Time - 1
    GTCn1,
    #[default]
    #[strum(props(Code = "SUT", Name = "Shu Time", Offset = "0.0", East = "-10", West = "10"))]
    /// Ganymede Coordinated Time
    GTC,
    #[strum(props(Code = "ZRT", Name = "Zakar Time", Offset = "17.10", East = "10", West = "30"))]
    /// Ganymede Coordinated Time + 1
    GTCp1,
    #[strum(props(Code = "HAT", Name = "Harpagia Time", Offset = "34.20", East = "30", West = "50"))]
    /// Ganymede Coordinated Time + 2
    GTCp2,
    #[strum(props(Code = "HST", Name = "Haroeris Time", Offset = "51.30", East = "50", West = "70"))]
    /// Ganymede Coordinated Time + 3
    GTCp3,
    #[strum(props(Code = "TST", Name = "Thetis Time", Offset = "68.40", East = "70", West = "90"))]
    /// Ganymede Coordinated Time + 4
    GTCp4,
    #[strum(props(Code = "HFT", Name = "Hershef Time", Offset = "85.50", East = "90", West = "110"))]
    /// Ganymede Coordinated Time + 5
    GTCp5,
    #[strum(props(Code = "AST", Name = "Agreus Time", Offset = "102.60", East = "110", West = "130"))]
    /// Ganymede Coordinated Time + 6
    GTCp6,
    #[strum(props(Code = "ANT", Name = "Amon Time", Offset = "119.70", East = "130", West = "150"))]
    /// Ganymede Coordinated Time + 7
    GTCp7,
    #[strum(props(Code = "MST", Name = "Marius Time", Offset = "136.80", East = "150", West = "170"))]
    /// Ganymede Coordinated Time + 8
    GTCp8,
    #[strum(props(Code = "MKT", Name = "Melkart Time", Offset = "153.90", East = "170", West = "180"))]
    /// Ganymede Coordinated Time + 9
    GTCp9,
}
