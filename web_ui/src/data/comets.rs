use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
use super::card::info::Page;

#[component(transparent)]
/// This transparent component contains all the supported comet routes.
pub fn CometRoutes() -> impl IntoView  {
    view! {
        <Route path="comets" view=move || view! { <Outlet/> }>
            <Route
                path="hailey"
                view=move || {
                    view! { <Page name="hailey"/> }
                }
            />

            <Route
                path="hale-bopp"
                view=move || {
                    view! { <Page name="hale-bopp"/> }
                }
            />

            <Route path="" view=Overview/>
        </Route>
    }
}

#[component]
/// This component contains an overview of comets supported.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Comets"/>
        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3">
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="hailey">
                    Hailey
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="hale-bopp"
                >
                    Hale Bopp
                </A>
                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
