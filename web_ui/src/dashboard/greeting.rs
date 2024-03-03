use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::stores::get_state;
use crate::stores::states::EarthCalendarState;

#[component]
/// This component introduces the new user.
pub fn Page() -> impl IntoView {
    let state = get_state::<EarthCalendarState>("preferred-calendar");

    view! {
        <p class="fs-4 lead p-4 bg-transparent card-text">
            "This is dashboard where you can utilize calendars and datetimes. 🚀"
        </p>
        <div class="vstack gap-4 py-5 d-flex justify-content-center align-items-center">
            <p class="lead">See a reference of supported timezones?</p>

            <A
                class="text-decoration-none btn btn-lg btn-success"
                href="celestial-tzs"
                attr:title="List of Timezones"
            >
                Celestial Timezones
                <i class="bi bi-view-list fs-5 btn btn-lg"></i>
            </A>
        </div>
        <div class="py-2">

            <p class="lead">Or try out some of our converters?</p>
            <div class="vstack gap-4 py-5 d-flex justify-content-center align-items-center">
                <A
                    class="text-decoration-none btn btn-lg btn-primary"
                    href="celestial-date"
                    attr:title="Date Converter"
                >
                    Date Converter
                    <i class="bi bi-calendar fs-5 btn btn-lg"></i>
                </A>
                <A
                    class="text-decoration-none btn btn-lg btn-danger"
                    href="celestial-time"
                    attr:title="Time Converter"
                >
                    Time Converter
                    <i class="bi bi-clock fs-5 btn btn-lg"></i>
                </A>
            </div>
        </div>
    }
}
