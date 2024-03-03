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
/// This component represents the grreetings a cosmic user sees along with the ability to look at supported bodies.
pub fn Content() -> impl IntoView {
    view! {
        <div class="bg-transparent container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <h1 class="bg-transparent display-5 fw-bold">Welcome to Cosmic Calendars</h1>
            <p class="bg-transparent col-md-8 fs-4 mx-4">A solution to celestial communication</p>
            <div class="hstack">
                <button
                    class=" btn btn-success mx-4"
                    type="button"
                    data-bs-toggle="offcanvas"
                    data-bs-target="#offcanvasBottom"
                    aria-controls="offcanvasBottom"
                >

                    "See what's supported"
                    <i class=" mx-2 bi bi-pencil"></i>
                </button>

                <A href="tutorials" class=" btn btn-secondary mx-2">
                    Tutorials
                    <i class=" mx-2 bi bi-book-half"></i>
                </A>
            </div>

            <div
                class=" offcanvas offcanvas-bottom w-auto h-auto"
                tabindex="-1"
                id="offcanvasBottom"
                aria-labelledby="offcanvasBottomLabel"
            >
                <div class=" offcanvas-header">
                    <h5 class=" offcanvas-title fs-2" id="offcanvasBottomLabel">
                        Celestial Bodies
                    </h5>
                    <button
                        type="button"
                        class="btn-close fs-4"
                        data-bs-dismiss="offcanvas"
                        aria-label="Close"
                    ></button>

                </div>
                <div class="offcanvas-body small rounded-4">
                    <SupportedData/>
                </div>

            </div>
        </div>
    }
}

