use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use crate::data::shared::Subtitle;
use super::card::info::Page;

#[component(transparent)]
/// This transparent component contains all the supported asteroid routes.
pub fn AsteroidRoutes() -> impl IntoView {
    view! {
        <Route path="asteroids" view=move || view! { <Outlet/> }>
            <Route
                path="ceres"
                view=move || {
                    view! { <Page name="ceres"/> }
                }
            />

            <Route
                path="davida"
                view=move || {
                    view! { <Page name="davida"/> }
                }
            />

            <Route
                path="eros"
                view=move || {
                    view! { <Page name="eros"/> }
                }
            />

            <Route
                path="europa"
                view=move || {
                    view! { <Page name="europa"/> }
                }
            />

            <Route
                path="hebe"
                view=move || {
                    view! { <Page name="hebe"/> }
                }
            />

            <Route
                path="hygiea"
                view=move || {
                    view! { <Page name="hygiea"/> }
                }
            />

            <Route
                path="juno"
                view=move || {
                    view! { <Page name="juno"/> }
                }
            />

            <Route
                path="pallas"
                view=move || {
                    view! { <Page name="pallas"/> }
                }
            />

            <Route
                path="vesta"
                view=move || {
                    view! { <Page name="vesta"/> }
                }
            />

            <Route path="" view=Overview/>
        </Route>
    }
}


#[component]
/// This component contains an overview of asteroids supported.
pub fn Overview() -> impl IntoView {
    view! {
        <Subtitle name="List of Asteroids"/>
        <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
            <ul class="list-group grid gap-3">
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="ceres">
                    Ceres
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="davida">
                    Davida
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="eros">
                    Eros
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="europa">
                    Europa
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="hebe">
                    Hebe
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="hygiea">
                    Hygiea
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="juno">
                    Juno
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="pallas">
                    Pallas
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="vesta">
                    Vesta
                </A>
                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
