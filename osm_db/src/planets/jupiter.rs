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

impl Body for Jupiter {
    /// A.D 1972 March 2, 12:00:00
    fn epoch(&self) -> f64 {
        2.441379e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.048775
    }

    fn orbital_period(&self) -> f64 {
        4_332.59
    }

    fn rotational_period(&self) -> f64 {
        35_700.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        778.479
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
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Jupiterian {
    #[strum(props(Code = "LYT", Name = "Luminary Time", Offset = "-5.9599998", East = "-180", West = "-150"))]
    /// Jupiter Coordinated Time - 6
    JTCn6,
    #[strum(props(Code = "SLT", Name = "Sentinel Time", Offset = "-4.9666665", East = "-150", West = "-120"))]
    /// Jupiter Coordinated Time - 5
    JTCn5,
    #[strum(props(Code = "AXT", Name = "Apex Time", Offset = "-3.9733332", East = "-120", West = "-90"))]
    /// Jupiter Coordinated Time - 4
    JTCn4,
    #[strum(props(Code = "NET", Name = "Nebulae Time", Offset = "-2.9799999", East = "-90", West = "-60"))]
    /// Jupiter Coordinated Time - 3
    JTCn3,
    #[strum(props(Code = "ET", Name = "Echo Time", Offset = "-1.9866666", East = "-60", West = "-30"))]
    /// Jupiter Coordinated Time - 2
    JTCn2,
    #[strum(props(Code = "RAT", Name = "Redstormia Time", Offset = "-0.9933333", East = "-30", West = "0"))]
    /// Jupiter Coordinated Time - 1
    JTCn1,
    #[default]
    #[strum(props(Code = "CTT", Name = "Citadel Time", Offset = "0.0", East = "0", West = "30"))]
    /// Jupiter Coordinated Time
    JTC,
    #[strum(props(Code = "JAT", Name = "Jovia Time", Offset = "0.9933333", East = "30", West = "60"))]
    /// Jupiter Coordinated Time + 1
    JTCp1,
    #[strum(props(Code = "SPT", Name = "Scapeia Time", Offset = "1.9866666", East = "60", West = "90"))]
    /// Jupiter Coordinated Time + 2
    JTCp2,
    #[strum(props(Code = "SST", Name = "Solis Time", Offset = "2.9799999", East = "90", West = "120"))]
    /// Jupiter Coordinated Time + 3
    JTCp3,
    #[strum(props(Code = "AAT", Name = "Aurora Time", Offset = "3.9733332", East = "120", West = "150"))]
    /// Jupiter Coordinated Time + 4
    JTCp4,
    #[strum(props(Code = "SAT", Name = "Solaria Time", Offset = "4.9666665", East = "150", West = "180"))]
    /// Jupiter Coordinated Time + 5
    JTCp5,
}

impl TimeZone for Jupiterian {
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
        Jupiter.rotational_period() / EARTH_ROTATIONAL_PERIOD
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

impl Jupiter {
    /// This method was inspired by chrono, so you can see the live mars date
    pub fn now(&mut self, offset: Jupiterian) -> DateTime {
        let now = chrono::Utc::now();
        let now = crate::julian::Julian.get_jd(now.year(), now.month() as i32, now.day() as i32, Jupiterian::offset(&offset));

        let date = self.to_date(now);
        let time = Jupiterian::now(&offset);

        DateTime { date, time }
    }
}
