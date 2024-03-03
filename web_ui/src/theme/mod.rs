/// This module contains a background video for new_user given the set theme.
pub mod animated;

/// This module contains a background image given the set theme.
pub mod image;

/// This module contains a theme switch for the app.
pub mod switch;

use leptos::*;
use leptos_theme::{theme, use_theme, Theme};
use leptos_use::use_media_query;

/// This is Bootstrap class theme
const BSTHEME: &'static str = "data-bs-theme";


#[component]
/// This component is a wrapper over the bootstrap color theming.
/// 
/// Inspired by *leptos_theme*
pub fn ThemeProvider( children: Children) -> impl IntoView {
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let prefers_dark = move || if is_dark_preferred_signal.get() { true } else { false };

    create_effect(move |_| {
        let current_theme = use_theme();
        update_bs_for_theme(current_theme.get(), prefers_dark());
    });
    view! { {children()} }
}

/// This function updates the theme according to bootstrap color mode
pub fn update_bs_for_theme(theme: Theme, prefers_dark: bool ) {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_element = document.document_element().unwrap();

    match theme {
        Theme::Light => {
            html_element.set_attribute(BSTHEME, "light").unwrap();
        }
        Theme::Dark => {
            html_element.set_attribute(BSTHEME, "dark").unwrap();
        }
        Theme::System => match prefers_dark {
            true => html_element.set_attribute(BSTHEME, "dark").unwrap(),
            false => html_element.set_attribute(BSTHEME, "light").unwrap(),
        },
    }
}