use crate::stores::get_state;
use crate::stores::states::CelestialChosenNameState;
use crate::stores::states::CelestialLocale;
use crate::stores::states::CosmicTimeZoneState;
use chrono::Local;
use leptos::svg::filter;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::mars::Mars;
use rust_solar::planets::mars::Martian;
use std::borrow::BorrowMut;
use strum::EnumProperty;
use web_sys::HtmlLabelElement;
use web_sys::HtmlOptionElement;
use web_sys::HtmlSelectElement;
use web_sys::MouseEvent;
use chrono_tz::TZ_VARIANTS;
use crate::utils::select::CelestialSelectionRight;
use crate::utils::select::CelestialSelectionLeft;
use rust_solar::planets::earth::EarthTimeZones;
use crate::env::{LEFTCARD, RIGHTCARD};

#[component]
/// This is a component that notifies the tz state, that the user sets it to earth
pub fn LocalEarth() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <span
            class="navbar-brand"
            on:click=move |_| {
                state
                    .1
                    .set(CosmicTimeZoneState {
                        name: "Earth".to_string(),
                        timezone: "Local".to_string(),
                    });
            }
        >

            <label class="btn btn-lg container-fluid btn-body" for=format!("vbtn-radio{}", "earth")>
                Local
            </label>
        </span>
    }
}

#[component]
/// This is a component that notifies the tz state, that the user sets it to earth
pub fn ExtraLocaleEarth() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let timezones = create_rw_signal(Vec::new());
    let mut tz: Vec<String> = EarthTimeZones::all_timezones(timezones.get());

    timezones.set(tz);

    let get_id = move |ev: MouseEvent| {
        let id = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap()
            .id();

        state.1.set(CosmicTimeZoneState {
            name: "Earth".to_string(),
            timezone: id,
        });

        // logging::log!("{:?}", id);
    };

    let view_tz = timezones
        .get()
        .into_iter()
        .map(|zone| {
            view! {
                <li class="list-group-item" id=format!("{}", zone)>
                    <span class="badge rounded-pill text-bg-success">Earth</span>

                    <span class="navbar-brand ">
                        <label
                            class="btn btn-lg lead container-fluid btn-body"
                            for=format!("vbtn-radio{}", zone)
                            id=format!("{}", zone)
                            on:click=get_id
                        >
                            {&zone}
                        </label>
                    </span>
                </li>
            }
        })
        .collect_view();

    view! { {view_tz} }
}
