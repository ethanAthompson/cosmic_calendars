use leptos::html::{time, Input};
use leptos::svg::filter;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use serde_json::to_string;
use serde_json::value::Value;
use serde_with::rust;
use web_sys::{Event, MouseEvent, Node, SubmitEvent};

use super::supported::Table as SupportedData;
#[component]
/// This component represents the welcoming buttons a cosmic user sees.
pub fn Content() -> impl IntoView {
    view! {
        <div class="bg-transparent row align-items-md-stretch ">
            <div class="bg-transparent col-md-6">
                <div class="bg-transparent h-100 p-5 rounded-3">
                    <h1>Date Converter</h1>
                    <p>
                        The date converter allows you to easily convert an earth date into a space date.
                    </p>
                    <A href="dashboard/celestial-date" class=" btn btn-primary btn-lg navbar-item ">
                        " See Date Converter"
                        <i class=" mx-2 bi bi-calendar"></i>
                    </A>
                </div>
            </div>
            <div class="bg-transparent col-md-6">
                <div class=" h-100 p-5 rounded-3">
                    <h1>Time Converter</h1>
                    <p>
                        The time converter allows you to easily convert an earth timezone into a space timezone.
                    </p>
                    <A href="dashboard/celestial-time" class=" btn btn-danger btn-lg navbar-item">
                        "See Time Converter"
                        <i class=" mx-2 bi bi-clock"></i>
                    </A>
                </div>
            </div>
        </div>
    }
}
