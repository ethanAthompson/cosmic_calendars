use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

/* 

Make it where, if the username search is empty, then you cannot launch.

*/

#[component]
pub fn SignUp() -> impl IntoView {
    view! {
        <form>
            <div class="mb-3">
                <label for="celestialInputUserName" class="form-label">
                    Username
                </label>
                <input
                    type="email"
                    class="form-control"
                    id="celestialInputUserName"
                    aria-describedby="celestialemailHelp"
                />
                <div id="emailHelp" class="form-text">
                    "Your name is collected for personalization"
                </div>
            </div>
        </form>
    }
}