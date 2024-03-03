use leptos::set_timeout;
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

use crate::context::NewUser;
use crate::stores::get_state;
use crate::utils::sheets::get_data;
use crate::utils::trackers::PageProgess;

#[component]
/// This component is allows the to enter the application.
pub fn Question() -> impl IntoView {
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    let joined = move |route: &'static str| {
        let battery = use_context::<RwSignal<NewUser>>().expect("User accepted");
        let navigate = leptos_router::use_navigate();

        battery.set(NewUser(true));
        is_new_user.1.set(NewUser(true).0);
        
        // manually directs the user to a specific route
        navigate(&format!("{}", route), Default::default());
  
    };

    view! {
        <div
            class="modal fadeIn"
            id="ModalToggleQ4"
            aria-hidden="true"
            data-bs-backdrop="static"
            data-bs-keyboard="false"
            aria-labelledby="ModalToggleLabelQ4"
            tabindex="-1"
        >

            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="lead modal-title fs-5" id="ModalToggleLabelQ4">
                            "You've reached the end of the setup!"
                        </h1>

                        <button
                            type="button"
                            class="btn-close"
                            data-bs-dismiss="modal"
                            aria-label="Close"
                        ></button>
                    </div>
                    <div class="modal-body">
                        <div class="vstack gap-2 col-md-5 mx-auto">
                            <button
                                on:click=move |_| joined("/dashboard")
                                data-bs-dismiss="modal"
                                aria-label="Close"
                                type="button"
                                class="btn btn-outline-success"
                            >
                                Enter Dashboard
                            </button>
                            <button
                                on:click=move |_| joined("/tutorials")
                                data-bs-dismiss="modal"
                                aria-label="Close"
                                type="button"
                                class="btn btn-outline-success"
                            >
                                Enter Tutorials
                            </button>
                            <button
                                on:click=move |_| joined("/data")
                                data-bs-dismiss="modal"
                                aria-label="Close"
                                type="button"
                                class="btn btn-outline-success"
                            >
                                Enter Data Collection
                            </button>
                            <button
                                on:click=move |_| joined("/about")
                                data-bs-dismiss="modal"
                                aria-label="Close"
                                type="button"
                                class="btn btn-outline-success"
                            >
                                Enter Presentation
                            </button>
                        </div>
                        <img
                            class="zoomer mx-1 py-2 img-fluid w-25"
                            src="/public/images/interactive/rustacean-flat-gesture.png"
                        />
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn bg-primary-subtle"
                            data-bs-target="#ModalToggleQ3"
                            data-bs-toggle="modal"
                        >
                            <i class="mx-1 bi bi-arrow-left"></i>
                            Prev
                        </button>
                    </div>
                    <PageProgess percent="100" purpose="5th page!"/>
                </div>
            </div>
        </div>
    }
}
