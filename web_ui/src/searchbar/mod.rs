/// Contains a format for each item in the searchbar
pub mod item;

/// Contains a wrapper for [`item`]
pub mod card;

/// Contains the data shown in each item
pub mod data;

/// Contains the filter that you use.
pub mod filter;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_media_query;
use serde::{Serialize, Deserialize};

#[component]
/// This component represents the searchbar logo that you see.
pub fn Logo() -> impl IntoView {
    let is_large_screen = use_media_query("(min-width: 570px)");

    view! {
        <Show
            when=move || is_large_screen.get() == true
            fallback=move || view! { <i class="bi bi-search mx-4"></i> }
        >
            <i class="bi bi-search px-2 "></i>
            Search
        </Show>
    }
}
