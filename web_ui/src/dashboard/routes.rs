use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use web_sys::{Event, HtmlAnchorElement, MouseEvent};

use super::Page;
use super::greeting::Page as IntroduceUser;
use super::celestial::Page as TimezoneDatabasePage;
use super::settings::Page as SettingsPage;
use crate::dashboard::date::Page as DateConverter;
use crate::dashboard::time::Page as TimeConverter;

#[component(transparent)]
pub fn DashboardRoutes() -> impl IntoView {
    view! {
        <Route path="/dashboard" view=Page>
            <Route path="celestial-tzs" view=TimezoneDatabasePage/>
            <Route path="celestial-date" view=DateConverter/>
            <Route path="celestial-time" view=TimeConverter/>
            <Route path="celestial-settings" view=SettingsPage/>
            <Route path="" view=IntroduceUser/>
        </Route>
    }
}