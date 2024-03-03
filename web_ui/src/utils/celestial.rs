use chrono::Local;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::mars::Mars;
use rust_solar::planets::mars::Martian;
use strum::EnumProperty;

use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;

/// Updates the timezone in localstorage depending on body (planet..) and battery (Arabia Time...)
pub fn set_celestial(condition: bool, body: String, battery: RwSignal<String>) {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    if true {
        state.1.set(CosmicTimeZoneState {
            name: body,
            timezone: battery.get(),
        });
    }
}

/// This function matches the response and returns an src link based off of celestial object you choose in your local storage
pub fn direct_tzname_image() -> String {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");


    match state.0.get().name.as_ref() {
        "Luna" => "https://science.nasa.gov/_ipx/animated_true&w_1280&f_webp/https://images-assets.nasa.gov/image/as11-40-5875/as11-40-5875~large.jpg%3Fw=1856%26h=1920%26fit=clip%26crop=faces%252Cfocalpoint",
        "Martian" => "https://mars.nasa.gov/system/resources/detail_files/21360_mars-twisted-tail.jpg",
        "Earth" => "https://explorer1.jpl.nasa.gov/assets/images/galleries/1972_BlueMarble_115334main_image_feature_329_ys_full.jpg",
        "Saturn" => "https://science.nasa.gov/_ipx/animated_true&w_1280&f_webp/https://images-assets.nasa.gov/image/PIA03152/PIA03152~orig.jpg%3Fw=849%26h=900%26fit=clip%26crop=faces%252Cfocalpoint",
        "Jupiter" => "https://www.nasa.gov/wp-content/uploads/2023/03/52303461859_0db4d9b739_o.png",
        "Neptune" => "https://www.nasa.gov/wp-content/uploads/2023/03/pia01492-main.jpg",
        "Ganymede" => "https://photojournal.jpl.nasa.gov/jpegMod/PIA01666_modest.jpg",
        _ => "https://www.nasa.gov/wp-content/uploads/2024/02/53495965723-5c5c821f78-o.png?resize=1536,859"
    }.to_string()
}
