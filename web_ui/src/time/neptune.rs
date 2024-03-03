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
use web_sys::Event;
use web_sys::MouseEvent;

use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use crate::env::{LEFTCARD, RIGHTCARD};
use crate::utils::select::*;

#[component]
pub fn MercuryTimezonesLeft() -> impl IntoView {

    let ctz = vec![
        "Neptunian Time",
        "Suporcalien Time",
        "Arcadian Time",
        "Chinain Time",
        "Aquaphorian Time",

    ];
    view! { <CelestialSelectionLeft tz_list=ctz pos="Left" body="Mercury"/> }
}

#[component]
pub fn MercuryTimezonesRight() -> impl IntoView {
    let ctz = vec![
        "Neptunian Time",
        "Suporcalien Time",
        "Arcadian Time",
        "Chinain Time",
        "Aquaphorian Time",

    ];

    view! { <CelestialSelectionRight tz_list=ctz pos="Right" body="Mercury"/> }
}