use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
use super::card::info::Page;

#[component(transparent)]
/// This transparent component contains all the supported moon routes.
pub fn MoonRoutes() -> impl IntoView {
    view! {
        <Route path="moons" view=move || view! { <Outlet/> }>
            <Route
                path="europa"
                view=move || {
                    view! { <Page name="europa"/> }
                }
            />

            <Route
                path="ganymede"
                view=move || {
                    view! { <Page name="ganymede"/> }
                }
            />

            <Route
                path="io"
                view=move || {
                    view! { <Page name="io"/> }
                }
            />

            <Route
                path="luna"
                view=move || {
                    view! { <Page name="luna"/> }
                }
            />

            <Route
                path="titan"
                view=move || {
                    view! { <Page name="titan"/> }
                }
            />

            <Route path="" view=Overview/>
        </Route>
    }
}


#[component]
/// This component contains an overview of moons supported.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Moons"/>
        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3">
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="europa">
                    Europa
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="ganymede">
                    Ganymede
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="io">
                    Io
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="luna">
                    Luna
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="titan">
                    Titan
                </A>
                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
