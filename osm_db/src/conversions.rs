/// This function converts astronomical units to kilometers
///
/// > ![Au](https://latex.codecogs.com/svg.image?1AU=1.495978707*10^{11})
/// 
/// > Numbers are according to <https://en.wikipedia.org/wiki/Astronomical_unit>
///
/// ### Example Of Venus orbiting average distance (0.723) AU
///
/// ```rust
///  use rust_solar::conversions::au2km;
///
///  // The AU of venus's' semi-major axis is 0.723
///  assert_eq!(108159260516.09999, au2km(0.723));
///
/// ```
///
pub fn au2km(value: f64) -> f64 {
    value * (1.495_978_707 * 100_000_000_000.0)
}

/// This function is a wrapper over calculating the radians in a circle
///
pub fn radians_in_circle() -> f64 {
    std::f64::consts::PI * 2.0
}

/// This function is calculates millitary time to standard time
pub fn military2standard(hour: i32) -> (u32, &'static str) {
    match hour {
        0 => (12, "AM"),
        1 => (1, "AM"),
        2 => (2, "AM"),
        3 => (3, "AM"),
        4 => (4, "AM"),
        5 => (5, "AM"),
        6 => (6, "AM"),
        7 => (7, "AM"),
        8 => (8, "AM"),
        9 => (9, "AM"),
        10 => (10, "AM"),    
        11 => (11, "AM"),
        12 => (12, "PM"),
        13 => (1, "PM"),
        14 => (2, "PM"),
        15 => (3, "PM"),
        16 => (4, "PM"),
        17 => (5, "PM"),
        18 => (6, "PM"),
        19 => (7, "PM"),
        20 => (8, "PM"),
        21 => (9, "PM"),
        22 => (10, "PM"),
        23 => (11, "PM"),
        _ => (0, "Unknown")
    }
}