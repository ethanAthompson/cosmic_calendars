use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, Theme, ThemeProvider};
use leptos_use::use_media_query;

#[component]
/// This component represents the larger screen's navbar items
pub fn Large() -> impl IntoView {
    view! {
        <div class="container-fluid">

            <A class="navbar-brand" href="dashboard">
                Dashboard
            </A>
            <A class="navbar-brand" href="data">
                Data
            </A>
            <A class="navbar-brand" href="tutorials">
                Tutorials
            </A>
            <A class="navbar-brand" href="feedback">
                Feedback
            </A>
        </div>
    }
}

#[component]
/// This component represents the smaller screen's navbar items
pub fn Small() -> impl IntoView {
    view! {
        <div class="navbar-item dropdown mx-0">
            <button
                class="btn dropdown-toggle"
                type="button"
                data-bs-toggle="dropdown"
                aria-expanded="false"
            >
                Links
            </button>
            <ul class="dropdown-menu container-fluid">
                <li class="dropdown-item">
                    <A class="navbar-brand " href="dashboard">
                        Dashboard
                    </A>
                </li>
                <li class="dropdown-item">
                    <A class="navbar-brand " href="data">
                        Data
                    </A>
                </li>
                <li class="dropdown-item">
                    <A class="navbar-brand " href="tutorials">
                        Tutorials
                    </A>
                </li>
                <li>
                    <hr class="dropdown-divider"/>
                </li>
                <li class="dropdown-item">
                    <A class="navbar-brand " href="feedback">
                        Feedback
                    </A>
                </li>

            </ul>
        </div>
    }
}
