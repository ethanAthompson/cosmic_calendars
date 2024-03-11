use chrono::{Datelike, Local, NaiveDate, Utc};
use icu::datetime::input::{DateInput, IsoTimeInput};
use icu_calendar::{ethiopian::EthiopianEraStyle, types::Era, AsCalendar};
use std::str::FromStr;

use crate::{
    julian::JD2NOON,
    kepler::{Body, Date},
    orbit::{MeanMotion, Perihelion, SemiAxis},
};
#[derive(Debug, Copy, Clone)]
/// This structure represents our home planet
pub struct Earth;


impl Body for Earth {
    /// This is not correct..
    fn epoch(&self) -> f64 {
        0.0
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(
            &mut MeanMotion,
            day,
            self.perihelion(),
            self.orbital_period(),
        )
    }
    fn orbital_eccentricity(&self) -> f64 {
        0.0167
    }

    fn orbital_period(&self) -> f64 {
        365.256
    }

    // Convert me into seconds pls...
    fn rotational_period(&self) -> f64 {
        // ^ Seconds
        86159.9988

        // ^ Decimal Hours
        // 23.933333
    }

    fn semimajor(&self) -> f64 {
        149.598
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn to_date(&mut self, julian_date: f64) -> Date {
        let jd = crate::julian::Julian.jd2greg_named(julian_date);
        Date {
            era: crate::kepler::Eras::Unknown,
            year: jd.date.year,
            month: jd.date.month as u8,
            day: jd.date.day as f64,
            ls: 0.0,
            season: "N/A".to_string(),
        }
    }

    /// Earth's perihelion lasts 77 days according to these calculations
    fn perihelion(&self) -> Perihelion {
        Perihelion {
            // Jan -> March
            month: (1.0, 78.0),
            ls: (0.0, 90.0),
            perihelion: 251.0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
/// This is an earth structure to represent an earth date
pub struct EarthDate {
    /// This is the year for the earth
    pub year: i32,
    /// This is the month for the earth
    pub month: u32,
    /// This is the day for the earth
    pub day: u32,
}

impl EarthDate {
    /// Returns the current date now
    pub fn now() -> Self {
        let now = chrono::Utc::now();
        Self {
            year: now.year(),
            month: now.month(),
            day: now.day(),
        }
    }

    /// Returns the julian epoch or the jd2noon
    pub fn julian_epoch() -> Self {
        let epoch = crate::julian::Julian.jd2greg_named(JD2NOON);

        Self {
            year: epoch.date.year,
            month: epoch.date.month,
            day: epoch.date.day,
        }
    }
}

#[derive(Debug, Default)]
/// This is a collection of codes for the year, month, or day
pub struct EarthCodes {
    /// This is the code for the year
    pub year: String,
    /// This is the code for the month
    pub month: String,
    /// This is the code for the day
    pub day: String,
}

#[derive(Debug, Default, Copy, Clone)]
/// This is a structure that represents the earths' time
pub struct EarthTime {
    /// This is the hour on the earth
    pub hour: u8,
    /// This is the minute on the earth
    pub minute: u8,
    /// This is the second on the earth
    pub second: u8,
}

#[derive(Debug, Default)]
/// This is a structure that represents the combination of Earth time and date
pub struct EarthDateTime {
    /// This is the earths' date
    pub date: EarthDate,
    /// This is the earths' codes
    pub codes: EarthCodes,
    /// This is the earths' time
    pub time: EarthTime,
}

impl EarthDateTime {
    /// This method sets a date time given the input
    pub fn set_datetime(input: String) -> Vec<(String, String)> {
        if let Ok(time) = chrono_tz::Tz::from_str(input.as_str()) {
            let time = chrono::DateTime::with_timezone(&Utc::now(), &time).format("%Y/%m/%d %r %Z");
            vec![(input, time.to_string())]
        } else {
            let name = Local::now()
                .with_timezone(&Local::now().timezone())
                .to_string();
            let time = Local::now().format("%Y/%m/%d %r %Z");

            vec![(name, time.to_string())]
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
/// This is a structure that represents earth timezones
pub struct EarthTimeZones;

impl EarthTimeZones {
    /// Returns all timezones from chrono database
    pub fn all_timezones(mut list: Vec<String>) -> Vec<String> {
        for variant in chrono_tz::TZ_VARIANTS.iter() {
            list.push(variant.to_string())
        }

        list
    }
}

/** ## This is a declarative macro that abstracts the [`EarthDateTime::set_datetime`] method.

    > Takes in a location which returns the datetime + timezone for that location.
*/
#[macro_export]
macro_rules! set_datetimes {
    ($($location: expr), *) => {
        $(
            EarthDateTime::set_datetime($location.clone());
        )*
    };
}

/** This is a declarative macro wrapper that containerizes a calendar type.
 *
*/
#[macro_export]
macro_rules! getcal {
    ($cal: ty) => {};
}

/** This is a declarative macro wrapper which acts like a match statement for making datetimes

## DSL {gregorian | julian | ...} keywords
 > These are the keywords which when specified, are given access to its special scope

 ## param {expr} $n
 > This represents an ['EarthDateTime'] struct

*/
#[macro_export]
macro_rules! get_icu {
    (gregorian $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_gregorian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (julian $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_julian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (indian $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_indian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (iso $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_iso_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (chinese $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_chinese_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::chinese::Chinese::default(),
            )
            .unwrap(),
        )
    };
    (roc $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_roc_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (ethiopian $n: expr, $era: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_ethiopian_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (japanese $n: expr, $era: expr ) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_japanese_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::Japanese::default(),
            )
            .unwrap(),
        )
    };
    (persian $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_persian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (coptic $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_coptic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (hebrew $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_hebrew_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::hebrew::Hebrew::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (buddhist $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_buddhist_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (islamic_civil $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_islamic_civil_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicCivil::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_observational $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_observational_islamic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicObservational::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_tabular $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_islamic_tabular_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicTabular::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_umm_al_qura $n: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_ummalqura_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (japanese_extended $n: expr, $era: expr) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_japanese_extended_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::JapaneseExtended::default(),
            )
            .unwrap(),
        )
    };
}

/** This is a declarative macro wrapper which acts like a match statement, for transporting datetimes to gregorian date

 ### Common calendar transportation
 - AnyCalender -> Gregorian
 - AnyCalendar -> ...

> A declarative macro wrapper which acts like a match statement, for transporting datetimes to gregorian date

## DSL {gregorian | julian | ...} keywords
 > these are the keywords which when specified, are given access to its special scope

## param {expr} $n
 > This represents an ['EarthDateTime'] struct

## param {ty} $cal
 > This represents a certain calendar to convert to.

## data
 > Generic Calendar? $n calendar_type: AnyCalendar,
*/
#[macro_export]
macro_rules! transport_icu {
    (gregorian $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_gregorian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (julian $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_julian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (indian $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_indian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (iso $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_iso_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (chinese $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_chinese_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::chinese::Chinese::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (roc $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_roc_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (ethiopian $n: expr, $era: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_ethiopian_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (japanese $n: expr, $era: expr, $cal: ident ) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_japanese_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::Japanese::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (persian $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_persian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (coptic $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_coptic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (hebrew $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_hebrew_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::hebrew::Hebrew::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (buddhist $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_buddhist_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_civil $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_islamic_civil_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicCivil::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_observational $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_observational_islamic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicObservational::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_tabular $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_islamic_tabular_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicTabular::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_umm_al_qura $n: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_ummalqura_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (japanese_extended $n: expr, $era: expr, $cal: ident) => {
        RustSolarCalendar.earth_maker(
            icu_calendar::DateTime::try_new_japanese_extended_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::JapaneseExtended::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
}

#[derive(Debug, Default, Clone, Copy)]
/// This is the calendar system I prefer to use over since is a ultra abstraction to say.
pub struct RustSolarCalendar;

impl RustSolarCalendar {
    /// This utility function acts as a buffer to a calendar type.
    pub fn get_cc<T: AsCalendar>(self, d: T) -> T {
        d
    }

    /// ## Constructs an earth datetime from calendar type
    ///
    /// > The Generic is used to automatically assign the proper date mechanisms to the generic calendar type given.
    ///
    /// * TLDR: This is a mega wrapper for calendar creation.
    pub fn earth_maker<T: AsCalendar>(self, date: icu_calendar::DateTime<T>) -> EarthDateTime {
        EarthDateTime {
            date: EarthDate {
                year: date.year().unwrap().number,
                month: date.month().unwrap().ordinal,
                day: date.day_of_month().unwrap().0,
            },
            codes: EarthCodes {
                year: date.year().unwrap().era.0.to_string(),
                month: date.month().unwrap().code.0.to_string(),
                day: format!("{:?}", date.iso_weekday().unwrap()),
            },
            time: EarthTime {
                hour: date.hour().unwrap().number(),
                minute: date.minute().unwrap().number(),
                second: date.second().unwrap().number(),
            },
        }
    }

    /// # Converts an earth datetime to a specific calendar
    ///
    /// * param {&self}
    ///  > Allows daisy param chaining
    ///
    /// * param {EarthDateTime} input
    ///  > Represents an earthdate
    ///
    /// * param {&'static str} calendar_type
    ///  > Represents the type of calendar you want
    ///
    /// * returns {EarthDateTime}
    ///  > Returns an earth that which can be displayed seperately
    ///
    pub fn construct_calendar(
        self,
        input: EarthDate,
        // calendar_type: &'static str,
        calendar_type: String,
    ) -> EarthDateTime {
        let date: NaiveDate = NaiveDate::from_ymd_opt(input.year, input.month, input.day).unwrap();

        let date = EarthDateTime {
            date: EarthDate {
                year: date.year(),
                month: date.month(),
                day: date.day(),
            },
            ..Default::default()
        };

        // https://tc39.es/proposal-intl-era-monthcode/
        match calendar_type.as_str() {
            "gregorian" => get_icu! {gregorian date},
            "julian" => get_icu!(julian date),
            "indian" => get_icu!(indian date),
            "iso" => get_icu!(iso date),
            /* China  */
            "chinese" => get_icu!(chinese date),
            "roc" => get_icu!(roc date),
            /* Ethiopian Eras */
            "ethiopian_alem" => get_icu!(ethiopian date, EthiopianEraStyle::AmeteAlem),
            "ethiopian_mihret" => get_icu!(ethiopian date, EthiopianEraStyle::AmeteMihret),
            /* The 5 common japanese eras */
            /* 40+ japanese eras: https://en.wikipedia.org/wiki/Japanese_era_name#:~:text=For%20example%2C%20S55%20means%20Sh%C5%8Dwa,era%20(%E5%B9%B3%E6%88%9031%E5%B9%B4). */
            "japanese_heisei" => get_icu!(japanese date, Era::from_str("heisei").unwrap()),
            "japanese_reiwa" => get_icu!(japanese date, Era::from_str("reiwa").unwrap()),
            "japanese_meiji" => get_icu!(japanese date, Era::from_str("meiji").unwrap()),
            "japanese_taisho" => get_icu!(japanese date, Era::from_str("taisho").unwrap()),
            "japanese_showa" => get_icu!(japanese date, Era::from_str("showa").unwrap()),
            "persian" => get_icu!(persian date),
            "coptic" => get_icu!(coptic date),
            "hewbrew" => get_icu!(hebrew date),
            "buddhist" => get_icu!(buddhist date),
            /* Islamic  */
            "islamic_civil" => get_icu!(islamic_civil date),
            "islamic_observational" => get_icu!(islamic_observational date),
            "islamic_tabular" => get_icu!(islamic_tabular date),
            "islamic_umm_al_qura" => get_icu!(islamic_umm_al_qura date),
            _ => EarthDateTime::default(),
        }
    }

    /// # Converts a constructed calendar to another calendar
    pub fn to_calendar<T: AsCalendar>(
        self,
        input: EarthDate,
        calendar_type: String,
        conversion_type: T,
    ) -> EarthDateTime {
        let date: NaiveDate = NaiveDate::from_ymd_opt(input.year, input.month, input.day).unwrap();

        let date = EarthDateTime {
            date: EarthDate {
                year: date.year(),
                month: date.month(),
                day: date.day(),
            },
            ..Default::default()
        };

        match calendar_type.as_str() {
            "gregorian" => transport_icu! {gregorian date, conversion_type},
            "julian" => transport_icu!(julian date, conversion_type),
            "indian" => transport_icu!(indian date, conversion_type),
            "iso" => transport_icu!(iso date, conversion_type),
            /* China  */
            "chinese" => transport_icu!(chinese date, conversion_type),
            "roc" => transport_icu!(roc date, conversion_type),
            /* Ethiopian Eras */
            "ethiopian_alem" => {
                transport_icu!(ethiopian date, EthiopianEraStyle::AmeteAlem, conversion_type)
            }
            "ethiopian_mihret" => {
                transport_icu!(ethiopian date, EthiopianEraStyle::AmeteMihret, conversion_type)
            }
            /* The 5 common japanese eras */
            "japanese_heisei" => {
                transport_icu!(japanese date, Era::from_str("heisei").unwrap(), conversion_type)
            }
            "japanese_reiwa" => {
                transport_icu!(japanese date, Era::from_str("reiwa").unwrap(), conversion_type)
            }
            "japanese_meiji" => {
                transport_icu!(japanese date, Era::from_str("meiji").unwrap(), conversion_type)
            }
            "japanese_taisho" => {
                transport_icu!(japanese date, Era::from_str("taisho").unwrap(), conversion_type)
            }
            "japanese_showa" => {
                transport_icu!(japanese date, Era::from_str("showa").unwrap(), conversion_type)
            }
            "persian" => transport_icu!(persian date, conversion_type),
            "coptic" => transport_icu!(coptic date, conversion_type),
            "hewbrew" => transport_icu!(hebrew date, conversion_type),
            "buddhist" => transport_icu!(buddhist date, conversion_type),
            /* Islamic  */
            "islamic_civil" => transport_icu!(islamic_civil date, conversion_type),
            "islamic_observational" => transport_icu!(islamic_observational date, conversion_type),
            "islamic_tabular" => transport_icu!(islamic_tabular date, conversion_type),
            "islamic_umm_al_qura" => transport_icu!(islamic_umm_al_qura date, conversion_type),
            _ => EarthDateTime::default(),
        }
    }
}
