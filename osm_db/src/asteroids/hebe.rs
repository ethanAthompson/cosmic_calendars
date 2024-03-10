use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, DateTime, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis},planets::EARTH_ROTATIONAL_PERIOD
};

use chrono::Datelike;
use julian_day_converter::JULIAN_DAY_UNIX_EPOCH_DAYS;
use strum::{AsRefStr, EnumProperty, VariantArray};

#[derive(Debug, Copy, Clone)]
/// This structure represents the second planet from the sun
pub struct Hebe;


impl Body for Hebe {
    /// A.D 1847 July 1, 12:00:00
    fn epoch(&self) -> f64 {
        2395844e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.2027
    }

    fn orbital_period(&self) -> f64 {
        1_380.65
    }

    fn rotational_period(&self) -> f64 {
        26_280.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        2.4249250 
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


/// Similar format as mars..
///
/// 
/// From:
/// 
/// Elevation map (in km) of (6) Hebe, with respect to,
/// a volume-equivalent ellipsoid best fitting our 3D-shape model.
/// The five major depressions are identified by numbers. 
/// 
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Hebeian {
    #[strum(props(Code = "FIT", Name = "First Depression Indigo Time", Offset = "-4.38", East = "0", West = "30"))]
    /// Hebe Coordinated Time - 6
    HeTC6,
    #[strum(props(Code = "FMT", Name = "First Depression Mount Time", Offset = "-3.65", East = "30", West = "60"))]
    /// Hebe Coordinated Time - 5
    HeTCn5,
    #[strum(props(Code = "SDIT", Name = "Second Depression Indigo Time", Offset = "-2.92", East = "60", West = "90"))]
    /// Hebe Coordinated Time - 4
    HeTCn4,
    #[strum(props(Code = "SDMT", Name = "Second Depression Mount Time", Offset = "-2.19", East = "90", West = "120"))]
    /// Hebe Coordinated Time - 3
    HeTCn3,
    #[strum(props(Code = "FPT", Name = "First Plain Time", Offset = "-1.46", East = "120", West = "150"))]
    /// Hebe Coordinated Time - 2
    HeTCn2,
    #[strum(props(Code = "TDIT", Name = "Third Depression Indigo Time", Offset = "-0.73", East = "150", West = "180"))]
    /// Hebe Coordinated Time - 1
    HeTCn1,
    #[default]
    #[strum(props(Code = "TDMT", Name = "Third Depression Mount Time", Offset = "0.0", East = "180", West = "210"))]
    /// Hebe Coordinated Time
    HeTC,
    #[strum(props(Code = "SPT", Name = "Second Plain Time", Offset = "0.73", East = "210", West = "240"))]
    /// Hebe Coordinated Time + 1
    HeTCp1,
    #[strum(props(Code = "DNT", Name = "Fourth Depression Noir Time", Offset = "1.46", East = "240", West = "270"))]
    /// Hebe Coordinated Time + 2
    HeTCp2,
    #[strum(props(Code = "CNAT", Name = "Converged Depression Azure Time", Offset = "2.19", East = "270", West = "300"))]
    /// Hebe Coordinated Time + 3
    HeTCp3,
    #[strum(props(Code = "DAT", Name = "Fifth Depression Azure Time", Offset = "2.92", East = "300", West = "330"))]
    /// Hebe Coordinated Time + 4
    HeTCp4,
    #[strum(props(Code = "DMT", Name = "Fifth Depression Mount Time", Offset = "3.65", East = "330", West = "360"))]
    /// Hebe Coordinated Time + 5
    HeTCp5,
}


impl TimeZone for Hebeian {
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
        Hebe.rotational_period() / EARTH_ROTATIONAL_PERIOD
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