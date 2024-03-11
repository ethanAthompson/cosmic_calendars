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

impl Mercury {
    /// This method was inspired by chrono, so you can see the live mars date
    pub fn now(&mut self, offset: Mercurian) -> DateTime {
        let now = chrono::Utc::now();
        let now = crate::julian::Julian.get_jd(now.year(), now.month() as i32, now.day() as i32, Mercurian::offset(&offset));

        let date = self.to_date(now);
        let time = Mercurian::now(&offset);

        DateTime { date, time }
    }
}


impl Body for Mercury {
    /// A.D 1959 September 9, 12:00:00
    fn epoch(&self) -> f64 {
        2.436821e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.2000
    }

    fn orbital_period(&self) -> f64 {
        88.00
    }

    fn rotational_period(&self) -> f64 {
        5_063_040.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        57.909
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


impl TimeZone for Mercurian {
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
        Mercury.rotational_period() / EARTH_ROTATIONAL_PERIOD
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