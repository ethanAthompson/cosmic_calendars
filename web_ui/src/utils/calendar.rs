use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::env::{LEFTCARD, RIGHTCARD};
use crate::stores::get_state;
use crate::stores::states::{CelestialCalendar, EarthCalendarState};
use crate::utils::select::*;
use rust_solar::planets::earth::EarthTimeZones;

fn earth_calendars() -> Vec<&'static str> {
    vec![
        "Gregorian",
        "Julian",
        "Indian",
        "Iso",
        "Chinese",
        "Republic of China (Roc)",
        "Ethiopian Alem",
        "Ethiopian Mihret",
        "Japanese Heisei",
        "Japanese Reiwa",
        "Japanese Meiji",
        "Japanese Taisho",
        "Japanese Showa",
        "Japanese Extended",
        "Persian",
        "Coptic",
        "Hebrew",
        "Buddhist",
        "Islamic Civil",
        "Islamic Observational",
        "Islamic Tabular",
        "Islamic Umm Al Qura",
    ]
}

fn rsar_calendars() -> Vec<&'static str> {
    vec![
        "Rsar",
        "Zsar"
    ]
}


#[component]
/// This component lists all of the supporting calendars.
pub fn SupportedCalendars() -> impl IntoView {
    let state = get_state::<EarthCalendarState>("preferred-calendar");

    // even if you delete the key, it'll always stay
    if state.0.get().preferred.is_empty() {
        // logging::log!("{:?}", state.0.get());

        state.1.set(EarthCalendarState {
            preferred: "Gregorian".to_string(),
        });
    }

    let save_preference = move |ev: MouseEvent| {
        ev.prevent_default();

        let href_self = ev
            .target()
            .expect("<save_preference>: HrefSelf [00]")
            .dyn_into::<HtmlAnchorElement>()
            .expect("<save_preference>: HrefSelf [01]")
            .id();

        state.1.set(EarthCalendarState {
            preferred: href_self,
        });
    };

    let dropdown_items = earth_calendars()
        .into_iter()
        .map(|v| {
            view! {
                <li>
                    <a
                        class="btn btn-outline-warning btn-lg dropdown-item"
                        id=format!("{}", v)
                        on:click=save_preference
                    >
                        {v}
                    </a>
                </li>
            }
        })
        .collect_view();
    // loop over supported calendars
    view! {
        <ul
            style="max-height:250px;"
            class=" text-primary dropdown-menu overflow-y-auto pe-auto user-select-auto"
        >
            {dropdown_items}
        </ul>
    }
}

#[component]
/// This component shows the earth calendar for each respective card
pub fn EarthCalendars(pos: &'static str) -> impl IntoView {
    view! {
        <Show
            when=move || pos == RIGHTCARD
            fallback=move || {
                view! {
                    <CelestialSelectionLeft tz_list=earth_calendars() pos=LEFTCARD body="Earth"/>
                }
            }
        >

            <CelestialSelectionRight tz_list=earth_calendars() pos=RIGHTCARD body="Earth"/>

        </Show>
    }
}

#[component]
/// This component shows the celestial calendar for each respective card
pub fn RsarCalendars(pos: &'static str) -> impl IntoView {
    view! {
        <Show
            when=move || pos == RIGHTCARD
            fallback=move || {
                view! { <RsarCalendarSelectionLeft tz_list=rsar_calendars() pos=LEFTCARD body=""/> }
            }
        >

            <RsarCalendarSelectionRight tz_list=rsar_calendars() pos=RIGHTCARD body=""/>

        </Show>
    }
}