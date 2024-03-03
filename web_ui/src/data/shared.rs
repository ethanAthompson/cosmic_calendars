
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};

#[component]
/// This component contains an overview of supported celestial bodies.
pub fn Subtitle(name: &'static str) -> impl IntoView {
    view! { <div class="container py-4 fs-4 lead">{name}</div> }
}
