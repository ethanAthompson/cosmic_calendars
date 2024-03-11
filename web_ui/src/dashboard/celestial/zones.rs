use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::stores::get_state;
use crate::stores::states::{CosmicTimeZoneState, UserNameState};
use crate::stores::states::EarthCalendarState;
use crate::utils::calendar::SupportedCalendars;
use crate::utils::celestial::direct_tzname_image;
use crate::time::martian::LocalMars;
use crate::time::earth::ExtraLocaleEarth;
use crate::utils::filter::filter_list;
use crate::time::mercury::LocalMercury;

#[component]
/// This component displays timezones from IANA DB + Celestial Timezones from RSAR DB
pub fn Timezones() -> impl IntoView {
    view! {
        <ul
            id="celestial-timezones-main"
            class="list-group list-group-flush text-center rounded-bottom overflow-y-scroll "
            style="max-height: 350px;"
        >

            <LocalMars/>
            <LocalMercury/>
            <ExtraLocaleEarth/>
        </ul>
    }
}