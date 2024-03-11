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
                path="511-davida"
                view=move || {
                    view! { <Page name="511-davida"/> }
                }
            />

            <Route
                path="433-eros"
                view=move || {
                    view! { <Page name="433-eros"/> }
                }
            />

            <Route
                path="52-europa"
                view=move || {
                    view! { <Page name="52-europa"/> }
                }
            />

            <Route
                path="6-hebe"
                view=move || {
                    view! { <Page name="6-hebe"/> }
                }
            />

            <Route
                path="10-hygiea"
                view=move || {
                    view! { <Page name="10-hygiea"/> }
                }
            />

            <Route
                path="3-juno"
                view=move || {
                    view! { <Page name="3-juno"/> }
                }
            />

            <Route
                path="2-pallas"
                view=move || {
                    view! { <Page name="2-pallas"/> }
                }
            />

            <Route
                path="4-vesta"
                view=move || {
                    view! { <Page name="4-vesta"/> }
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
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="511-davida"
                >
                    Davida
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="433-eros">
                    Eros
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="52-europa"
                >
                    Europa
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="6-hebe">
                    Hebe
                </A>
                <A
                    class="btn btn-lg btn-outline-warning list-group-item display-5"
                    href="10-hygiea"
                >
                    Hygiea
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="3-juno">
                    Juno
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="2-pallas">
                    Pallas
                </A>
                <A class="btn btn-lg btn-outline-warning list-group-item display-5" href="4-vesta">
                    Vesta
                </A>
                <A href="../" class="btn btn-lg btn-outline-danger">
                    <span>Go back</span>
                </A>
            </ul>
        </nav>
    }
}
