use std::f64::consts::PI;

use crate::{
    kepler::{Date, Eras},
    orbit::Season,
    planets::EARTH_ROTATIONAL_PERIOD,
};

/// i
pub const RIC: f64 = PI * 2.0;

#[derive(Default, Debug, Copy, Clone)]
/// o
pub struct Example;

// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
impl Example {
    /// o
    pub fn semimajor(&self) -> f64 {
        1.00000
    }

    /// o
    pub fn semiminor(&self) -> f64 {
        0.99986
    }
    /// o
    pub fn orbital_eccentricity(&self) -> f64 {
        0.0934
    }

    /// o
    pub fn day_in_seconds(&self, is_solar: bool) -> f64 {
        match is_solar {
            true => 88_775.245,
            false => 88_642.663,
        }
    }

    /// o
    pub fn year_in_days(&self, is_leap: bool) -> f64 {
        match is_leap {
            true => 669.6,
            false => 668.6,
        }
    }

    /// o
    pub fn is_leapyear(&self) -> bool {
        false
    }

    /// 8th month of mars is the date of the perihelion
    pub fn perihelion_date(&self) -> f64 {
        let month_start = 468.5;
        let month_end = 514.6;
        let ls_start = 240.0;
        let ls_end = 270.0;
        let perihelion_ls = 251.0;

        let avg_days = month_end - month_start;
        let avg_ls = ls_end - ls_start;
        let until_peri = perihelion_ls - ls_start;
        let peri_day = avg_days / avg_ls;

        let date = (peri_day * until_peri) + month_start;

        // println!("{date}");

        date
    }

    /// 2 * PI * (1 * PeriLs / 360)
    pub fn perihelion_time(&self) -> f64 {
        2.0 * PI * (1.0 - 251.0 / 360.0)
    }

    /// o
    pub fn epoch(&self) -> f64 {
        2.442765667e6
        // 2442765.66667
    }

    /// o
    pub fn average_ls(&self) -> f64 {
        30.0
    }

    /// o
    pub fn perihelian_elapse(&mut self, day: f64) -> f64 {
        (day - self.perihelion_date()) / self.year_in_days(false)
    }

    /// o
    pub fn mean_motion(&mut self, day: f64) -> f64 {
        2.0 * PI * (self.perihelian_elapse(day) - self.perihelian_elapse(day).round())
    }

    /// o
    pub fn mean_anomaly(&mut self, day: f64) -> f64 {
        self.mean_motion(day).abs()
    }

    /// o
    pub fn true_anomaly(&mut self, day: f64) -> f64 {
        let mut zdx = 10.0;

        let xref = self.mean_anomaly(day);

        let mut zx0 = xref + self.orbital_eccentricity() * xref.sin();
        // println!("Zx0 +: {zx0}");

        while zdx > 1.0e-7 {
            // En = - ((En - e * En.sin() - M(t)) / 1 - e * En.cos() )
            zdx = -(zx0 - self.orbital_eccentricity() * zx0.sin() - xref)
                / (1.0 - self.orbital_eccentricity() * zx0.cos());

            // En = En + En+1
            zx0 = zx0 + zdx;
        }

        if self.mean_motion(day) < 0.0 {
            zx0 = -zx0;
        };

        //println!("Zx0 -: {zx0}");

        let mean_motion =
            ((1.0 + self.orbital_eccentricity()) / (1.0 - self.orbital_eccentricity())).sqrt();

        // Eccentric Anomaly
        // v = 2 * ( ((1 + e) / (1 - e)).sqrt() * (E / 2).tan() ).atan()
        let zteta = 2.0 * (mean_motion * (zx0 / 2.0).tan()).atan();

        // println!("Zteta: {zteta}");

        return zteta;
    }

    /// o
    pub fn compute_ls(&mut self, day: f64) -> f64 {
        let theta = self.true_anomaly(day);

        // Mean anomaly; position of body in orbit since the perihelian time
        let mut ls = theta - self.perihelion_time();

        // if its less than the perihelion time
        if ls < 0.0 {
            ls += RIC;
        }

        // if its more than the perihelion time
        if ls > RIC {
            ls -= RIC;
        }

        // println!("Degrees {}", ls.to_degrees());
        return ls.to_degrees();
    }

    /// o
    pub fn to_date(&mut self, jd: f64) -> Date {
        let mut tmp_year = 12.0;
        let mut tmp_day = (jd - self.epoch()) * EARTH_ROTATIONAL_PERIOD / self.day_in_seconds(true);

        // After Discovery
        while tmp_day >= self.year_in_days(false) {
            tmp_day -= self.year_in_days(false);
            tmp_year += 1.0;
        }

        // Before Discovery
        while tmp_day < 0.0 {
            tmp_day += self.year_in_days(false);
            tmp_year -= 1.0;
        }

        let ls = self.compute_ls(tmp_day);
        let year = tmp_year;
        let month = 1.0 + (ls / self.average_ls()).floor();
        let day = 1.0 + tmp_day.floor();
        let season = Season::default().from(ls as u32);

        // callibrates era according to year's coefficient type (- or +)
        let era = match year as i32 > 0 {
            true => Eras::AD,
            false => Eras::BD,
        };

        // AD vs BD
        return Date {
            era,
            year,
            month,
            day,
            ls,
            season,
        };
    }
}
