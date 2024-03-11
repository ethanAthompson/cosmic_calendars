use chrono::Local;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use osm_db::planets::mercury::Mercury;
use osm_db::kepler::TimeZone;
use strum::EnumProperty;
use web_sys::Event;
use web_sys::MouseEvent;

use crate::env::{LEFTCARD, RIGHTCARD};
use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use osm_db::planets::mercury::Mercurian;

pub fn set_mercury_tz() -> osm_db::kepler::DateTime {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    match state.0.get().timezone.as_str() {
        "EYT" => Mercury.now(Mercurian::MeTCn6),
        "COT" => Mercury.now(Mercurian::MeTCn5),
        "BAT" => Mercury.now(Mercurian::MeTCn4),
        "ENT" => Mercury.now(Mercurian::MeTCn3),
        "RIT" => Mercury.now(Mercurian::MeTCn2),
        "CST" => Mercury.now(Mercurian::MeTCn1),
        "HT" => Mercury.now(Mercurian::MeTC),
        "SIT" => Mercury.now(Mercurian::MeTCp1),
        "BNT" => Mercury.now(Mercurian::MeTCp2),
        "BST" => Mercury.now(Mercurian::MeTCp3),
        "HKT" => Mercury.now(Mercurian::MeTCp4),
        "CAT" => Mercury.now(Mercurian::MeTCp5),
        "SI" => Mercury.now(Mercurian::MeTCp6),
        _ => Mercury.now(Mercurian::MeTC),
    }
}

#[component]
/// This is a component that allows the user to see the current time for mars with given timezone
pub fn LocalMercury() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <MercurianTimezone tz=Mercurian::MeTCn6 radio=12/>
        <MercurianTimezone tz=Mercurian::MeTCn5 radio=1/>
        <MercurianTimezone tz=Mercurian::MeTCn4 radio=2/>
        <MercurianTimezone tz=Mercurian::MeTCn3 radio=3/>
        <MercurianTimezone tz=Mercurian::MeTCn2 radio=4/>
        <MercurianTimezone tz=Mercurian::MeTCn1 radio=5/>
        <MercurianTimezone tz=Mercurian::MeTC radio=6/>
        <MercurianTimezone tz=Mercurian::MeTCp1 radio=7/>
        <MercurianTimezone tz=Mercurian::MeTCp2 radio=8/>
        <MercurianTimezone tz=Mercurian::MeTCp3 radio=9/>
        <MercurianTimezone tz=Mercurian::MeTCp4 radio=10/>
        <MercurianTimezone tz=Mercurian::MeTCp5 radio=11/>
        <MercurianTimezone tz=Mercurian::MeTCn6 radio=12/>
    }
}

#[component]
/// This component notifies the tz state, that the user sets it to a certain martain tz.
pub fn MercurianTimezone(tz: Mercurian, radio: u32) -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <li class="list-group-item" id=format!("Mercurian-{}", tz.get_str("Name").unwrap())>
            <span class="badge rounded-pill text-bg-secondary">Mercury</span>
            <span
                class="navbar-brand"
                on:click=move |_| {
                    state
                        .1
                        .set(CosmicTimeZoneState {
                            name: "Mercurian".to_string(),
                            timezone: format!("{}", tz.get_str("Code").unwrap()),
                        });
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
