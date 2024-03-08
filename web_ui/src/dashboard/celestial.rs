pub mod zones;

use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::stores::get_state;
use crate::stores::states::{CosmicTimeZoneState, UserNameState};
use crate::stores::states::EarthCalendarState;
use crate::utils::calendar::SupportedCalendars;
use crate::utils::celestial::direct_tzname_image;
use crate::time::martian::LocalMars;
use crate::time::earth::ExtraLocaleEarth;
use crate::utils::filter::filter_list;
use crate::dashboard::celestial::zones::Timezones;

#[component]
/// This component is the list of references that you see.
pub fn Page() -> impl IntoView {
    let state = get_state::<UserNameState>("home-user-name");
    let state_ref = create_rw_signal(state.0.get().value);

    view! {
        <div class="fadeIn bg-transparent container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <div class="text-center">
                <p class="lead py-2">
                    "Your current time for the selected location is shown above."
                </p>
                <h1>List provided by RSAR and IANA Databases</h1>
                <CelestialSearcher/>
                <ul class="list-group list-group-flush p-4 border border-4 rounded-bottom">
                    <Timezones/>
                </ul>
            </div>
        </div>
    }
}

#[component]
/// This component allows the user to search for Celestial timezones
pub fn CelestialSearcher() -> impl IntoView {
    let battery = create_rw_signal("".to_string());
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    
    let filter_search = move |ev| {
        battery.set(event_target_value(&ev));
        filter_list("celestial-timezones-main", battery.get());
    };

    view! {
        <div class="me-auto input-group input-group-lg flex-nowrap border border-2 rounded-top">
            <i
                class="input-group-text bi bi-search py-2 rounded-0"
                id="SearchModalLabel-wrapping"
            ></i>
            <input
                type="text"
                class="focus-ring focus-ring-success form-control py-2 rounded-0"
                placeholder="Search IANA + RSAR Databases"
                aria-label="Search IANA + RSAR Databases"
                aria-describedby="SearchModalLabel-wrapping"
                on:input=filter_search
                prop:value=battery
            />
        </div>
    }
}

