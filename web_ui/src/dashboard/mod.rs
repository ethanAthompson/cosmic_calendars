/// This module contains code for the list of timezones that are displayed.
pub mod celestial;

/// This module contains routes for the dashboard.
pub mod routes;

/// This module contains greetings for the dashboard.
pub mod greeting;

/// This module contains settings for the dashboard
pub mod settings;

/// This module contains date operations for dashboard
pub mod date;

/// This module contains time operations for dashboard
pub mod time;

use leptos::html::Div;
use leptos::html::Span;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::on_click_outside;
use leptos_use::use_media_query;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::stores::get_state;
use crate::stores::states::EarthCalendarState;
use crate::stores::states::{CosmicTimeZoneState, UserNameState};
use crate::utils::calendar::SupportedCalendars;
use crate::utils::celestial::direct_tzname_image;

#[component]
/// This component is the dashboard that you see.
pub fn Page() -> impl IntoView {
    let state = get_state::<UserNameState>("home-user-name");
    let calendar_state = get_state::<EarthCalendarState>("preferred-calendar");

    let state_ref = create_rw_signal(state.0.get().value);
    let is_large_screen = use_media_query("(min-width: 700px)");

    let is_mobile = create_rw_signal("card-header vstack justify-content-between");

    // a derived signal to adjust class for mobile users..
    let class_mobile = move || -> &str {
        if !is_large_screen.get() == true {
            "card-header vstack justify-content-between"
        } else {
            "card-header hstack justify-content-between"
        }
    };

    view! {
        <div class="container-xxl bd-gutter flex-wrap py-5">
            <div class="card text-center ">
                <div class=class_mobile>
                    <div class="row">
                        <span class="px-4 text-center fs-3 text-warning ">
                            "👋 Welcome back, " {state_ref}
                        </span>
                    </div>
                    <div class="row">
                        <div class="d-flex gap-1 flex-wrap flex-lg-nowrap">
                            <div class="vr"></div>
                            <A href="/dashboard" attr:title="Dashboard Home">
                                <i class="bi bi-house fs-5 btn btn-lg"></i>
                            </A>
                            <A href="celestial-date" attr:title="Date Converter">
                                <i class="bi bi-calendar fs-5 btn btn-lg"></i>
                            </A>
                            <A href="celestial-time" attr:title="Time Converter">
                                <i class="bi bi-clock fs-5 btn btn-lg"></i>
                            </A>
                            <A href="celestial-tzs" attr:title="List of Timezones">
                                <i class="bi bi-view-list fs-5 btn btn-lg"></i>
                            </A>
                            <A href="celestial-settings" attr:title="Settings">
                                <i class="bi bi-gear fs-5 btn btn-lg"></i>
                            </A>
                        </div>
                    </div>
                </div>
                <div class="card-body">
                    <Outlet/>
                </div>
            </div>
        </div>
    }
}

