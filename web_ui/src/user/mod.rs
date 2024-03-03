/// This module contains signup code for the user.
pub mod signup;

use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::loading::Page as LoadingPage;
use crate::stores::get_state;

#[component]
/// This component renders a route's child when the user is not new
pub fn Page() -> impl IntoView {
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    view! {
        <Show when=move || is_new_user.0.get() fallback=move || view! { <LoadingPage/> }>
            <Outlet/>
        </Show>
    }
}
