use chrono::{Datelike, Timelike};

use crate::planets::earth::{EarthCodes, EarthDate, EarthDateTime, EarthTime};

/// The first Julian Date epoch
pub const JD2NOON: f64 = 2451545.0;

#[derive(Default, Debug, Copy, Clone)]
/// This structure contains julian date calculations
pub struct Julian;

impl Julian {
    /// J2000 = JD - ['JD2NOON']
    pub fn days_since_j2000(self, year: i32, month: i32, day: i32, offset: f64) -> f64 {
        let j2 = self.get_jd(year, month, day, offset) - JD2NOON;

        println!("{:?} Days since j2000", j2);

        j2
    }

    /// converts julian date to gregorian date
    pub fn jd2greg(self, jd: f64) {
        if let Ok(date_time) = julian_day_converter::julian_day_to_datetime(jd) {
            println!("The date time is {}", date_time.format("%Y-%m-%d %H:%M:%S"));
        }
    }

    /// converts julian date to gregorian date with a structure
    pub fn jd2greg_named(self, jd: f64) -> EarthDateTime {
        if let Ok(date_time) = julian_day_converter::julian_day_to_datetime(jd) {
            EarthDateTime {
                date: EarthDate {
                    year: date_time.year(),
                    month: date_time.month(),
                    day: date_time.day(),
                },
                codes: EarthCodes::default(),
                time: EarthTime {
                    hour: date_time.hour() as u8,
                    minute: date_time.minute() as u8,
                    second: date_time.second() as u8
                }
            }
        } else {
            EarthDateTime::default()
        }
    }

    ///  your offset is decimal hours in military time: ex; 20.5 is 20:05pm is 8:05pm
    pub fn get_jd(self, year: i32, month: i32, day: i32, offset: f64) -> f64 {
        let jd = (367 as f64 * year as f64
            - (7 * (year + (month + 9) / 12) / 4) as f64
            - (((3 * (year + (month - 9) / 7) / 100) + 1) / 4) as f64
            + (275 * month / 9) as f64
            + day as f64
            + 1721028.5 as f64
            + offset as f64 / 24.0) as f64;

        println!("Julian date: {:?}", jd);

        jd
    }
}
