
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
/// This structure represents the second planet from the sun
pub struct Venus;
impl Venus {
    /// This method was inspired by chrono, so you can see the live mars date
    pub fn now(&mut self, offset: Venarian) -> DateTime {
        let now = chrono::Utc::now();
        let now = crate::julian::Julian.get_jd(now.year(), now.month() as i32, now.day() as i32, Venarian::offset(&offset));

        let date = self.to_date(now);
        let time = Venarian::now(&offset);

        DateTime { date, time }
    }
}
impl Body for Venus {
    /// A.D 1990 August 10, 12:00:00:0
    fn epoch(&self) -> f64 {
        2.448_114e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0070
    }

    fn orbital_period(&self) -> f64 {
        225.00
    }

    fn rotational_period(&self) -> f64 {
        20_995.20
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        108.210
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
pub enum Venarian {
    #[strum(props(Code = "AAT", Name= "Asteria Time", Offset = "-2332.8", East = "-240", West = "-270"))]
    /// Venus Coordinated Time - 4
    VTCn4,
    #[strum(props(Code = "TST", Name= "Themis Time", Offset = "-1749.6", East = "-270", West = "-300"))]
    /// Venus Coordinated Time - 3
    VTCn3,
    #[strum(props(Code = "GET", Name= "Guinevere Time", Offset = "-1166.4", East = "-300", West = "-330"))]
    /// Venus Coordinated Time - 2
    VTCn2,
    #[strum(props(Code = "LAT", Name= "Lavinia Time", Offset = "-583.2", East = "-330", West = "0"))]
    /// Venus Coordinated Time - 1
    VTCn1,
    #[default]
    #[strum(props(Code = "MLT", Name= "Mawell Time", Offset = "0.0", East = "0", West = "30"))]
    /// Venus Coordinated Time
    VTC,
    #[strum(props(Code = "LET", Name= "Leda Time", Offset = "583.2", East = "30", West = "60"))]
    /// Venus Coordinated Time + 1
    VTCp1,
    #[strum(props(Code = "TET", Name= "Tellus Time", Offset = "1166.4", East = "60", West = "90"))]
    /// Venus Coordinated Time + 2
    VTCp2,
    #[strum(props(Code = "OAT", Name= "Ovda Time", Offset = "1749.6", East = "90", West = "120"))]
    /// Venus Coordinated Time + 3
    VTCp3,
    #[strum(props(Code = "THT", Name= "Thetis Time", Offset = "2332.8", East = "120", West = "150"))]
    /// Venus Coordinated Time + 4
    VTCp4,
    #[strum(props(Code = "AST", Name= "Artemis Time", Offset = "2916", East = "150", West = "180"))]
    /// Venus Coordinated Time + 5
    VTCp5,
    #[strum(props(Code = "AAT", Name= "Atla Time", Offset = "3499.2", East = "180", West = "210"))]
    /// Venus Coordinated Time + 6
    VTCp6,
    #[strum(props(Code = "VNT", Name= "Venarian Time", Offset = "4082.4", East = "210", West = "240"))]
    /// Venus Coordinated Time + 7
    VTCp7,
}




impl TimeZone for Venarian {
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
        Venus.rotational_period() / EARTH_ROTATIONAL_PERIOD
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