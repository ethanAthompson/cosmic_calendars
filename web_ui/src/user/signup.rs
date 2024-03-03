use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::utils::IS_IOS;
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use crate::stores::get_state;
use crate::stores::states::EarthCalendarState;
use crate::stores::states::{CosmicTimeZoneState, UserNameState};
use crate::utils::calendar::SupportedCalendars;
use crate::utils::celestial::direct_tzname_image;

/*

Make it where, if the username search is empty, then you cannot launch.

*/

#[component]
/// This compponent is needed to increase user personalization.
pub fn SignUp() -> impl IntoView {
    let username = create_rw_signal("".to_string());
    let state = get_state::<UserNameState>("home-user-name");
    let state_ref = create_rw_signal(state.0.get().value);

    let update_username = move |ev| {
        state.1.set(UserNameState {value: event_target_value(&ev)});
        username.set(state.0.get().value);

        // logging::log!("{:?}", state.0.get().value);
    };

    view! {
        <div class="container-fluid w-auto py-2">
            <div class="input-group mb-3">
                <span class="input-group-text">
                    <i class="bi bi-person fs-5"></i>
                </span>
                <div class="form-floating">
                    <input
                        type="text"
                        class="form-control"
                        id="floatingInputGroup1UserName"
                        placeholder="Username"
                        title="Please Enter a username"
                        on:input=update_username
                    />
                    <label for="floatingInputGroup1UserName">Username</label>
                </div>
            </div>
            <div class="lead py-2 container-fluid">
                <Show
                    when=move || username.get().is_empty()
                    fallback=move || {
                        view! {
                            <div class="vstack gap-2">
                                <span>Welcome to Cosmic Calendars!</span>
                                <span class="py-2 alert alert-success">{username}</span>
                            </div>
                        }
                    }
                >

                    <div class="alert alert-danger">Please enter a username</div>
                </Show>
            </div>
        </div>
    }
}
