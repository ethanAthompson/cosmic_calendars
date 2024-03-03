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

use crate::stores::get_state;
use crate::stores::states::EarthCalendarState;
use crate::stores::states::UserNameState;
use crate::user::signup::SignUp;
use crate::utils::calendar::SupportedCalendars;
use crate::utils::sheets::get_data;
use crate::utils::trackers::PageProgess;

#[component]
/// The question that asks you to who you are?
pub fn Question() -> impl IntoView {

    let username = create_rw_signal("".to_string());
    let state = get_state::<UserNameState>("home-user-name");
    let state_ref = create_rw_signal(state.0.get().value);
    let is_stopped = create_rw_signal(false);

    create_effect(move |_| {
        // logging::log!("{:?}", state.0.get().value);

        if state.0.get().value.is_empty() {
            is_stopped.set(true);
        } else {
            is_stopped.set(false);
        }
    });


    view! {
        <div
            class="modal fadeIn"
            id="ModalToggleQ3"
            aria-hidden="true"
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            aria-labelledby="ModalToggleLabelQ3"
            tabindex="-1"
        >

            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="lead modal-title fs-5" id="ModalToggleLabelQ3">
                            "Before you complete the setup, who are you?"
                        </h1>

                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body position-relative">
                        <div class="container-fluid w-auto mx-auto">
                            <SignUp/>
                        </div>
                    </div>
                    <div class="modal-footer d-flex justify-content-between align-items-center">
                        <button
                            class="btn bg-primary-subtle"
                            data-bs-target="#ModalToggleQ2"
                            data-bs-toggle="modal"
                        >
                            <i class="mx-1 bi bi-arrow-left"></i>
                            Prev
                        </button>
                        <button
                            class="btn btn-primary"
                            data-bs-target="#ModalToggleQ4"
                            data-bs-toggle="modal"
                            disabled=is_stopped
                        >

                            Next
                            <i class="mx-1 bi bi-arrow-right"></i>
                        </button>
                    </div>
                    <PageProgess percent="75" purpose="4th page!"/>
                </div>
            </div>
        </div>
    }
}
