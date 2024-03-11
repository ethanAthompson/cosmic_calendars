use crate::dashboard::date::Dynamic;
use crate::dashboard::date::options::*;
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
use osm_db::planets::mercury::Mercury;
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


/* The Earth Date -> Celestial Date */
#[component]
pub fn Page() -> impl IntoView {
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

        // ! Does not reset year??
        logging::log!("Year: {:?}", year.get());
        month.set("".to_string());
        day.set("".to_string());
        calendar.set("".to_string());
        host.set("".to_string());
        year.set("".to_string());

    };

    let append_row = move |ev: MouseEvent| {
        if (celestial_cosmic_dates.0.get().len() == 0
            || (year.get().is_empty() == false
                && month.get().is_empty() == false
                && day.get().is_empty() == false
                && calendar.get().is_empty() == false
                && host.get().is_empty() == false))
        {
            // ! **************** All you have to do is update this part only! ****************************
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
                                "{}/{}/{}, {:.2}° in {}",
                                date.year, date.month, date.day, date.ls, date.season
                            ),
                            id: uuid::Uuid::new_v4().to_string(),
                        });
                    });
                }
                "Mercury" => {
                    logging::log!("Mars Date!");
                    // ! Look here for conversion!.
                    let jd = Julian.get_jd(
                        year.get().parse::<i32>().unwrap(),
                        month.get().parse::<i32>().unwrap(),
                        day.get().parse::<i32>().unwrap(),
                        12.0,
                    );
                    let date = Mercury.to_date(jd);

                    celestial_cosmic_dates.1.update(|t| {
                        t.push(CosmicDateRow {
                            host: host.get(),
                            cal: calendar.get(),
                            date: format!(
                                "{}/{}/{}, {:.2}° in {}",
                                date.year, date.month, date.day, date.ls, date.season
                            ),
                            id: uuid::Uuid::new_v4().to_string(),
                        });
                    });
                }
                "Earth" => {

                    fn setter<T: AsCalendar>(
                        input_cal_env_ref: String,
                        input_date: String,
                        as_calendar_ref: T,
                    ) -> String {
                        let date = NaiveDate::parse_from_str(&input_date, "%Y/%m/%d");
                        // convert this date into a gregorian date..
                        let value = EarthDate {
                            year: date.expect("Was Expecting a proper year!").year(),
                            month: date.expect("Was Expecting a proper month!").month(),
                            day: date.expect("Was Expecting a proper day!").day(),
                        };
                
                
                        let post_cal = RustSolarCalendar.construct_calendar(value, input_cal_env_ref);
                
                        let date_str = format!(
                            "{}/{}/{} {}",
                            post_cal.date.year, post_cal.date.month, post_cal.date.day, post_cal.codes.year
                        );
                        logging::log!("{:?}", date_str);
                
                
                        return date_str;
                    }

                    
                    celestial_cosmic_dates.1.update(|t| {
                        // construct date in the chosen calendar first, then return string, so you can show era
                        t.push(CosmicDateRow {
                            host: host.get(),
                            cal: calendar.get(),
                            date:  match calendar.get().as_str() {
                                "gregorian" => {
                                    setter(
                                        calendar.get(),
                                        format!("{}/{}/{}", year.get(), month.get(), day.get()),
                                        Gregorian::default(),
                                    )
                                }
                                "julian" => {
                               setter(
                                calendar.get(),
                                format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::julian::Julian::default(),
                                    )
                                }
                                "indian" => {
                                   setter(
                                    calendar.get(),                                      
                                      format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::indian::Indian::default(),
                                    )
                                }
                                "iso" => {
                                   setter(
                                    calendar.get(),                                     
                                       format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::iso::Iso::default(),
                                    )
                                }
                                "chinese" => {
                                  setter(
                                    calendar.get(),                                       
                                     format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::chinese::Chinese::default(),
                                    )
                                }
                                "roc" => {
                                    setter(
                                        calendar.get(),                                    
                                            format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::roc::Roc::default(),
                                    )
                                }
                                "ethiopian_alem" => {
                               setter(
                                calendar.get(),                                       
                                 format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::ethiopian::Ethiopian::default(),
                                    )
                                }
                                "ethiopian_mihret" => {
                                  setter(
                                    calendar.get(),                                       
                                     format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::ethiopian::Ethiopian::default(),
                                    )
                                }
                                "japanese_heisei" => {
                                 setter(
                                    calendar.get(),                                      
                                      format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::Japanese::default(),
                                    )
                                }
                                "japanese_reiwa" => {
                                    setter(
                                        calendar.get(),                                   
                                             format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::Japanese::default(),
                                    )
                                }
                                "japanese_meiji" => {
                                 setter(
                                    calendar.get(),                                      
                                      format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::Japanese::default(),
                                    )
                                }
                                "japanese_taisho" => {
                                   setter(
                                    calendar.get(),                                    
                                        format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::Japanese::default(),
                                    )
                                }
                                "japanese_showa" => {
                                     setter(
                                        calendar.get(),                                    
                                            format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::Japanese::default(),
                                    )
                                }
                                "japanese_extended" => {
                                 setter(
                                    calendar.get(),                                     
                                       format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::japanese::JapaneseExtended::default(),
                                    )
                                }
                                "persian" => {
                                    setter(
                                        calendar.get(),                                     
                                           format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::persian::Persian::default(),
                                    )
                                }
                                "coptic" => {
                                 setter(
                                    calendar.get(),                                        
                                    format!("{}/{}/{}", year.get(), month.get(), day.get()),


                                        icu_calendar::coptic::Coptic::default(),
                                    )
                                }
                                "hebrew" => {
                                  setter(
                                    calendar.get(),                                     
                                       format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::hebrew::Hebrew::new_always_calculating(),
                                    )
                                }
                                "buddhist" => {
                                    setter(
                                        calendar.get(),                                    
                                            format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::buddhist::Buddhist::default(),
                                    )
                                }
                                "islamic_civil" => {
                                   setter(
                                    calendar.get(),                                      
                                      format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::islamic::IslamicCivil::new_always_calculating(),
                                    )
                                }
                                "islamic_observational" => {
                              setter(
                                calendar.get(),                                     
                                   format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::islamic::IslamicObservational::new_always_calculating(),
                                    )
                                }
                                "islamic_tabular" => {
                                   setter(
                                    calendar.get(),                                     
                                       format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::islamic::IslamicTabular::new_always_calculating(),
                                    )
                                }
                                "islamic_umm_al_qura" => {
                                   setter(
                                    calendar.get(),                                      
                                      format!("{}/{}/{}", year.get(), month.get(), day.get()),

                                        icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
                                    )
                                }
                                _ => "".to_string()},
                            
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

                                        <CalendarOptions/>

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
                                        <HostOptions keep_earth=true/>
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
                                    prop:value=year
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
                                    type="submit"

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
