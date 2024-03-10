use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, DateTime, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis}, planets::EARTH_ROTATIONAL_PERIOD
};

use chrono::Datelike;
use julian_day_converter::JULIAN_DAY_UNIX_EPOCH_DAYS;
use strum::{AsRefStr, EnumProperty, VariantArray};

#[derive(Debug, Copy, Clone)]
/// This structure represents the second planet from the sun
pub struct Ganymede;

impl Body for Ganymede {
    /// A.D 1610 January 7, 12:00:00
    fn epoch(&self) -> f64 {
        2309107e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0013
    }

    fn orbital_period(&self) -> f64 {
        7.16
    }

    fn rotational_period(&self) -> f64 {
        618_192.00
    }

    /// These numbers are derived from a formula..
    /// 1st, find the common ratio for each day (46.1) days for the perihelion months
    /// 2nd, find the average ls (30) ls per month
    fn perihelion(&self) -> Perihelion {
        Perihelion { month: (468.5, 514.6), ls: (240.0, 270.0), perihelion: 251.0 }
    }

    fn semimajor(&self) -> f64 {
        0.007155182
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

impl TimeZone for Ganymedian {
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
        Ganymede.rotational_period() / EARTH_ROTATIONAL_PERIOD
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