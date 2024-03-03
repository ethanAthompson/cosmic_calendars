use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};

use super::planets::PlanetRoutes;
use super::moons::MoonRoutes;
use super::asteroids::AsteroidRoutes;
use super::comets::CometRoutes;
use super::exoplanets::ExoPlanetRoutes;
use super::Overview;

#[component(transparent)]
/// This transparent component contains all the supported data routes.
pub fn DataRoutes() -> impl IntoView  {
    view! {
        <Route path="/data" view=move || view! { <Outlet/> }>
            <PlanetRoutes/>
            <MoonRoutes/>
            <AsteroidRoutes/>
            <CometRoutes/>
            <ExoPlanetRoutes/>
            <Route path="" view=Overview/>
        </Route>
    }
}

