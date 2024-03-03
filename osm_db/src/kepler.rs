use displaydoc::Display;
use strum::AsRefStr;

use crate::{
    orbit::{MeanMotion, Perihelion, Season, SemiAxis, SolarLongitude, Type},
    planets::EARTH_ROTATIONAL_PERIOD,
};

/// This trait acts as a common field for all planets, asteroids, moons, exo-planets, and comets
///
/// ## Limitations
/// `Only Solar`: Sidereal days is not supported
///
pub trait Body {
    /// Calculates the reference point which the body was discovered
    fn epoch(&self) -> f64;
    /// Calculates the deviation of an orbit's path from a perfect circle.
    fn orbital_eccentricity(&self) -> f64;
    /// Calculates the days in time it takes a body to orbit a host body that's the sun or a planet.
    fn orbital_period(&self) -> f64;
    /// Calculates the seconds in time it takes a body to rotate on its' axis.
    fn rotational_period(&self) -> f64;
    /// A wrapper that's shared throughout the code
    fn perihelion(&self) -> Perihelion;
    /// Calculates the average distance of this body from the sun.
    fn semimajor(&self) -> f64;
    /// Calculates the shortest distance between the center of the body to the edge of the body.
    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }
    /// Calculates the mean motion which is the perihelian elapse.
    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(
            &mut MeanMotion,
            day,
            self.perihelion(),
            self.orbital_period(),
        )
    }
    /// Final Calculation into date
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


#[derive(Debug, Default, AsRefStr, Clone, Copy)]
/// This represents eras that the date is in
pub enum Eras {
    #[strum(serialize = "AD")]
    /// After Discovery, this must be a present or latter date.
    AD,
    #[strum(serialize = "BD")]
    /// Before Discovery, this must be an old date.
    BD,
    #[default]
    /// Unkown, you must've entered something wrong.
    Unknown,
}


#[derive(Display, Debug, Default, Clone)]
/// This is a collection of what a date should consist of
pub struct Date {
    /// This is the era of body
    pub era: Eras,
    /// This is the year of body
    pub year: i32,
    /// This is the month of body
    pub month: u8,
    /// This is the day of body
    pub day: f64,
    /// This is the ls of the body
    pub ls: f64,
    /// This is the season of the body (Optional)
    pub season: String,
}

impl Date {
    /// This method is a wrapper to compute the date of a body/
    ///
    /// The `1.0` is added to make sure that year, month, or day is not 0.
    ///
    /// Some planets may have different dates for seasons,
    /// the seasons are baesd on the 360 orbital path.
    ///
    pub fn compute(
        &self,
        julian_date: f64,
        epoch: f64,
        rotational_period: f64,
        mut peri: Perihelion,
        semimajor: f64,
        orbital_eccentricity: f64,
        orbital_period: f64,
    ) -> Self {
        let mut tmp_year = 12.0;
        let mut tmp_day = (julian_date - epoch) * EARTH_ROTATIONAL_PERIOD / rotational_period;

        let shape = Type::default().shape(orbital_eccentricity);

        while tmp_day >= orbital_period {
            tmp_day -= orbital_period;
            tmp_year += 1.0;
        }

        while tmp_day < 0.0 {
            tmp_day += orbital_period;
            tmp_year -= 1.0;
        }

        let ls = SolarLongitude.compute(
            shape,
            tmp_day,
            orbital_eccentricity,
            peri,
            orbital_period,
            semimajor,
        );

        let year = tmp_year;
        let month = 1.0 + (ls / peri.avg_ls()).floor();
        let day = 1.0 + tmp_day.floor();
        let season = Season::default().from(ls as u32);
        let era = match year as i32 > 0 {
            true => Eras::AD,
            false => Eras::BD,
        };

        Self {
            era,
            year: year as i32,
            month: month as u8,
            day,
            ls,
            season,
        }
    }
}


#[derive(Display, Debug, Default, Clone)]
/// This is a collection of what a time should consist of
pub struct Time {
    /// This is the hour of the body
    pub hour: i32,
    /// This is the minute of the body
    pub minute: u8,
    /// This is the second of the body
    pub second: u8,
    /// This is the offset code of the body
    pub code: String,
    /// This is the name code of the body
    pub name: String,
    /// This is the offset name code of the body
    pub offset_name: String,
    /// This is the hour type of body (Millitary Time or 12Hour)
    pub hour_type: String,
}


#[derive(Display, Debug, Default, Clone)]
/// This is a collection of what a time and date should consist of, although seperately
pub struct DateTime {
    /// This is the date of the body
    pub date: Date,
    /// This is the time of the body
    pub time: Time
}

/// This trait acts as a common field for all  all planets, asteroids, moons, exo-planets, and comets.
///
/// The timezone is implemented for specific timezones
/// because each timezone has specific calculations to generate a time from UTC.
///
///
pub trait TimeZone {
    /// This method gets the millisencds since j2000 epoch
    fn millis(&self) -> f64;
    /// This method calibrates the body specific timezones
    fn offset(&self) -> f64;    
    /// This method calculates the julian day offset from the coordinated time offset
    fn julian_offset(&self) -> f64;
    /// This method gets the jd_ut of the timezone
    fn julian_date_universal_time(&self) -> f64;
    /// This method calculates (body / host) rotational periods
    ///
    /// Body Earth Ratio
    ///  * body_rotational_period / earth_rotational_period
    ///
    /// Body Moon Ratio
    ///  * moon_rotational_period / body_rotational_period (host planet of the exact moon)
    ///
    fn body_host_ratio(&self) -> f64;
    /// This method gets the jd_tt of the timezone
    fn julian_date_terrestial_time(&self) -> f64;
    /// This method gets the jd2000 time
    fn julian_date_2000_time(&self) -> f64;
    /// This method gets the day date for a timezone (msd, vsd, ...)
    fn day_date(&self) -> f64;
    /// This method is the fractional hour that splits the day_date to hours
    fn fractional_hour(&self) -> f64;
    /// This method is the fractional hour that splits the day_date to minutes
    fn fractional_minute(&self) -> f64;
    /// This method gets the coordinated time
    fn coordinated_time(&self) -> f64;
    /// This method generates a new timezone and returns the time for it
    fn now(&self) -> Time;
}


#[derive(Display, Debug, Clone, Copy, Default, AsRefStr)]
/// The hour type of the timezone
pub enum HourType {
    /// Ante Meridiem
    #[strum(serialize = "AM")]
    AM,
    /// Post meridiem
    #[strum(serialize = "PM")]
    PM,
    /// The base case of an hour type
    #[default]
    #[strum(serialize = "Unknown")]
    Unknown,
}

impl HourType {
    /// This method computes the hour type of the time given an hour.
    /// 
    /// This is according to military time not standard time
    pub fn new(&self, hour: u8) -> String {
        match hour {
            0..=11 => Self::AM,
            12..=24 => Self::PM,
            _ => Self::Unknown,
        }
        .as_ref()
        .to_string()
    }
}
