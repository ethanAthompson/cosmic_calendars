use leptos::*;
use leptos_theme::{theme, use_theme, Theme};
use leptos_use::use_media_query;
use crate::stores::get_state;

#[component]
/// This is a button that utilizes the leptos theme library to toggle the theme switch
pub fn Button(theme: RwSignal<leptos_theme::Theme>) -> impl IntoView {
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    view! {
        <Show
            when=move || theme.get() == Theme::System
            fallback=move || {
                view! {
                    <Show when=move || theme.get() == Theme::Dark fallback=LightSwitch>
                        <DarkSwitch/>
                    </Show>
                }
            }
        >

            <Show when=move || is_dark_preferred_signal.get() == true fallback=LightSwitch>
                <DarkSwitch/>
            </Show>
        </Show>
    }
}


#[component]
/// Represents the switch for darkmoode
pub fn DarkSwitch() -> impl IntoView {
    let theme: RwSignal<Theme> = use_theme();

    view! {
        <i
            data-bs-theme="dark"
            class="bi bi-toggle-on pointer display-5"
            on:click=move |_| { theme.set(Theme::Light) }
        ></i>
    }
}

#[component]
/// Represents the switch for lightmode
pub fn LightSwitch() -> impl IntoView {
    let theme: RwSignal<Theme> = use_theme();

    view! {
        <i
            data-bs-theme="light"
            on:click=move |_| { theme.set(Theme::Dark) }
            class="bi bi-toggle-off display-5"
        ></i>
    }
}