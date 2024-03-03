use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
use super::card::info::Page;

#[component(transparent)]
/// This transparent component contains all the supported exo-planet routes.
pub fn ExoPlanetRoutes() -> impl IntoView  {
    view! {
        <Route path="exo-planets" view=move || view! { <Outlet/> }>
            <Route
                path="kepler-22b"
                view=move || {
                    view! { <Page name="kepler-22b"/> }
                }
            />

            <Route
                path="kepler-186f"
                view=move || {
                    view! { <Page name="kepler-186f"/> }
                }
            />

            <Route
                path="kepler-442b"
                view=move || {
                    view! { <Page name="kepler-442b"/> }
                }
            />

            <Route path="" view=Overview/>
        </Route>
    }
}

#[component]
/// This component contains an overview of exo-planets supported.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Exoplanets"/>
        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3">
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="kepler-22b"
                >
                    Kepler 22b
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="kepler-186f"
                >
                    Kepler 186f
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="kepler-442b"
                >
                    Kepler 442b
                </A>

                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
