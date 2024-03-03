
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

use crate::time::earth::{LocalEarth, ExtraLocaleEarth};
use crate::utils::trackers::PageProgess;
use crate::time::LocalTimer;
use crate::time::martian::LocalMars;
use crate::utils::sheets::get_data;
use crate::stores::get_state;

#[component]
/// The question that asks you to select a celestial timezone (earth/mars/...)
pub fn Question() -> impl IntoView {
    view! {
        <div
            class="modal fadeIn"
            id="ModalToggleQ2"
            aria-hidden="true"
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            aria-labelledby="ModalToggleLabelQ2"
            tabindex="-1"
        >
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="lead modal-title fs-5" id="ModalToggleLabelQ2">
                            Choose a celestial timezone.
                            <i class="mx-1 bi bi-rocket"></i>
                        </h1>

                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">
                        <div class="alert alert-primary ">
                            <LocalTimer/>
                        </div>
                        <div class="accordion accordian-flush" id="Q1accordion">
                            <ChooseEarth/>
                            <ChooseMars/>
                            <ChooseLuna/>
                            <Choose3Vesta/>
                        </div>
                    </div>
                    <div class="modal-footer d-flex justify-content-between">
                        <button
                            class="btn bg-primary-subtle"
                            data-bs-target="#ModalToggleQ1"
                            data-bs-toggle="modal"
                        >
                            <i class="mx-1 bi bi-arrow-left"></i>
                            Prev

                        </button>
                        <button
                            class="btn btn-primary"
                            data-bs-target="#ModalToggleQ3"
                            data-bs-toggle="modal"
                        >
                            Next
                            <i class="mx-1 bi bi-arrow-right"></i>
                        </button>
                    </div>
                    <PageProgess percent="50" purpose="3rd page!"/>
                </div>
            </div>
        </div>
    }
}


#[component]
pub fn ChooseEarth() -> impl IntoView {
    view! {
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button
                    class="accordion-button"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#Q0collapseOne"
                    aria-expanded="false"
                    aria-controls="Q0collapseOne"
                >
                    Earth
                </button>
            </h2>
            <div
                id="Q0collapseOne"
                class="accordion-collapse collapse"
                data-bs-parent="#01accordion"
            >
                <div class="p-2">
                    <LocalEarth/>
                </div>
                <div class="overflow-auto" style="max-height:250px;">
                    <ExtraLocaleEarth/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ChooseMars() -> impl IntoView {
    view! {
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button
                    class="accordion-button collapsed"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#Q1collapseOne"
                    aria-expanded="false"
                    aria-controls="Q1collapseOne"
                >
                    Mars
                </button>
            </h2>
            <div
                id="Q1collapseOne"
                class="accordion-collapse collapse"
                data-bs-parent="#Q1accordion"
            >
                <div class="accordion-body">
                    <ul class="list-group list-group-flush flex-fill">

                        <LocalMars/>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ChooseLuna() -> impl IntoView {
    view! {
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button
                    class="accordion-button collapsed"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#Q2collapseTwo"
                    aria-expanded="false"
                    aria-controls="QcollapseTwo"
                >
                    Luna (WIP)
                </button>
            </h2>
            <div
                id="Q2collapseTwo"
                class="accordion-collapse collapse"
                data-bs-parent="#Q2accordion"
            >
                <div class="accordion-body"></div>
            </div>
        </div>
    }
}

#[component]
pub fn Choose3Vesta() -> impl IntoView {
    view! {
        <div class="accordion-item">
            <h2 class="accordion-header">
                <button
                    class="accordion-button collapsed"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#Q3collapseThree"
                    aria-expanded="false"
                    aria-controls="Q3collapseThree"
                >
                    3 Vesta (WIP)
                </button>
            </h2>
            <div
                id="Q3collapseThree"
                class="accordion-collapse collapse"
                data-bs-parent="#Q3accordion"
            >
                <div class="accordion-body"></div>
            </div>
        </div>
    }
}