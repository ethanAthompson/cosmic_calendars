use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, Theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::use_media_query;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use rust_solar::planets::mars::Martian;
use web_sys::{Event, HtmlAnchorElement, HtmlVideoElement, MouseEvent};

use crate::utils::trackers::PageProgess;
use crate::utils::sheets::get_data;
use crate::stores::get_state;

#[component]
/// Represents the Question that asks you to manually set (light/dark/system) themes.
pub fn Question() -> impl IntoView {
    let theme = use_theme();

    view! {
        <div
            class="modal zoomer"
            id="ModalToggleQ0"
            aria-hidden="true"
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            aria-labelledby="ModalToggleLabelQ0"
            tabindex="-1"
        >
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="lead modal-title fs-5" id="ModalToggleLabelQ0">
                            Do you prefer light or dark mode?
                            <i class="mx-2 bi bi-palette"></i>
                        </h1>

                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">
                        <div
                            class="btn-group-vertical container-fluid"
                            role="group"
                            aria-label="Vertical radio toggle button group"
                        >
                            <input
                                type="radio"
                                class="btn-check"
                                name="vbtn-radio"
                                id="firstCheckboxSystemTheme"
                                autocomplete="off"
                                on:click=move |_| { theme.set(Theme::System) }

                                checked
                            />
                            <label
                                class="btn btn-outline-success text-body fw-bold "
                                for="firstCheckboxSystemTheme"
                            >
                                "System's"
                                Mode
                                <i class="mx-2 bi bi-sun"></i>
                            </label>
                            <input
                                type="radio"
                                class="btn-check"
                                name="vbtn-radio"
                                on:click=move |_| { theme.set(Theme::Dark) }
                                id="secondCheckboxDarkTheme"
                                autocomplete="off"
                            />
                            <label
                                class="btn btn-outline-success text-body fw-bold"
                                for="secondCheckboxDarkTheme"
                            >
                                Dark Mode
                                <i class="mx-2 bi bi-moon"></i>
                            </label>
                            <input
                                type="radio"
                                class="btn-check"
                                name="vbtn-radio"
                                id="thirdCheckboxLightTheme"
                                autocomplete="off"
                                on:click=move |_| { theme.set(Theme::Light) }
                            />

                            <label
                                class="btn btn-outline-success text-body fw-bold"
                                for="thirdCheckboxLightTheme"
                            >
                                "Light Mode"
                                <i class="mx-2 bi bi-display"></i>
                            </label>
                        </div>

                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn btn-primary"
                            data-bs-target="#ModalToggleQ1"
                            data-bs-toggle="modal"
                        >
                            Next
                            <i class="mx-1 bi bi-arrow-right"></i>
                        </button>
                    </div>
                    <PageProgess percent="0" purpose="1st page!"/>
                </div>
            </div>
        </div>
    }
}