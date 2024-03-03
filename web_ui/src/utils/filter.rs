use crate::stores::get_state;
use crate::stores::states::CelestialCalendar;
use crate::stores::states::CosmicTimeZoneState;
use crate::time::martian::LocalMars;
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
use web_sys::HtmlOptionElement;
use web_sys::HtmlSelectElement;
use web_sys::HtmlSpanElement;
use web_sys::HtmlUListElement;
use web_sys::MouseEvent;

/// This function hides elements based on the verfication of a string.
pub fn filter_list(parent_id: &'static str, client: String) {
    let parent = document()
        .get_element_by_id(parent_id)
        .unwrap()
        .dyn_into::<HtmlUListElement>()
        .expect("<filter_list: parent>: Cannot dynamically cast this element")
        .children();

    for i in 0..=parent.length() - 1 {
        let item = parent
            .item(i)
            .expect("<filter_list: <Range: Parent>>: Item does not exist")
            .id();

        let child = document()
            .get_element_by_id(&item)
            .unwrap()
            .dyn_into::<HtmlLiElement>()
            .unwrap();

        if client.is_empty() {
            child.set_hidden(false);
        } else {
            if item.to_lowercase().contains(&client.to_lowercase()) {
                child.set_hidden(false);
            } else {
                child.set_hidden(true);
            }
        }
    }
}

/// This function hides elements based on the verfication of a string.
pub fn filter_options(parent_id: String, client: String) {
    let parent = document()
        .get_element_by_id(&parent_id)
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .expect("<filter_list: parent>: Cannot dynamically cast this element")
        .children();

        for i in 0..=parent.length() - 1 {
        let item = parent
            .item(i)
            .expect("<filter_list: <Range: Parent>>: Item does not exist")
            .id();

        let child = document()
            .get_element_by_id(&item)
            .unwrap()
            .dyn_into::<HtmlOptionElement>()
            .unwrap();

        if client == child.value() {
            child.set_selected(true);
        } else {
            ()
        }
    }
}

