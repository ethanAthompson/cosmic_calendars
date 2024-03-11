use chrono::Local;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use rust_solar::kepler::TimeZone;
use rust_solar::planets::mars::Mars;
use strum::{AsRefStr, EnumProperty, VariantArray, EnumIter, IntoEnumIterator};
use web_sys::Event;
use web_sys::MouseEvent;
use rust_solar::planets::mars::Martian;

use crate::stores::get_state;
use crate::stores::states::CosmicTimeZoneState;
use crate::env::{LEFTCARD, RIGHTCARD};
use crate::utils::select::*;

pub fn set_mars_tz() -> rust_solar::kepler::DateTime {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    match state.0.get().timezone.as_str() {
        "NT" => Mars.now(Martian::MTC),
        "AGT" => Mars.now(Martian::MTCn1),
        "MT" => Mars.now(Martian::MTCn2),
        "TT" => Mars.now(Martian::MTCn3),
        "OT" => Mars.now(Martian::MTCn4),
        "AMT" => Mars.now(Martian::MTCn5),
        "ABT" => Mars.now(Martian::MTCp1),
        "HT" => Mars.now(Martian::MTCp2),
        "UT" => Mars.now(Martian::MTCp3),
        "ET" => Mars.now(Martian::MTCp4),
        "ACT" => Mars.now(Martian::MTCp5),
        _ => Mars.now(Martian::MTC),
    }
}

#[component]
/// This is a component that allows the user to see the current time for mars with given timezone
pub fn LocalMars() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

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
/// This component notifies the tz state, that the user sets it to a certain martain tz.
pub fn MartianTimezone(tz: Martian, radio: u32) -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <li class="list-group-item" id=format!("Martian-{}", tz.get_str("Name").unwrap())>
            <span class="badge rounded-pill text-bg-danger">Mars</span>
            <span
                class="navbar-brand"
                on:click=move |_| {
                    state
                        .1
                        .set(CosmicTimeZoneState {
                            name: "Martian".to_string(),
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

#[component]
/// This component notifies the tz state, that the user sets it to a certain martain tz.
pub fn SelectedMartianTimezone(#[prop(optional)] pos:&'static str, tz: Martian, radio: u32) -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");

    view! {
        <option
            class="list-group-item"
            id=format!("{}-Martian-Timezone-{}", pos, tz.get_str("Name").unwrap())
            value=tz.get_str("Name").unwrap()
            class="navbar-brand"
        >
            {tz.get_str("Name").unwrap()}
        </option>
    }
}

#[component]
/// This component updates a martian offset
pub fn MartianOffsets<F>(adjust_offset: F, cosmic_state_ref: RwSignal<String>) -> impl IntoView
where
    F: FnMut(Event) + 'static + Clone,
{
    view! {
        <select class="form-select w-100" on:change=adjust_offset.clone()>
            <SelectedMartianTimezone tz=Martian::MTCn5 radio=1/>
            <SelectedMartianTimezone tz=Martian::MTCn4 radio=2/>
            <SelectedMartianTimezone tz=Martian::MTCn3 radio=3/>
            <SelectedMartianTimezone tz=Martian::MTCn2 radio=4/>
            <SelectedMartianTimezone tz=Martian::MTCn1 radio=5/>
            <SelectedMartianTimezone tz=Martian::MTC radio=6/>
            <SelectedMartianTimezone tz=Martian::MTCp1 radio=7/>
            <SelectedMartianTimezone tz=Martian::MTCp2 radio=8/>
            <SelectedMartianTimezone tz=Martian::MTCp3 radio=9/>
            <SelectedMartianTimezone tz=Martian::MTCp4 radio=10/>
            <SelectedMartianTimezone tz=Martian::MTCp5 radio=11/>
        </select>
    }
}

#[component]
/// This component updates a martian offset for the left or right cards
pub fn MartianOffsetOptions(pos: &'static str) -> impl IntoView
{
    view! {
        <SelectedMartianTimezone pos=pos tz=Martian::MTCn5 radio=1/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCn4 radio=2/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCn3 radio=3/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCn2 radio=4/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCn1 radio=5/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTC radio=6/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCp1 radio=7/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCp2 radio=8/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCp3 radio=9/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCp4 radio=10/>
        <SelectedMartianTimezone pos=pos tz=Martian::MTCp5 radio=11/>
    }
}
