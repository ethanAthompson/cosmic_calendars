/// Contains time for earth area
pub mod earth;

// Contains time for martian area
pub mod martian;

// Contains time for mercury area
pub mod mercury;

// Contains time for neptune area
pub mod neptune;

use chrono::Local;
use chrono::Utc;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::mars::Mars;
use rust_solar::planets::mars::Martian;
use rust_solar::set_datetimes;
use strum::EnumProperty;
use rust_solar::planets::earth::EarthDateTime;

use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use crate::time::martian::set_mars_tz;

#[component]
/// This component represents the standard way of coordinating time given the users' choice
pub fn LocalTimer() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    // even if you delete the key, it'll always stay
    if state.0.get_untracked().name.is_empty() {
        // logging::log!("{:?}", state.0.get());

        state.1.set(CosmicTimeZoneState {
            name: "Earth".to_string(),
            timezone: "Local".to_string(),
        });
    }

    let datetime = chrono::Local::now().format("%v %r %Z").to_string();
    let local_str = create_rw_signal(format!("{}", ""));
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            if state.0.get().name == "Earth" && state.0.get().timezone == "Local" {
                let datetime = Local::now().format("%v %r %Z").to_string();
                local_str.update(|v| *v = format!("{}", datetime));
            } else {
                if state.0.get().name == "Earth" {
                    let datetime = set_datetimes! {state.0.get().timezone};
                    local_str.update(|v| *v = format!("{}", datetime[0].1));
                }
            }

            if state.0.get().name == "Martian" {
                let now = set_mars_tz();
                let hour = rust_solar::conversions::military2standard(now.time.hour);

                let date = format!(
                    "{}/{}/{}, {:.2}° ",
                    now.date.year, now.date.month, now.date.day, now.date.ls
                );

                let standard = format!(
                    "{}:{}:{} {} {} ",
                    hour.0, now.time.minute, now.time.second, hour.1, now.time.name
                );

                local_str.update(|v| *v = format!("{}{}", date, standard));
            }
        },
        1000,
    );

    // logging::log!("{:?}", local_str.get_untracked());

    view! {
        <Show
            when=move || local_str.get().is_empty() == false
            fallback=move || {
                view! {
                    <span class="hstack gap-2 d-flex justify-content-center">
                        <span class="spinner-grow spinner-grow-sm" aria-hidden="true"></span>
                        <span class="mx-1">Loading...</span>
                    </span>
                }
            }
        >

            <span class="py-3">Your Local {state.0.get().name} Time</span>
            <p class="">{local_str}</p>
        </Show>
    }
}
