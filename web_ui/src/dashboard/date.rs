pub mod left;
pub mod options;
pub mod right;

use crate::dashboard::date::left::Page as Page1;
use crate::dashboard::date::options::*;
use crate::dashboard::date::right::Page as Page2;
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

#[component]
pub fn Page() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let page_state = get_state::<String>("date-converter-page-state");

    if page_state.0.get().is_empty() {
        page_state.1.set("page-1".to_string());
    }

    let false_page = move |ev: MouseEvent| {
        logging::log!("{:?}", "Now Swapping True");

        page_state.1.set("page-2".to_string());
    };

    let true_page = move |ev: MouseEvent| {
        logging::log!("{:?}", "Now Swapping False");

        page_state.1.set("page-1".to_string());
    };

    view! {
        <div class="container-fluid ">
            <Show when=move || page_state.0.get() == "page-1".to_string()>

                <div
                    class="position-absolute"
                    title="Try our Celestial Date to Earth Date Converter"
                >
                    <span
                        on:click=false_page
                        class="btn btn-danger w-50 h-50 position-absolute top-0 start-100 translate-middle p-4 my-2 mx-0 border border-light rounded-top rounded-bottom"
                    >
                        <span class="p-2 text-start">1</span>
                    </span>
                </div>
            </Show>

            <Show when=move || page_state.0.get() == "page-2".to_string()>
                <div
                    class="position-absolute"
                    title="Try our Earth Date to Celestial Date Converter"
                >
                    <span
                        on:click=true_page
                        class="btn btn-success w-50 h-50 position-absolute top-0 start-100 translate-middle p-4 my-2 mx-0 border border-light rounded-top rounded-bottom"
                    >
                        <span class="p-2 text-start">2</span>
                    </span>
                </div>
            </Show>
            <div class="fadeIn">
                <Page1/>
            </div>

            <div class="fadeIn">
                <Page2/>
            </div>
        </div>
    }
}

#[component]
pub fn Dynamic(#[prop(into)] input: CosmicDateRow) -> impl IntoView {
    let input_env = create_rw_signal(input.clone());
    let input_date_env = create_rw_signal(input.date.clone());
    let input_cal_env = create_rw_signal(input.cal.clone());
    let is_changed = create_rw_signal(false);
    let local_calendar = create_rw_signal("".to_string());
    let is_readable = create_rw_signal(false);

    fn debug(ev: Event, input: String) {
        logging::log!(
            "Now Converting : {} into {} calendar",
            input,
            event_target_value(&ev)
        );
    }

    fn filter_numbers(input_string: &str) -> String {
        let re = regex::Regex::new(r"[^\d/]").unwrap();
        re.replace_all(input_string, "").to_string()
    }

    fn setter<T: AsCalendar>(
        ev: Event,
        input_date_env_ref: String,
        input_cal_env_ref: String,
        as_calendar_ref: T,
    ) -> String {
        logging::log!("Regex: {:?}", &filter_numbers(&input_date_env_ref));
        let date = NaiveDate::parse_from_str(&filter_numbers(&input_date_env_ref), "%Y/%m/%d");
        // convert this date into a gregorian date..
        let value = EarthDate {
            year: date.expect("Was Expecting a proper year!").year(),
            month: date.expect("Was Expecting a proper month!").month(),
            day: date.expect("Was Expecting a proper day!").day(),
        };

        logging::log!("cal: {:?}", event_target_value(&ev));
        logging::log!("input_cal: {:?}", input_cal_env_ref);

        let post_cal = RustSolarCalendar.to_calendar(value, input_cal_env_ref, as_calendar_ref);

        let date_str = format!(
            "{}/{}/{} {}",
            post_cal.date.year, post_cal.date.month, post_cal.date.day, post_cal.codes.year
        );
        logging::log!("{:?}", date_str);

        // debug(ev, input_cal_env_ref);

        return date_str;
    }

    let dynamic_calendar = move |ev: Event| {
        is_changed.set(true);
        local_calendar.set(event_target_value(&ev.clone()).to_title_case());

        match event_target_value(&ev).as_str() {
            "gregorian" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    Gregorian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "julian" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::julian::Julian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "indian" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::indian::Indian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "iso" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::iso::Iso::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "chinese" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::chinese::Chinese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "roc" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::roc::Roc::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "ethiopian_alem" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::ethiopian::Ethiopian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "ethiopian_mihret" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::ethiopian::Ethiopian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_heisei" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_reiwa" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_meiji" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_taisho" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_showa" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese_extended" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::JapaneseExtended::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "persian" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::persian::Persian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "coptic" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::coptic::Coptic::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "hebrew" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::hebrew::Hebrew::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "buddhist" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::buddhist::Buddhist::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic_civil" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicCivil::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic_observational" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicObservational::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic_tabular" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicTabular::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic_umm_al_qura" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            _ => {
                logging::log!("Not Available: Please send feedback for this...");
            }
        };
    };

    view! {
        <div class="py-4">
            <div class="card mb-3 py-0 " style="max-width: 700px; min-height: 100px;">
                <div class="row g-0">
                    <div class="col-md-4">
                        // Make image fit so its not too large
                        <img
                            src=direct_profile_picture(input.host.clone())
                            class="img-thumbnail rounded-start h-100"
                            alt=format!("A picture of {}", input.host.clone())
                        />
                    </div>
                    <div class="col-md-8">
                        <div class="card-body position-relative">
                            <span class=format!(
                                "fs-5 position-absolute top-0 start-100 translate-middle badge rounded-pill {}",
                                colored_class_badge(input.host.clone()),
                            )>ID: {input.host.clone()}</span>
                            <h5 class="card-title fs-2 text-info">
                                {input.cal.clone().to_title_case()}
                            </h5>
                            <p class="card-text fs-5">{input_date_env}</p>
                            <Show when=move || { is_changed.get() == true }>
                                <p class="lead">
                                    {input.cal.clone().to_title_case()} " Date"
                                    <i class="mx-2 bi bi-arrow-right"></i>
                                    <span>{local_calendar}</span> " Date"
                                </p>
                            </Show>
                            <Show when=move || {
                                input.host.clone().contains("Earth") && is_changed.get() == false
                            }>
                                <select
                                    class="form-select fs-5  "
                                    aria-label="Select a preferred calendar"
                                    name="earth-calendar"
                                    required=true
                                    on:input=dynamic_calendar
                                >

                                    <CalendarOptions/>
                                </select>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
