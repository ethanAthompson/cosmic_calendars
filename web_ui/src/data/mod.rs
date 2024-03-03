/// This component contains moons routes
pub mod moons;

/// This component contains planets routes
pub mod planets;

/// This component contains asteroids routes
pub mod asteroids;

/// This component contains exoplanets routes
pub mod exoplanets;

/// This component contains comet routes
pub mod comets;

/// This module contains the common format for each celestial body
pub mod card;

/// This module contains the data page's routes
pub mod routes;

/// This module contains shared data page code.
pub mod shared;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
/*

The planets' picture will be sticky as you'll be able to constantly see it as you so other data

https://www.w3schools.com/howto/howto_css_image_sticky.asp

*/

#[component]
/// This component is the data page you see.
pub fn Page() -> impl IntoView {
    view! {
        <main class="container-fluid py-5 ">
            <p class="text-center display-5">View Celestial Bodies</p>
            <div class="container py-2">
                <Outlet/>
            </div>
        </main>
    }
}

#[component]
/// This component contains an overview of supported celestial bodies.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Celestial Bodies"/>

        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3">
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="planets">
                    Planets
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="moons">
                    Moons
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="asteroids"
                >
                    Asteroids
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="comets">
                    Comets
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="exo-planets"
                >
                    Exo-Planets
                </A>
                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
