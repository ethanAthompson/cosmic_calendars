#[cfg(test)]
mod tests {
    use icu_calendar::{chinese::Chinese, julian::Julian, Gregorian};
    use rust_solar::{planets::earth::{EarthDate, EarthDateTime, EarthTimeZones, RustSolarCalendar}, set_datetimes};

    #[test]
    fn julian2chinese_jd_epoch() {
        let input = EarthDate::julian_epoch();
        let calendar_type = "julian";
        let conversion_type = Chinese::default();
        let cal = RustSolarCalendar.to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn gregorian2chinese_jd_epoch() {
        let input = EarthDate::julian_epoch();
        let calendar_type = "gregorian";
        let conversion_type = Chinese::default();
        let cal = RustSolarCalendar.to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian2chinese_now() {
        let input = EarthDate::now();
        let calendar_type = "julian";
        let conversion_type = Chinese::default();
        let cal = RustSolarCalendar.to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian_now() {
        let input = EarthDate::now();
        let calendar_type = "julian";
        let cal = RustSolarCalendar.construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn gregorian_now() {
        let input = EarthDate::now();
        let calendar_type: &str = "gregorian";
        let cal = RustSolarCalendar.construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn chinese_now() {
        let input = EarthDate::now();
        let calendar_type = "chinese";
        let cal = RustSolarCalendar.construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian2chinese2gregorian() {
        let input = EarthDate::now();
        let cal = RustSolarCalendar.construct_calendar(input, "julian");
        let cal_1 = RustSolarCalendar.to_calendar(cal.date, "julian", Chinese::default());
        let cal_2 = RustSolarCalendar.to_calendar(cal_1.date, "chinese", Gregorian::default());
        println!("{:?}", cal_2);
    }

    #[test]
    fn gregorian2chinese2julian() {
        let input = EarthDate::now();
        let cal = RustSolarCalendar.construct_calendar(input, "gregorian");
        let cal_1 = RustSolarCalendar.to_calendar(cal.date, "gregorian", Chinese::default());
        let cal_2 = RustSolarCalendar.to_calendar(cal_1.date, "chinese", Julian::default());
        println!("{:?}", cal_2);
    }

    #[test]
    fn chinese2julian2gregorian() {
        let input = EarthDate::now();
        let cal = RustSolarCalendar.construct_calendar(input, "chinese");
        let cal_1 = RustSolarCalendar.to_calendar(cal.date, "chinese", Julian::default());
        let cal_2 = RustSolarCalendar.to_calendar(cal_1.date, "julian", Gregorian::default());
        println!("{:?}", cal_2);
    }


    #[test]
    fn does_all_timezones() {
        let list = EarthTimeZones::all_timezones(Vec::new());
        println!("{:?}", list);
    }

    #[test]
    fn does_set_datetime() {
        let dt = EarthDateTime::set_datetime( "America/New_York".to_string());
        println!("DT : {:?}, {:?}", dt[0].0, dt[0].1);
    }

    #[test]
    fn does_set_datetimes() {
        set_datetimes! {
            "Japan".to_string(), "EST".to_string(), "APPLE".to_string()
        };
    }
}
