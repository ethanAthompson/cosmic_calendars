use crate::time::martian::set_mars_tz;
use chrono::Datelike;
use leptos::RwSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use osm_db::asteroids;
use osm_db::asteroids::ceres::Ceres;
use osm_db::asteroids::eros::Eros;
use osm_db::asteroids::hebe::Hebe;
use osm_db::asteroids::vesta::Vesta;
use osm_db::kepler::Body;
use osm_db::moons::europa::Europa;
use osm_db::moons::ganymede::Ganymede;
use osm_db::moons::io::Io;
use osm_db::moons::luna::Luna;
use osm_db::moons::titan::Titan;
use osm_db::orbit::Perihelion;
use osm_db::orbit::Type;
use osm_db::planets::earth::Earth;
use osm_db::planets::jupiter::Jupiter;
use osm_db::planets::mars::Mars;
use osm_db::planets::mercury::Mercury;
use osm_db::planets::neptune::Neptune;
use osm_db::planets::pluto::Pluto;
use osm_db::planets::saturn::Saturn;
use osm_db::planets::uranus::Uranus;
use osm_db::planets::venus::Venus;

pub fn data_abstraction(
    name: &'static str,
    orbital_eccentricity: RwSignal<f64>,
    orbital_period: RwSignal<f64>,
    rotational_period: RwSignal<f64>,
    perihelion: RwSignal<Perihelion>,
    semimajor: RwSignal<f64>,
    semiminor: RwSignal<f64>,
    mean_motion: RwSignal<f64>,
    orbit_type: RwSignal<String>,
    map_src: RwSignal<String>,

) -> impl IntoView {
    match name {
        "earth" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Earth.orbital_eccentricity());
            orbital_period.set(Earth.orbital_period());
            rotational_period.set(Earth.rotational_period());
            perihelion.set(Earth.perihelion());
            semimajor.set(Earth.semimajor());
            semiminor.set(Earth.semiminor());
            mean_motion.set(Earth.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "mars" => {
            let sol = set_mars_tz().date.day;
            orbital_eccentricity.set(Mars.orbital_eccentricity());
            orbital_period.set(Mars.orbital_period());
            rotational_period.set(Mars.rotational_period());
            perihelion.set(Mars.perihelion());
            semimajor.set(Mars.semimajor());
            semiminor.set(Mars.semiminor());
            mean_motion.set(Mars.mean_motion(sol));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "mercury" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Mercury.orbital_eccentricity());
            orbital_period.set(Mercury.orbital_period());
            rotational_period.set(Mercury.rotational_period());
            perihelion.set(Mercury.perihelion());
            semimajor.set(Mercury.semimajor());
            semiminor.set(Mercury.semiminor());
            mean_motion.set(Mercury.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "neptune" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Neptune.orbital_eccentricity());
            orbital_period.set(Neptune.orbital_period());
            rotational_period.set(Neptune.rotational_period());
            perihelion.set(Neptune.perihelion());
            semimajor.set(Neptune.semimajor());
            semiminor.set(Neptune.semiminor());
            mean_motion.set(Neptune.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));

        }
        "jupiter" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Jupiter.orbital_eccentricity());
            orbital_period.set(Jupiter.orbital_period());
            rotational_period.set(Jupiter.rotational_period());
            perihelion.set(Jupiter.perihelion());
            semimajor.set(Jupiter.semimajor());
            semiminor.set(Jupiter.semiminor());
            mean_motion.set(Jupiter.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));

        }      
        "saturn" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Saturn.orbital_eccentricity());
            orbital_period.set(Saturn.orbital_period());
            rotational_period.set(Saturn.rotational_period());
            perihelion.set(Saturn.perihelion());
            semimajor.set(Saturn.semimajor());
            semiminor.set(Saturn.semiminor());
            mean_motion.set(Saturn.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));

        }
        "pluto" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Pluto.orbital_eccentricity());
            orbital_period.set(Pluto.orbital_period());
            rotational_period.set(Pluto.rotational_period());
            perihelion.set(Pluto.perihelion());
            semimajor.set(Pluto.semimajor());
            semiminor.set(Pluto.semiminor());
            mean_motion.set(Pluto.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));

        }
        "venus" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Venus.orbital_eccentricity());
            orbital_period.set(Venus.orbital_period());
            rotational_period.set(Venus.rotational_period());
            perihelion.set(Venus.perihelion());
            semimajor.set(Venus.semimajor());
            semiminor.set(Venus.semiminor());
            mean_motion.set(Venus.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "uranus" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Uranus.orbital_eccentricity());
            orbital_period.set(Uranus.orbital_period());
            rotational_period.set(Uranus.rotational_period());
            perihelion.set(Uranus.perihelion());
            semimajor.set(Uranus.semimajor());
            semiminor.set(Uranus.semiminor());
            mean_motion.set(Uranus.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "luna" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Luna.orbital_eccentricity());
            orbital_period.set(Luna.orbital_period());
            rotational_period.set(Luna.rotational_period());
            perihelion.set(Luna.perihelion());
            semimajor.set(Luna.semimajor());
            semiminor.set(Luna.semiminor());
            mean_motion.set(Luna.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "europa" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Europa.orbital_eccentricity());
            orbital_period.set(Europa.orbital_period());
            rotational_period.set(Europa.rotational_period());
            perihelion.set(Europa.perihelion());
            semimajor.set(Europa.semimajor());
            semiminor.set(Europa.semiminor());
            mean_motion.set(Europa.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "titan" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Titan.orbital_eccentricity());
            orbital_period.set(Titan.orbital_period());
            rotational_period.set(Titan.rotational_period());
            perihelion.set(Titan.perihelion());
            semimajor.set(Titan.semimajor());
            semiminor.set(Titan.semiminor());
            mean_motion.set(Titan.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "io" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Io.orbital_eccentricity());
            orbital_period.set(Io.orbital_period());
            rotational_period.set(Io.rotational_period());
            perihelion.set(Io.perihelion());
            semimajor.set(Io.semimajor());
            semiminor.set(Io.semiminor());
            mean_motion.set(Io.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "ganymede" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Ganymede.orbital_eccentricity());
            orbital_period.set(Ganymede.orbital_period());
            rotational_period.set(Ganymede.rotational_period());
            perihelion.set(Ganymede.perihelion());
            semimajor.set(Ganymede.semimajor());
            semiminor.set(Ganymede.semiminor());
            mean_motion.set(Ganymede.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "ceres" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Ceres.orbital_eccentricity());
            orbital_period.set(Ceres.orbital_period());
            rotational_period.set(Ceres.rotational_period());
            perihelion.set(Ceres.perihelion());
            semimajor.set(Ceres.semimajor());
            semiminor.set(Ceres.semiminor());
            mean_motion.set(Ceres.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "4-vesta" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Vesta.orbital_eccentricity());
            orbital_period.set(Vesta.orbital_period());
            rotational_period.set(Vesta.rotational_period());
            perihelion.set(Vesta.perihelion());
            semimajor.set(Vesta.semimajor());
            semiminor.set(Vesta.semiminor());
            mean_motion.set(Vesta.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "433-eros" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Eros.orbital_eccentricity());
            orbital_period.set(Eros.orbital_period());
            rotational_period.set(Eros.rotational_period());
            perihelion.set(Eros.perihelion());
            semimajor.set(Eros.semimajor());
            semiminor.set(Eros.semiminor());
            mean_motion.set(Eros.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        "6-hebe" => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(Hebe.orbital_eccentricity());
            orbital_period.set(Hebe.orbital_period());
            rotational_period.set(Hebe.rotational_period());
            perihelion.set(Hebe.perihelion());
            semimajor.set(Hebe.semimajor());
            semiminor.set(Hebe.semiminor());
            mean_motion.set(Hebe.mean_motion(now));
            orbit_type.set(format!(
                "{:?}",
                Type::shape(&Type::Unknown, orbital_eccentricity.get())
            ));
            map_src.set(format!("{}", ""));
        }
        _ => {
            let now = chrono::Utc::now().day() as f64;
            orbital_eccentricity.set(-1.0);
            orbital_period.set(-1.0);
            rotational_period.set(-1.0);
            semimajor.set(-1.0);
            semiminor.set(-1.0);
            mean_motion.set(-1.0);
            orbit_type.set(format!("N/A",));
            map_src.set(format!("{}", "N/A"));
        }
    }
}
