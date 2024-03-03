use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use chrono::Local;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::earth::EarthTimeZones;
use rust_solar::planets::mars::Mars;
use rust_solar::planets::mars::Martian;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use strum::EnumProperty;
use web_sys::HtmlLabelElement;
use web_sys::HtmlLiElement;
use web_sys::HtmlSpanElement;
use web_sys::HtmlUListElement;
use web_sys::MouseEvent;

#[component]
/// This component is the settings page
pub fn Page() -> impl IntoView {
   
    view! {
        <div class="container-fluid ">
            <h1>Live dates must be activated via settings (performance reasons... WIP)</h1>
            <h1>Calendar can be changed within settings option... (WIP)</h1>
        </div>
    }
}
