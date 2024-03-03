/// This module contains the table for the `[supported data]` button
pub mod supported;

/// This module contains the home page's greetings and welcomings that you see.
pub mod welcome;

/// The converter buttons you see..
pub mod converters;



use leptos::html::{time, Input};
use leptos::svg::filter;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use serde_json::to_string;
use serde_json::value::Value;
use serde_with::rust;
use web_sys::{Event, MouseEvent, Node, SubmitEvent};

use crate::stores::states::FeedbackSearchFilterStates;
use self::converters::Content as ConverterContent;
use self::welcome::Content as WelcomeContent;

use crate::stores::get_state;

#[component]
/// This component is the home page that you see.
pub fn Page() -> impl IntoView {
    view! {
        <main class="bg-transparent container-fluid py-5 bd-navbar">
            <WelcomeContent/>
            <div class="bg-transparent container-xxl bd-gutter flex-wrap flex-lg-nowrap py-5">
                <ConverterContent/>
            </div>
        </main>
    }
}
