use crate::dashboard::date::options::*;
use crate::dashboard::date::Dynamic;
use crate::stores::get_state;
use crate::stores::states::CosmicDateRow;
use crate::stores::states::CosmicTimeColumn;
use crate::stores::states::EarthCalendarState;
use crate::stores::states::{CosmicTimeZoneState, UserNameState};
use crate::time::earth::ExtraLocaleEarth;
use crate::time::martian::LocalMars;
use crate::utils::calendar::SupportedCalendars;
use crate::utils::celestial::direct_tzname_image;
use crate::utils::colors::*;
use crate::utils::filter::filter_list;
use crate::utils::images::direct_profile_picture;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use icu_calendar::AsCalendar;
use icu_calendar::Gregorian;
use leptos::server_fn::default;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use osm_db::julian::Julian;
use osm_db::kepler::Body;
use osm_db::kepler::TimeZone;
use osm_db::planets::earth::EarthDate;
use osm_db::planets::earth::EarthDateTime;
use osm_db::planets::earth::EarthTimeZones;
use osm_db::planets::earth::RustSolarCalendar;
use osm_db::planets::mars::Mars;
use osm_db::planets::mars::Martian;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use strum::EnumProperty;
use web_sys::Event;
use web_sys::HtmlLabelElement;
use web_sys::HtmlLiElement;
use web_sys::HtmlSpanElement;
use web_sys::HtmlUListElement;
use web_sys::MouseEvent;
extern crate inflector;
use inflector::Inflector;

/* The Celestial Date -> Earth Date */
#[component]
pub fn Page() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let page_state = get_state::<String>("date-converter-page-state");
    let celestial_cosmic_dates = get_state::<Vec<CosmicDateRow>>("dashboard-comsic-earth-dates");
    let celestial_column = create_rw_signal(Vec::<CosmicDateRow>::new());

    let year = create_rw_signal("".to_string());
    let ls = create_rw_signal("".to_string());
    let calendar = create_rw_signal("".to_string());
    let host = create_rw_signal("".to_string());

    create_effect(move |_| {
        celestial_column.set(celestial_cosmic_dates.0.get());
    });

    let reset = move |ev: MouseEvent| {
        logging::log!("Resetting Rows");

        celestial_column.get().clear();
        celestial_cosmic_dates.1.set(Vec::new());

        year.set("".to_string());
        ls.set("".to_string());
        calendar.set("".to_string());
        host.set("".to_string());
    };

    let append_row = move |ev: MouseEvent| {
        logging::log!("Appending Row");

        // logging::log!("{:?}", host.get());
        // logging::log!(
        //     "{:?},{:?},{:?},{:?}",
        //     year.get().is_empty(),
        //     ls.get().is_empty(),
        //     calendar.get().is_empty(),
        //     host.get().is_empty()
        // );

        if year.get().is_empty() == false
            && ls.get().is_empty() == false
            && calendar.get().is_empty() == false
            && host.get().is_empty() == false
        {
            // ! **************** All you have to do is update this part only! ****************************
            match host.get().as_str() {
                "Mars" => {
                    // logging::log!("Mars Date!");

                    celestial_cosmic_dates.1.update(|t| {
                        t.push(CosmicDateRow {
                            host: host.get(),
                            cal: calendar.get(),
                            // I'm planning to throw the created date into whatever the calendar date is,
                            // so its accurrate to that calendar.
                            date: format!(
                                "{} → {}",
                                format!("{}/{}", year.get(), ls.get()),
                                format!("{}/{}/{} {}", 2024, 3, 4, "ce")
                            ),
                            id: uuid::Uuid::new_v4().to_string(),
                        });
                    });
                }
                _ => {
                    logging::log!("Soon!");
                }
            }
        }
    };

    let update_input = move |ev: Event, input: String| match input.as_str() {
        "calendar" => calendar.set(event_target_value(&ev)),
        "host" => host.set(event_target_value(&ev)),
        "year" => year.set(event_target_value(&ev)),
        "ls" => ls.set(event_target_value(&ev)),
        _ => todo!(),
    };

    view! {
        <Show when=move || page_state.0.get() == "page-2".to_string()>
            <div class="fadeIn">
                <p class="lead">Celestial Date to Earth Date</p>
                <div class="py-4">
                    <Form
                        action=""
                        class="vstack gap-4 p-4 rounded-5 d-flex justify-content-center focus-ring focus-ring-primary"
                    >
                        <div class="mx-4">
                            <div class="card">
                                <div class="card-body">
                                    <div class="px-0 mx-0">
                                        <select
                                            class="form-select fs-5 text-center"
                                            aria-label="Select a preferred calendar"
                                            required=true
                                            name="earth-calendar"
                                            prop:value=calendar

                                            on:input=move |ev: Event| update_input(
                                                ev,
                                                "calendar".to_string(),
                                            )
                                        >

                                            <CalendarOptions/>
                                        </select>
                                        <select
                                            class="form-select fs-5 text-center"
                                            aria-label="Select a host"
                                            required=true
                                            name="host"
                                            prop:value=host
                                            on:input=move |ev: Event| update_input(
                                                ev,
                                                "host".to_string(),
                                            )
                                        >

                                            <HostOptions keep_earth=false/>
                                        </select>
                                    </div>
                                    <div class="input-group">
                                        <input
                                            type="text"
                                            placeholder="LS"
                                            name="celestial-ls"
                                            prop:value=ls
                                            aria-label="Solar Longitude"
                                            on:input=move |ev: Event| update_input(ev, "ls".to_string())
                                            class="form-control py-4 text-center fs-2 focus-ring focus-ring-success"
                                            required=true
                                        />
                                        <input
                                            type="text"
                                            placeholder="YYYY"
                                            name="celestial-year"
                                            aria-label="Year of Body"
                                            prop:value=year
                                            on:input=move |ev: Event| update_input(
                                                ev,
                                                "year".to_string(),
                                            )

                                            class="form-control py-4 text-center fs-2 focus-ring focus-ring-success"
                                            required=true
                                        />
                                    </div>
                                    <button
                                        on:click=append_row
                                        type="submit"
                                        class="hstack d-flex py-4 justify-content-center gap-2 btn btn-outline-success rounded-0 btn-lg w-100 text-center"
                                    >
                                        <i class="bi bi-box fs-5"></i>
                                        Create Live Date
                                    </button>
                                    <button
                                        class="btn btn-outline-success rounded-0 py-4 w-100 rounded-bottom"
                                        on:click=reset
                                    >
                                        <i class="bi bi-arrow-clockwise fs-5"></i>
                                        <span class="fs-5 mx-2">Reset</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="mx-4 d-flex justify-content-center">
                            <div class="card" style="width: 150rem;">
                                <h5 class="card-text p-4">Date History</h5>
                                <div class="card-body overflow-y-auto">
                                    <div
                                        style="max-height: 350px;"
                                        class="d-flex justify-content-center"
                                    >
                                        <For
                                            // Data was not updating becuase the keys were not unique..
                                            each=move || celestial_column.get().clone()
                                            key=|column| column.id.clone()
                                            children=move |data: CosmicDateRow| {
                                                view! {
                                                    <Dynamic input=CosmicDateRow {
                                                        host: data.host,
                                                        cal: data.cal,
                                                        date: data.date,
                                                        id: data.id,
                                                    }/>
                                                }
                                            }
                                        />

                                    </div>
                                </div>
                            </div>
                        </div>
                    </Form>
                </div>
            </div>
        </Show>
    }
}
