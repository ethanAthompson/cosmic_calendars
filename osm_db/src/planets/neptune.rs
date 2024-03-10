
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


impl Body for Neptune {
    /// A.D 1846 September 23, 12:00:00
    fn epoch(&self) -> f64 {
        2395563e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.2000
    }

    fn orbital_period(&self) -> f64 {
        60_182.00
    }

    fn rotational_period(&self) -> f64 {
        57_420.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        4_514.953
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(&mut MeanMotion, day, self.perihelion(), self.orbital_period())
    }

    fn to_date(&mut self, julian_date: f64) -> Date {
        Date::default().compute(julian_date, self.epoch(), self.rotational_period(), self.perihelion(), self.semimajor(), self.orbital_eccentricity(), self.orbital_period())
    }
}

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


impl TimeZone for Neptunian {
    fn millis(&self) -> f64 {
        chrono::Utc::now().timestamp_millis() as f64
    }

    fn offset(&self) -> f64 {
        self.get_str("Offset").unwrap().parse::<f64>().expect("Offset to be established")
    }

    fn julian_offset(&self) -> f64 {
        JULIAN_DAY_UNIX_EPOCH_DAYS - self.get_str("Offset").unwrap().parse::<f64>().expect("Offset to be established") / 24.0
    }

    fn julian_date_universal_time(&self) -> f64 {
        let number_of_days: f64 = 8.64 * 10.0_f64.powf(7.0);

        // coordinates the offset instead of JULIAN_DAY_UNIX_EPOCH_DAYS alone
        self.julian_offset() + (self.millis() / number_of_days)
    }

    fn body_host_ratio(&self) -> f64 {
        Neptune.rotational_period() / EARTH_ROTATIONAL_PERIOD
    }

    fn julian_date_terrestial_time(&self) -> f64 {
        // leap seconds since January 1st, 2017
        let leap_seconds = 37.0 + 32.184;

        self.julian_date_universal_time() + (leap_seconds) / EARTH_ROTATIONAL_PERIOD
    }

    fn julian_date_2000_time(&self) -> f64 {
        // number of fractional days since noon on jan 1, 2000
        self.julian_date_terrestial_time() - JD2NOON
    }

    /* Adjust this to Jupiter standards */
    fn day_date(&self) -> f64 {
        let reference = self.julian_date_2000_time() - 4.5;

        // goes backwards to december 29th 1873
        let midday_positive = 44_796.0;

        // the adjustment by Mars24 site
        let adjustment = 0.00096;

        (reference / self.body_host_ratio()) + midday_positive - adjustment
    }

    fn coordinated_time(&self) -> f64 {
        let msd = self.day_date();

        (24.0 * msd) % 24.0
    }

    fn fractional_hour(&self) -> f64 {
        let msd = self.day_date();

        msd.fract()
    }

    fn fractional_minute(&self) -> f64 {
        (24.0 * self.fractional_hour()).fract()
    }

    fn now(&self) -> Time {
        let hour = (24.0 * self.fractional_hour()).floor();
        let minute = (60.0 * self.fractional_minute()).floor();
        let second = 60.0 * (60.0 * self.fractional_minute()).fract();

        Time {
            hour: hour as i32,
            minute: minute as u8,
            second: second as u8,
            code: self.get_str("Code").unwrap().to_string(),
            name: self.get_str("Name").unwrap().to_string(),
            offset_name: self.as_ref().to_string(),
            hour_type: HourType::new(&HourType::Unknown, hour as u8),
        }
    }
}