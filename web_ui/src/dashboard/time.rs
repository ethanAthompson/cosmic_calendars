use crate::stores::get_state;
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
pub fn Page() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <div class="fadeIn">
            <div class="container-fluid">
                <p class="lead">
                    Celestial Search Bar: Select x timezones for earth and celestial bodies...
                </p>
                <SearchBar/>
                <div class="py-4">
                    <div class="p-2 rounded-5 focus-ring focus-ring-primary ">
                        <Output/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SearchBar() -> impl IntoView {
    let battery = create_rw_signal("".to_string());
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    let filter_search = move |ev| {
        battery.set(event_target_value(&ev));
        filter_list("celestial-timezones-main", battery.get());
    };

    view! {
        <div class="p-4 border rounded-5 focus-ring focus-ring-primary">
            <div class="me-auto rounded-2 input-group input-group-lg flex-nowrap border border-5 rounded-top">
                <i
                    class="input-group-text bi bi-search py-2 rounded-0"
                    id="SearchModalLabel-wrapping"
                ></i>
                <input
                    type="text"
                    class="focus-ring focus-ring-success form-control py-2 rounded-0"
                    placeholder="Search IANA + OSM Databases"
                    aria-label="Search IANA + OSM Databases"
                    aria-describedby="SearchModalLabel-wrapping"
                    on:input=filter_search
                    prop:value=battery
                />
            </div>
            <ul class="list-group list-group-flush">
                <ul
                    id="celestial-timezones-main"
                    class="list-group list-group-flush text-center rounded-bottom overflow-y-scroll "
                    style="max-height: 150px;"
                >
                    <CurrentMars/>
                    <CurrentEarth/>
                </ul>
            </ul>
        </div>
    }
}

#[component]
pub fn Output() -> impl IntoView {
    /*
       When the user clicks <Japan> then the dynamic component is appended to the column vectors,
       if the host is earth go in earth, anything else goes into the celestial column.
    */
    let earth_column = create_rw_signal(Vec::<CosmicTimeColumn>::new());
    let celestial_column = create_rw_signal(Vec::<CosmicTimeColumn>::new());
    let earth_cosmic_times = get_state::<Vec<CosmicTimeColumn>>("dashboard-earth-cosmic-times");
    let celestial_cosmic_times = get_state::<Vec<CosmicTimeColumn>>("dashboard-celestial-cosmic-times");

    create_effect(move |_| {
        earth_column.set(earth_cosmic_times.0.get());
    });

    let reset = move |ev: MouseEvent| {
        earth_column.get().clear();;
        earth_cosmic_times.1.set(Vec::new());
    };

    view! {
        <div class="py-4">
            <span class="btn btn-danger py-2 w-100" on:click=reset>
                <i class="bi bi-arrow-clockwise fs-5"></i>
                <span class="fs-5 mx-2">Reset</span>
            </span>
        </div>

        <div class="hstack gap-4 p-4 me-auto rounded-2 input-group input-group-lg flex-nowrap border border-5 rounded-top">
            <div class="mx-4 py-5 ">
                <For
                    // Data was not updating becuase the keys were not unique..
                    each=move || earth_column.get().clone()
                    key=|column| column.tz.clone()
                    children=move |data: CosmicTimeColumn| {
                        view! {
                            <Dynamic input=CosmicTimeColumn {
                                host: data.host,
                                tz: data.tz,
                                time: data.time,
                            }/>
                        }
                    }
                />

            </div>
            <Show when=move || {
                (earth_cosmic_times.0.get().len() > 0 && celestial_cosmic_times.0.get().len() > 0)
                    == true
            }>
                <div class="mx-2 ">
                    <i class="bi display-1 bi-arrow-right"></i>
                </div>
            </Show>
            <div class="mx-4 py-5 ">
                <For
                    each=move || celestial_column.get().clone()
                    key=|column| column.tz.clone()
                    children=move |data: CosmicTimeColumn| {
                        view! {
                            <Dynamic input=CosmicTimeColumn {
                                host: data.host,
                                tz: data.tz,
                                time: data.time,
                            }/>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
pub fn Dynamic(#[prop(into)] input: CosmicTimeColumn) -> impl IntoView {
    view! {
        <div class="py-2">
            <div class="card mb-3 py-0" style="max-width: 540px; min-height: 100px;">
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
                            <h5 class="card-title">{input.tz.clone()}</h5>
                            <p class="card-text">{input.time}</p>
                            <p class="card-text">
                                <small class="text-body-secondary">
                                    <div class="form-check d-flex gap-2 justify-content-end">
                                        <input
                                            class="form-check-input"
                                            type="checkbox"
                                            value=""
                                            id="flexCheckDefault-increase-readability"
                                        />
                                        <label
                                            class="form-check-label"
                                            for="flexCheckDefault-increase-readability"
                                        >
                                            Increase Readability
                                        </label>
                                    </div>
                                </small>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/* Below are the current times for mars: Should be a single function that loops over an enum or such like.. */
#[component]
pub fn CurrentMars() -> impl IntoView {
    view! {
        <MartianTimezone tz=Martian::MTCn5 radio=1/>
        <MartianTimezone tz=Martian::MTCn4 radio=2/>
        <MartianTimezone tz=Martian::MTCn3 radio=3/>
        <MartianTimezone tz=Martian::MTCn2 radio=4/>
        <MartianTimezone tz=Martian::MTCn1 radio=5/>
        <MartianTimezone tz=Martian::MTC radio=6/>
        <MartianTimezone tz=Martian::MTCp1 radio=7/>
        <MartianTimezone tz=Martian::MTCp2 radio=8/>
        <MartianTimezone tz=Martian::MTCp3 radio=9/>
        <MartianTimezone tz=Martian::MTCp4 radio=10/>
        <MartianTimezone tz=Martian::MTCp5 radio=11/>
    }
}

#[component]
/* Make this better by looping over rust_solar martian timezones; I have to clean up the library to make utilizing it in leptos easier.. */
pub fn MartianTimezone(tz: Martian, radio: u32) -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <li class="list-group-item" id=format!("Martian-{}", tz.get_str("Name").unwrap())>
            <span class="badge rounded-pill text-bg-danger">Mars</span>
            <span
                class="navbar-brand"
                on:click=move |_| {
                    logging::log!("{:?}", format!("{}", tz.get_str("Code").unwrap()));
                }
            >

                <label
                    class="btn btn-lg container-fluid btn-body"
                    for=format!("vbtn-radio{}", radio)
                >
                    {tz.get_str("Name").unwrap()}
                </label>
            </span>
        </li>
    }
}

/* Below are the current times for Earth */
#[component]
pub fn CurrentEarth() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let earth_cosmic_times = get_state::<Vec<CosmicTimeColumn>>("dashboard-earth-cosmic-times");

    let timezones =EarthTimeZones::all_timezones(Vec::new());

    create_effect(move |_| {
        /* WARNING: This causes a BorrowMut Error */
        // earth_cosmic_times.1.set(earth_cosmic_times_ref.get());
    });


    let get_id = move |ev: MouseEvent| {
        let id = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap()
            .id();

        earth_cosmic_times.1.update(|t| {
            t.push(CosmicTimeColumn {
                host: "Earth".to_string(),
                tz: id.clone(),
                /* 2/28/24: Work on copying time/mod.rs implementation */
                time: "Do I work?".to_string(),
            });
        });

        logging::log!("{:?} Has generated..", id);
    };

    let view_tz = timezones
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
