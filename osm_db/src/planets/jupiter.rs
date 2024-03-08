
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

/* This is the least redundant and easiest way to use chatGPT for such tedious work..


    Ask ChatGPT to create a list of names, in this format
    #[strum(props(
        Code = "AMT",
        Name = "Amazonis Time",
        Offset = "-12.5",
        East = "-180",
        West = "-162"
    ))]
    /// Mars Coordinated Time - 5
    MTCn5,

    * Replace MTCn5 with JTCn5 (where n5 can go up to n48 -> p48 where n0 is just JTC) *
    * Replace Mars Coordinated Time with Jupiter Coordinated Time (+ or - n)
    * Replace East with specific value given the increment you specifiy
    * Replace the offset given to you by the "Main Astronomy Calculations", where its added up from  <-- JTC -->
    * Replace Name with the custom names given, so ".... Time"
    * Replace Code with the code names for each custom name..
*/
// #[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
// pub enum Jupiterian {

// }

// impl Body for Jupiter {}

// impl TimeZone for Jupiterian {}