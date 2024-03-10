
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
pub struct Titan;


impl Body for Titan {
    /// A.D 1655 March 25, 12:00:00
    fn epoch(&self) -> f64 {
        2325620e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0288
    }

    fn orbital_period(&self) -> f64 {
        15.92
    }

    fn rotational_period(&self) -> f64 {
        1_373_760.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        0.008167696
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
/// The difference is its from +/- 240 instead of 180..
/// 
/// 
#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty, VariantArray)]
pub enum Titanian {
    #[strum(props(Code = "TST", Name = "Tsegihi Time", Offset = "-1913.40", East = "360", West = "330"))]
    /// Titan Coordinated Time - 5
    TTCn5,
    #[strum(props(Code = "SOT", Name = "Senkyo Time", Offset = "-1530.72", East = "330", West = "300"))]
    /// Titan Coordinated Time - 4
    TTCn4,
    #[strum(props(Code = "HPT", Name = "Hetpet Time", Offset = "-1147.04", East = "300", West = "270"))]
    /// Titan Coordinated Time - 3
    TTCn3,
    #[strum(props(Code = "BTT", Name = "Belet Time", Offset = "-765.36", East = "270", West = "240"))]
    /// Titan Coordinated Time - 2
    TTCn2,
    #[strum(props(Code = "CNT", Name = "Caladan Time", Offset = "-382.68", East = "240", West = "210"))]
    /// Titan Coordinated Time - 1
    TTCn1,
    #[default]
    #[strum(props(Code = "DNT", Name = "Dilmun Time", Offset = "0.0", East = "210", West = "180"))]
    /// Titan Coordinated Time
    TTC,
    #[strum(props(Code = "SAT", Name = "Shangrila Time", Offset = "382.68", East = "180", West = "150"))]
    /// Titan Coordinated Time + 1
    TTCp1,
    #[strum(props(Code = "TT", Name = "Tui Time", Offset = "765.36", East = "150", West = "120"))]
    /// Titan Coordinated Time + 2
    TTCp2,
    #[strum(props(Code = "XUT", Name = "Xanadu Time", Offset = "1147.04", East = "120", West = "90"))]
    /// Titan Coordinated Time + 3
    TTCp3,
    #[strum(props(Code = "HIT", Name = "Hotei Time", Offset = "1530.72", East = "90", West = "60"))]
    /// Titan Coordinated Time + 4
    TTCp4,
    #[strum(props(Code = "MYT", Name = "Momoy Time", Offset = "1913.40", East = "60", West = "30"))]
    /// Titan Coordinated Time + 5
    TTCp5,
    #[strum(props(Code = "QAT", Name = "Quivira Time", Offset = "2296.08", East = "30", West = "0"))]
    /// Titan Coordinated Time + 6
    TTCp6,
}


impl TimeZone for Titanian {
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
        Titan.rotational_period() / EARTH_ROTATIONAL_PERIOD
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