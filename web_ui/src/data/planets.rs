use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
use super::card::info::Page;

#[component(transparent)]
/// This transparent component contains all the supported planet routes.
pub fn PlanetRoutes() -> impl IntoView  {
    view! {
        <Route path="planets" view=move || view! { <Outlet/> }>
            <Route
                path="earth"
                view=move || {
                    view! { <Page name="earth"/> }
                }
            />

            <Route
                path="jupiter"
                view=move || {
                    view! { <Page name="jupiter"/> }
                }
            />

            <Route
                path="mars"
                view=move || {
                    view! { <Page name="mars"/> }
                }
            />

            <Route
                path="mercury"
                view=move || {
                    view! { <Page name="mercury"/> }
                }
            />

            <Route
                path="neptune"
                view=move || {
                    view! { <Page name="neptune"/> }
                }
            />

            <Route
                path="pluto"
                view=move || {
                    view! { <Page name="pluto"/> }
                }
            />

            <Route
                path="saturn"
                view=move || {
                    view! { <Page name="saturn"/> }
                }
            />

            <Route
                path="uranus"
                view=move || {
                    view! { <Page name="uranus"/> }
                }
            />

            <Route
                path="venus"
                view=move || {
                    view! { <Page name="venus"/> }
                }
            />

            <Route path="" view=Overview/>
        </Route>
    }
}

#[component]
/// This component contains an overview of planets supported.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Planets"/>
        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3 ">
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="earth">
                    Earth
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="jupiter">
                    Jupiter
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="mars">
                    Mars
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="mercury">
                    Mercury
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="neptune">
                    Neptune
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="pluto">
                    Pluto
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="saturn">
                    Saturn
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="uranus">
                    Uranus
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="venus">
                    Venus
                </A>

                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
