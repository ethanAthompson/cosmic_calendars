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
pub fn Page1() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let page_state = get_state::<String>("date-converter-page-state");
    let celestial_cosmic_dates = get_state::<Vec<CosmicDateRow>>("dashboard-earth-cosmic-dates");
    let celestial_column = create_rw_signal(Vec::<CosmicDateRow>::new());

    let year = create_rw_signal("".to_string());
    let month = create_rw_signal("".to_string());
    let day = create_rw_signal("".to_string());
    let calendar = create_rw_signal("".to_string());
    let host = create_rw_signal("".to_string());

    create_effect(move |_| {
        celestial_column.set(celestial_cosmic_dates.0.get());
    });

    let reset = move |ev: MouseEvent| {
        celestial_column.get().clear();
        celestial_cosmic_dates.1.set(Vec::new());

        year.set("".to_string());
        month.set("".to_string());
        day.set("".to_string());
        // calendar.set(" ".to_string());
        // host.set(" ".to_string());
    };

    let append_row = move |ev: MouseEvent| {
        if year.get().is_empty() == false
            && month.get().is_empty() == false
            && day.get().is_empty() == false
            && calendar.get().is_empty() == false
            && host.get().is_empty() == false
        {
            match host.get().as_str() {
                "Mars" => {
                    logging::log!("Mars Date!");
                    // ! Look here for conversion!.
                    let jd = Julian.get_jd(
                        year.get().parse::<i32>().unwrap(),
                        month.get().parse::<i32>().unwrap(),
                        day.get().parse::<i32>().unwrap(),
                        12.0,
                    );
                    let date = Mars.to_date(jd);

                    celestial_cosmic_dates.1.update(|t| {
                        t.push(CosmicDateRow {
                            host: host.get(),
                            cal: calendar.get(),
                            date: format!(
                                "{}/{}/{}, {:.2} in {}",
                                date.year, date.month, date.day, date.ls, date.season
                            ),
                            id: uuid::Uuid::new_v4().to_string(),
                        });
                    });
                }
                "Earth" => {
                    celestial_cosmic_dates.1.update(|t| {
                        t.push(CosmicDateRow {
                            host: host.get(),
                            cal: calendar.get(),
                            date: format!("{}/{}/{}", year.get(), month.get(), day.get()),
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
        "month" => month.set(event_target_value(&ev)),
        "day" => day.set(event_target_value(&ev)),
        _ => todo!(),
    };

    view! {
        <Show when=move || page_state.0.get() == "page-1".to_string()>
            <div class="fadeIn">

                <p class="lead">Earth Date to Celestial Date</p>
                <div class="py-4">
                    <div class="hstack gap-4 p-2 rounded-5 d-flex justify-content-center focus-ring focus-ring-primary">
                        <div class="mx-4">
                            <h5>Earth Date</h5>
                            <Form action="" class="card" attr:style="width: 22rem;">
                                <div class="card-body container-fluid">
                                    <select
                                        class="form-select fs-5  "
                                        aria-label="Select a preferred calendar"
                                        name="earth-calendar"
                                        required=true
                                        on:input=move |ev: Event| update_input(
                                            ev,
                                            "calendar".to_string(),
                                        )

                                        prop:value=calendar
                                    >

                                        <option selected value="">
                                            "Calendar: <Select a Calendar>"
                                        </option>
                                        <option value="gregorian">Calendar: Gregorian</option>
                                        <option value="julian">Calendar: Julian</option>
                                        <option value="indian">Calendar: Indian</option>
                                        <option value="iso">Calendar: Iso</option>
                                        <option value="chinese">Calendar: Chinese</option>
                                        <option value="roc">
                                            Calendar: Republic of China (Roc)
                                        </option>
                                        <option value="ethiopian-alem">
                                            Calendar: Ethiopian Alem
                                        </option>
                                        <option value="ethiopian-mihret">
                                            Calendar: Ethiopian Mihret
                                        </option>
                                        <option value="japanese-heisei">
                                            Calendar: Japanese Heisei
                                        </option>
                                        <option value="japanese-reiwa">
                                            Calendar: Japanese Reiwa
                                        </option>
                                        <option value="japanese-meiji">
                                            Calendar: Japanese Meiji
                                        </option>
                                        <option value="japanese-taisho">
                                            Calendar: Japanese Taisho
                                        </option>
                                        <option value="japanese-showa">
                                            Calendar: Japanese Showa
                                        </option>
                                        <option value="japanese-extended">
                                            Calendar: Japanese Extended
                                        </option>
                                        <option value="persian">Calendar: Persian</option>
                                        <option value="coptic">Calendar: Coptic</option>
                                        <option value="hebrew">Calendar: Hebrew</option>
                                        <option value="buddhist">Calendar: Buddhist</option>
                                        <option value="islamic-civil">
                                            Calendar: Islamic Civil
                                        </option>
                                        <option value="islamic-observational">
                                            Calendar: Islamic Observational
                                        </option>
                                        <option value="islamic-tabular">
                                            Calendar: Islamic Tabular
                                        </option>
                                        <option value="islamic-umm-al-qura">
                                            Calendar: Islamic Umm Al Qura
                                        </option>
                                    </select>
                                    <select
                                        class="form-select fs-5  "
                                        aria-label="Select a preferred Host"
                                        name="host"
                                        required=true
                                        on:input=move |ev: Event| update_input(
                                            ev,
                                            "host".to_string(),
                                        )

                                        prop:value=host
                                    >

                                        <option selected value="">
                                            "Host: <Select a Host>"
                                        </option>
                                        <option value="Mars">Host: Mars</option>
                                        <option value="Saturn">Host: Saturn</option>
                                        <option value="Neptune">Host: Neptune</option>
                                        <option value="Earth">Host: Earth</option>
                                    </select>
                                </div>
                                <input
                                    type="text"
                                    aria-label="Year"
                                    placeholder="YYYY"
                                    name="earth-year"
                                    on:input=move |ev: Event| update_input(ev, "year".to_string())

                                    class="form-control rounded-0 fs-5 text-center"
                                    required=true
                                    prop:year=year
                                />

                                <input
                                    type="text"
                                    aria-label="Month"
                                    placeholder="MM"
                                    name="earth-month"
                                    on:input=move |ev: Event| update_input(ev, "month".to_string())

                                    class="form-control rounded-0 fs-5 text-center"
                                    required=true
                                    prop:value=month
                                />

                                <input
                                    type="text"
                                    aria-label="Day"
                                    placeholder="DD"
                                    name="earth-day"
                                    on:input=move |ev: Event| update_input(ev, "day".to_string())

                                    class="form-control rounded-0 fs-5 text-center"
                                    required=true
                                    prop:value=day
                                />

                                <button
                                    on:click=append_row
                                    type="submit"
                                    class="hstack d-flex justify-content-center gap-2 btn btn-outline-danger rounded-0 btn-lg w-100 text-center"
                                >
                                    <i class="bi bi-box fs-5"></i>
                                    Create Live Date
                                </button>
                                <button
                                    class="btn btn-outline-danger rounded-0 py-2 w-100 rounded-bottom"
                                    on:click=reset
                                >
                                    <i class="bi bi-arrow-clockwise fs-5"></i>
                                    <span class="fs-5 mx-2">Reset</span>
                                </button>
                            </Form>
                        </div>
                        <div class="mx-4">
                            <h5>Chosen Celestial Body</h5>
                            <div class="card" style="width: 50rem;">
                                <div class="card-body overflow-y-auto">
                                    <div style="max-height: 350px;">
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
                    </div>
                </div>
            </div>
        </Show>
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

    fn setter<T: AsCalendar>(
        ev: Event,
        input_date_env_ref: String,
        input_cal_env_ref: String,
        as_calendar_ref: T,
    ) -> String {
        let date = NaiveDate::parse_from_str(&input_date_env_ref, "%Y/%m/%d");
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
            "{}/{}/{}",
            post_cal.date.year, post_cal.date.month, post_cal.date.day
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
            "ethiopian-alem" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::ethiopian::Ethiopian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "ethiopian-mihret" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::ethiopian::Ethiopian::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-heisei" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-reiwa" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-meiji" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-taisho" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-showa" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::japanese::Japanese::default(),
                );
                input_date_env.update(|v| *v = value);
            }
            "japanese-extended" => {
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
            "islamic-civil" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicCivil::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic-observational" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicObservational::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic-tabular" => {
                let value = setter(
                    ev.clone(),
                    input_date_env.get(),
                    input_cal_env.get(),
                    icu_calendar::islamic::IslamicTabular::new_always_calculating(),
                );
                input_date_env.update(|v| *v = value);
            }
            "islamic-umm-al-qura" => {
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
        <div class="py-2">
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
                                "position-absolute top-0 start-100 translate-middle badge rounded-pill {}",
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

                                    <option selected value="">
                                        "Calendar: <Select a Calendar>"
                                    </option>
                                    <option value="gregorian">Calendar: Gregorian</option>
                                    <option value="julian">Calendar: Julian</option>
                                    <option value="indian">Calendar: Indian</option>
                                    <option value="iso">Calendar: Iso</option>
                                    <option value="chinese">Calendar: Chinese</option>
                                    <option value="roc">Calendar: Republic of China (Roc)</option>
                                    <option value="ethiopian-alem">Calendar: Ethiopian Alem</option>
                                    <option value="ethiopian-mihret">
                                        Calendar: Ethiopian Mihret
                                    </option>
                                    <option value="japanese-heisei">
                                        Calendar: Japanese Heisei
                                    </option>
                                    <option value="japanese-reiwa">Calendar: Japanese Reiwa</option>
                                    <option value="japanese-meiji">Calendar: Japanese Meiji</option>
                                    <option value="japanese-taisho">
                                        Calendar: Japanese Taisho
                                    </option>
                                    <option value="japanese-showa">Calendar: Japanese Showa</option>
                                    <option value="japanese-extended">
                                        Calendar: Japanese Extended
                                    </option>
                                    <option value="persian">Calendar: Persian</option>
                                    <option value="coptic">Calendar: Coptic</option>
                                    <option value="hebrew">Calendar: Hebrew</option>
                                    <option value="buddhist">Calendar: Buddhist</option>
                                    <option value="islamic-civil">Calendar: Islamic Civil</option>
                                    <option value="islamic-observational">
                                        Calendar: Islamic Observational
                                    </option>
                                    <option value="islamic-tabular">
                                        Calendar: Islamic Tabular
                                    </option>
                                    <option value="islamic-umm-al-qura">
                                        Calendar: Islamic Umm Al Qura
                                    </option>
                                </select>
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Page2() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let page_state = get_state::<String>("date-converter-page-state");

    view! {
        <Show when=move || page_state.0.get() == "page-2".to_string()>
            <div class="fadeIn">
                <p class="lead">Celestial Date to Earth Date</p>
                <div class="py-4">
                    <div class="vstack gap-4 p-4 rounded-5 d-flex justify-content-center focus-ring focus-ring-primary">
                        <div class="mx-4">
                            <div class="card">
                                <div class="card-body">
                                    <div class="px-0 mx-0">
                                        <select
                                            class="form-select fs-5 text-center"
                                            aria-label="Select a preferred calendar"
                                        >
                                            <option selected>Select a preferred calendar</option>
                                            <option value="1">One</option>
                                            <option value="2">Two</option>
                                            <option value="3">Three</option>
                                        </select>
                                    </div>
                                    <div class="py-4 px-0 mx-0">
                                        <div class="form-floating d-flex py-5 justify-content-center container-fluid px-0 mx-0 border border-4 rounded-3">
                                            <input
                                                type="text"
                                                readonly
                                                class="form-control-plaintext fs-1 text-center fst-italic"
                                                id="floatingPlaintextInput"
                                                placeholder="name@example.com"
                                                value="4661/3/5"
                                            />
                                            <label for="floatingPlaintextInput" class="text-center ">
                                                <p class="fs-4 lead text-success fw-bold">
                                                    "Output: Neptune Date -> Earth Date"
                                                </p>
                                            </label>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="mx-4 p-4">
                            <h5 class="lead">Celestial Date</h5>
                            <Form action="" class="card">
                                <div class="card-body">
                                    <div class="input-group">
                                        <input
                                            type="text"
                                            placeholder="LS"
                                            name="celestial-ls"
                                            aria-label="Solar Longitude"
                                            class="form-control py-4 text-center fs-2 focus-ring focus-ring-success"
                                            required=true
                                        />
                                        <input
                                            type="text"
                                            placeholder="YYYY"
                                            name="celestial-year"
                                            aria-label="Year of Body"
                                            class="form-control py-4 text-center fs-2 focus-ring focus-ring-success"
                                            required=true
                                        />
                                    </div>
                                    <button class="my-4 hstack d-flex justify-content-center gap-2 btn btn-outline-success rounded py-4 btn-lg w-100 text-center">
                                        <i class="bi bi-box fs-5"></i>
                                        Create Live Date
                                    </button>
                                </div>
                            </Form>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
