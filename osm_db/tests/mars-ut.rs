#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use rust_solar::{
        conversions::military2standard,
        julian::Julian,
        kepler::{Body, TimeZone},
        planets::mars::{Mars, Martian},
    };

    #[test]
    pub fn mars_to_date() {
        let now = chrono::Utc::now();
        let offset = Martian::MTCn5.offset();
        let jd = Julian.get_jd(now.year(), now.month() as i32, now.day() as i32, offset);
        let date = Mars.to_date(jd);

        Julian.jd2greg(jd);
        println!("{:?}", offset);
        println!("The date is {:?}", date);
    }

    #[test]
    pub fn mars_to_time() {
        let time = Martian::MTCp5.now();
        println!("Time now: {:?}", time);
    }

    #[test]
    pub fn mars_datetime_military() {
        let now = Mars.now(Martian::MTCp3);

        let date = format!(
            "{:?}/{:?}/{:?}, {:?}°",
            now.date.year, now.date.month, now.date.day, now.date.ls
        );

        let military = format!(
            "{:?}:{:?}:{:?}, {:?}",
            now.time.hour, now.time.minute, now.time.second, now.time.name
        );

        println!("{date} {military} {:?}", now.time.name);
    }

    #[test]
    pub fn mars_datetime_standard() {
        let now = Mars.now(Martian::MTCp3);
        let hour = military2standard(now.time.hour);

        let date = format!(
            "{:?}/{:?}/{:?}, {:?}°",
            now.date.year, now.date.month, now.date.day, now.date.ls
        );

        let standard = format!(
            "{:?}:{:?}:{:?} {:?}",
            hour.0, now.time.minute, now.time.second, hour.1
        );

        println!("{date} {standard} {:?}", now.time.name);
    }

    #[test]
    pub fn martian_tz_1() {
        let value = Martian::MTCn3.julian_date_universal_time();

        println!("JD UT: {:?}", value);
    }

    #[test]
    pub fn martian_tz_2() {
        let value = Martian::MTCn3.julian_date_terrestial_time();

        println!("JD TT: {:?}", value);
    }

    #[test]
    pub fn martian_tz_3() {
        let value = Martian::MTCn3.julian_date_2000_time();

        println!("JD 2000 T: {:?}", value);
    }

    #[test]
    pub fn martian_tz_4() {
        let value = Martian::MTCn3.body_host_ratio();

        println!("Body Host: {:?}", value);
    }

    #[test]
    pub fn martian_tz_5() {
        let value = Martian::MTCn3.day_date();

        println!("Day Date: {:?}", value);
    }

    #[test]
    pub fn martian_tz_6() {
        let value = Martian::MTCn3.coordinated_time();

        println!("Coordinated Time: {:?}", value);
    }

    #[test]
    pub fn martian_tz_7() {
        let value = Martian::MTCn4.now();

        println!("Coordinated Time: {:?}", value);
    }

    #[test]
    pub fn martian_tz_8() {
        let value = Martian::MTCn1.offset();

        println!("Coordinated Time: {:?}", value);
    }
}
