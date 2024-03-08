use crate::time::martian::set_mars_tz;
use chrono::Datelike;
use leptos::RwSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use osm_db::kepler::Body;
use osm_db::orbit::Perihelion;
use osm_db::orbit::Type;
use osm_db::planets::earth::Earth;
use osm_db::planets::mars::Mars;

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
    dark_topology_map: RwSignal<String>,
    light_topology_map: RwSignal<String>,
) -> impl IntoView {
    match name {
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
            dark_topology_map.set("/public/images/solar/dark-mars-tz.jpeg".to_string());
            light_topology_map.set("/public/images/solar/light-mars-tz.png".to_string());
        }

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
            dark_topology_map.set("/public/images/solar/earth_ls.jpg".to_string());
            light_topology_map.set("/public/images/solar/earth_ls.jpg".to_string());
        }
        _ => {
            //  todo!()
        }
    }
}
