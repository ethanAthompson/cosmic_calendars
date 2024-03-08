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
/// This structure represents the fourth planet from the sun
pub struct Mars;

#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
/// This structure represents the martian timezone
///
/// Offset is in 1 decisol, (-2.5 west, +2.5 east)
///
/// There is no DST on mars
///
/// 1 sol = 25 hours
/// 1 decisol = 2.5 hours
///
/// 12.5 + 12.5 = 25
/// MTC-5 to MTC+5 is 25 hours
pub enum Martian {
    #[strum(props(
        Code = "AMT",
        Name = "Amazonis Time",
        Offset = "-12.5",
        East = "-180",
        West = "-162"
    ))]
    /// Mars Coordinated Time - 5
    MTCn5,
    #[strum(props(
        Code = "OT",
        Name = "Olympus Time",
        Offset = "-10.0",
        East = "-162",
        West = "-126"
    ))]
    /// Mars Coordinated Time - 4
    MTCn4,
    #[strum(props(
        Code = "TT",
        Name = "Tharsis Time",
        Offset = "-7.5",
        East = "-126",
        West = "-90"
    ))]
    /// Mars Coordinated Time - 3
    MTCn3,
    #[strum(props(
        Code = "MT",
        Name = "Marineris Time",
        Offset = "-5.0",
        East = "-90",
        West = "-54"
    ))]
    /// Mars Coordinated Time - 2
    MTCn2,
    #[strum(props(
        Code = "AGT",
        Name = "Argyre Time",
        Offset = "-2.5",
        East = "-54",
        West = "-18"
    ))]
    /// Mars Coordinated Time - 1
    MTCn1,
    #[default]
    #[strum(props(
        Code = "NT",
        Name = "Noachis Time",
        Offset = "0.0",
        East = "-18",
        West = "18"
    ))]
    /// Mars Coordinated Time
    MTC,
    #[strum(props(
        Code = "ABT",
        Name = "Arabia Time",
        Offset = "2.5",
        East = "18",
        West = "54"
    ))]
    /// Mars Coordinated Time + 1
    MTCp1,
    #[strum(props(
        Code = "HT",
        Name = "Hellas Time",
        Offset = "5.0",
        East = "54",
        West = "90"
    ))]
    /// Mars Coordinated Time + 2
    MTCp2,
    #[strum(props(
        Code = "UT",
        Name = "Utopia Time",
        Offset = "7.5",
        East = "90",
        West = "126"
    ))]
    /// Mars Coordinated Time + 3
    MTCp3,
    #[strum(props(
        Code = "ET",
        Name = "Elysium Time",
        Offset = "10.0",
        East = "126",
        West = "162"
    ))]
    /// Mars Coordinated Time + 4
    MTCp4,
    #[strum(props(
        Code = "ACT",
        Name = "Arcadia Time",
        Offset = "12.5",
        East = "162",
        West = "180"
    ))]
    /// Mars Coordinated Time + 5
    MTCp5,
}

impl Body for Mars {
    /// A.D 1975 December 19, 04:00:00.3
    fn epoch(&self) -> f64 {
        2.442765667e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0934
    }

    fn orbital_period(&self) -> f64 {
        668.6
    }

    fn rotational_period(&self) -> f64 {
        88_775.245
    }
    
    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion {
            month: (468.5, 514.6),
            ls: (240.0, 270.0),
            perihelion: 251.0,
        }
    }

    fn semimajor(&self) -> f64 {
        1.52
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(
            &mut MeanMotion,
            day,
            self.perihelion(),
            self.orbital_period(),
        )
    }

    fn to_date(&mut self, julian_date: f64) -> Date {
        Date::default().compute(
            julian_date,
            self.epoch(),
            self.rotational_period(),
            self.perihelion(),
            self.semimajor(),
            self.orbital_eccentricity(),
            self.orbital_period(),
        )
    }
}

impl TimeZone for Martian {
    fn millis(&self) -> f64 {
        chrono::Utc::now().timestamp_millis() as f64
    }

    fn offset(&self) -> f64 {
        self.get_str("Offset")
            .unwrap()
            .parse::<f64>()
            .expect("Offset to be established")
    }

    fn julian_offset(&self) -> f64 {
        JULIAN_DAY_UNIX_EPOCH_DAYS
            - self
                .get_str("Offset")
                .unwrap()
                .parse::<f64>()
                .expect("Offset to be established")
                / 24.0
    }

    fn julian_date_universal_time(&self) -> f64 {
        let number_of_days: f64 = 8.64 * 10.0_f64.powf(7.0);

        // coordinates the offset instead of JULIAN_DAY_UNIX_EPOCH_DAYS alone
        self.julian_offset() + (self.millis() / number_of_days)
    }

    fn body_host_ratio(&self) -> f64 {
        // ! This is the only thing you have to change physically
        Mars.rotational_period() / EARTH_ROTATIONAL_PERIOD
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

    // mars sol date is a martian version of the julian date
    fn day_date(&self) -> f64 {
        // midnight for mars prime meridian (jan 6th 2000)
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

impl Mars {
    /// This method was inspired by chrono, so you can see the live mars date
    pub fn now(&mut self, offset: Martian) -> DateTime {
        let now = chrono::Utc::now();
        let now = crate::julian::Julian.get_jd(
            now.year(),
            now.month() as i32,
            now.day() as i32,
            Martian::offset(&offset),
        );

        let date = self.to_date(now);
        let time = Martian::now(&offset);

        DateTime { date, time }
    }
}
