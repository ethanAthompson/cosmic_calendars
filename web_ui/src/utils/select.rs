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

use crate::env::{LEFTCARD, RIGHTCARD};
use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use crate::stores::states::{CelestialCalendar, CelestialChosenNameState, CelestialDateState};
use crate::utils::filter::filter_options;

#[component]
/// This component creates a timezone list out of any list of string slices (for any position of a card)
pub fn CelestialSelection(
    mut tz_list: Vec<&'static str>,
    pos: &'static str,
    body: &'static str,
) -> impl IntoView {
    let celestial_calendar_state = get_state::<CelestialCalendar>("dashboard-earth-calendar");
    let tz_signal = create_rw_signal(tz_list);
    let env_body = create_rw_signal(body.clone());
    let env_pos = create_rw_signal(pos.clone());

    create_effect(move |_| {
        filter_options(
            format!(
                "<CelestialSelection>-{}-{}",
                env_pos.get().clone(),
                env_body.get().clone(),
            ),
            match pos {
                RIGHTCARD => celestial_calendar_state.0.get().right,
                LEFTCARD => celestial_calendar_state.0.get().left,
                _ => todo!(),
            },
        );
        if pos == LEFTCARD {
            celestial_calendar_state.1.set(CelestialCalendar {
                left: celestial_calendar_state.0.get().left,
                right: celestial_calendar_state.0.get().right,
            });
        } else {
            celestial_calendar_state.1.set(CelestialCalendar {
                left: celestial_calendar_state.0.get().left,
                right: celestial_calendar_state.0.get().right,
            });
        }
    });
    let view_tz = tz_signal
        .get()
        .into_iter()
        .map(|zone| {
            view! {
                <option
                    class="list-group-item"
                    id=format!("<CelestialSelectionOption>-{}-{}", pos, zone)
                    value=format!("{}-{}", pos, zone)
                >
                    {zone}
                </option>
            }
        })
        .collect_view();

    view! {
        <select
            class="form-select w-100 rounded-0"
            aria-label=format!("Label: {}-{}", pos, body)
            id=format!("<CelestialSelection>-{}-{}", pos, body)
            required=true
            on:click=move |ev| {
                let celestial_calendar_state = get_state::<
                    CelestialCalendar,
                >("dashboard-earth-calendar");
                filter_options(
                    format!(
                        "<CelestialSelection>-{}-{}",
                        env_pos.get().clone(),
                        env_body.get().clone(),
                    ),
                    event_target_value(&ev)
                        .strip_prefix(
                            match pos {
                                RIGHTCARD => "R-",
                                LEFTCARD => "L-",
                                _ => todo!(),
                            },
                        )
                        .unwrap_or("")
                        .to_string(),
                );
                if pos == LEFTCARD {
                    celestial_calendar_state
                        .1
                        .set(CelestialCalendar {
                            left: event_target_value(&ev),
                            right: celestial_calendar_state.0.get().right,
                        });
                } else {
                    celestial_calendar_state
                        .1
                        .set(CelestialCalendar {
                            left: celestial_calendar_state.0.get().left,
                            right: event_target_value(&ev),
                        });
                }
            }
        >

            {view_tz}
        </select>
    }
}
#[component]
/// This component only renders the component if a key in local storage matches it (Left Card)
pub fn CelestialSelectionLeft(
    mut tz_list: Vec<&'static str>,
    pos: &'static str,
    body: &'static str,
) -> impl IntoView {
    let celestial_name_state = get_state::<CelestialChosenNameState>("celestial-clicked-name");

    view! {
        <Show when=move || celestial_name_state.0.get().left == body>
            <CelestialSelection tz_list=tz_list.clone() pos=pos body=body/>
        </Show>
    }
}

#[component]
/// This component only renders the component if a key in local storage matches it (Right Card)
pub fn CelestialSelectionRight(
    mut tz_list: Vec<&'static str>,
    pos: &'static str,
    body: &'static str,
) -> impl IntoView {
    let celestial_name_state = get_state::<CelestialChosenNameState>("celestial-clicked-name");

    view! {
        <Show when=move || celestial_name_state.0.get().right == body>
            <CelestialSelection tz_list=tz_list.clone() pos=pos body=body/>
        </Show>
    }
}

#[component]
pub fn RsarCalendarSelectionLeft(
    mut tz_list: Vec<&'static str>,
    pos: &'static str,
    body: &'static str,
) -> impl IntoView {
    let celestial_name_state = get_state::<CelestialChosenNameState>("celestial-clicked-name");

    view! {
        <Show when=move || !(celestial_name_state.0.get().left == "Earth")>
            <CelestialSelection tz_list=tz_list.clone() pos=pos body=body/>
        </Show>
    }
}

#[component]
pub fn RsarCalendarSelectionRight(
    mut tz_list: Vec<&'static str>,
    pos: &'static str,
    body: &'static str,
) -> impl IntoView {
    let celestial_name_state = get_state::<CelestialChosenNameState>("celestial-clicked-name");

    view! {
        <Show when=move || !(celestial_name_state.0.get().right == "Earth")>
            <CelestialSelection tz_list=tz_list.clone() pos=pos body=body/>
        </Show>
    }
}
