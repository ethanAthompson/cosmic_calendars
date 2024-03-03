/// This module contains earth calculations
pub mod earth;
/// This module contains jupiter calculations
pub mod jupiter;
/// This module contains mars calculations
pub mod mars;
/// This module contains mercury calculations
pub mod mercury;
/// This module contains neptune calculations
pub mod neptune;
/// This module contains pluto calculations
pub mod pluto;
/// This module contains saturn calculations
pub mod saturn;
/// This module contains uranus calculations
pub mod uranus;
/// This module contains venus calculations
pub mod venus;

/// This is the rotational period for earth in seconds
pub const EARTH_ROTATIONAL_PERIOD: f64 = 86400.0;

/// This is the orbital period for earth in days
pub const EARTH_ORBITAL_PERIOD: f64 = 365.25;
