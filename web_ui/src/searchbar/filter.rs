use leptos::wasm_bindgen::JsCast;
use leptos::{svg::filter, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_media_query, watch_debounced_with_options, WatchDebouncedOptions};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSpanElement;

use crate::context::NavbarSearchString;

#[component]
/// This component represents the filter ability
///
/// Works by traversing <SPAN> nodes in the DOM
///
pub fn Filter() -> impl IntoView {
    let battery = use_context::<RwSignal<NavbarSearchString>>().expect("Cosmic Search Recieved");
    let track_battery = create_rw_signal(battery.get().0);

    let filter_search = move |ev| {
        battery.set(NavbarSearchString(event_target_value(&ev)));
        // logging::log!("You searched for {:?}", battery.get().0);

        let spans = document()
            .get_element_by_id("cosmic-search-main")
            .unwrap()
            .dyn_into::<HtmlSpanElement>()
            .unwrap()
            .children();

        for i in 0..=spans.length() - 1 {
            let item = spans.item(i).expect("Value").id();

            if battery.get().0.is_empty() {
                document()
                    .get_element_by_id(&item)
                    .unwrap()
                    .dyn_into::<HtmlSpanElement>()
                    .unwrap()
                    .set_hidden(false);
            } else {
                // Warnign! Strict Search
                // => item.contains(&battery.get().0) 
                // Warning! Non-Strict Search
                if item.to_lowercase().contains(&battery.get().0.to_lowercase()) {
                    // logging::log!(
                    //     "Found some that contains it: {:?}~{:?}",
                    //     &battery.get().0,
                    //     &item
                    // );
                    document()
                        .get_element_by_id(&item)
                        .unwrap()
                        .dyn_into::<HtmlSpanElement>()
                        .unwrap()
                        .set_hidden(false);
                } else {
                    document()
                        .get_element_by_id(&item)
                        .unwrap()
                        .dyn_into::<HtmlSpanElement>()
                        .unwrap()
                        .set_hidden(true);
                }
            }

            // logging::log!("{:?}", item);
        }
    };

    view! {
        <div class="me-auto input-group input-group-lg flex-nowrap">
            <i class="input-group-text bi bi-search py-2 " id="SearchModalLabel-wrapping"></i>
            <input
                type="text"
                class="form-control py-2"
                placeholder="Search Cosmic Calendars"
                aria-label="Search Cosmic Calendars"
                aria-describedby="SearchModalLabel-wrapping"
                on:input=filter_search
                prop:value=track_battery
                autofocus=true
            />
        </div>
    }
}
