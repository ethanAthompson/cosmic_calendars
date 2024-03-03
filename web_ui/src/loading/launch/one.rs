
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::use_media_query;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use rust_solar::planets::mars::Martian;
use web_sys::{Event, HtmlAnchorElement, HtmlVideoElement, MouseEvent};

use crate::stores::states::EarthCalendarState;
use crate::utils::trackers::PageProgess;
use crate::utils::calendar::SupportedCalendars;
use crate::utils::sheets::get_data;
use crate::stores::get_state;

#[component]
/// The question that asks you to choose a preferred calendar.
pub fn Question() -> impl IntoView {
    let state = get_state::<EarthCalendarState>("preferred-calendar");

    let state_ref = create_rw_signal(state.0.get().preferred.clone());

    create_effect(move |_| {
        state.0.get();
        // logging::log!("{}", state.0.get().preferred);

        // Tracks the subscribers of the earth calendar state within the dependancy and updates accordingly.
        state_ref.set(state.0.get().preferred.clone());
    });

    view! {
        <div
            class="modal fadeIn"
            id="ModalToggleQ1"
            aria-hidden="true"
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            aria-labelledby="ModalToggleLabelQ1"
            tabindex="-1"
        >
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="lead modal-title fs-5" id="ModalToggleLabelQ1">
                            <span class="dropup-center dropdown py-2">
                                <button
                                    class="btn btn-body dropdown-toggle btn-lg w-100"
                                    type="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    Choose a calendar
                                    <i class="mx-1 bi bi-calendar2-range"></i>
                                </button>
                                <SupportedCalendars/>
                            </span>

                        </h1>

                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">
                        <div class="alert alert-info my-0">
                            <span class="lead">{state_ref}</span>
                        </div>
                    </div>
                    <div class="modal-footer d-flex justify-content-between">
                        <button
                            class="btn bg-primary-subtle"
                            data-bs-target="#ModalToggleQ0"
                            data-bs-toggle="modal"
                        >
                            <i class="mx-1 bi bi-arrow-left"></i>
                            Prev

                        </button>
                        <button
                            class="btn btn-primary"
                            data-bs-target="#ModalToggleQ2"
                            data-bs-toggle="modal"
                        >
                            Next
                            <i class="mx-1 bi bi-arrow-right"></i>
                        </button>
                    </div>
                    <PageProgess percent="25" purpose="2nd page!"/>

                </div>
            </div>
        </div>
    }
}