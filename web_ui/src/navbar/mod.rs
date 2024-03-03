// Contains navbar items (small or large)
pub mod items;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, Theme, ThemeProvider};
use leptos_use::use_media_query;
 

use self::items::Large as LargeNavbarItems;
use self::items::Small as SmallNavbarItems;
use crate::stores::states::CosmicTimeZoneState;
use crate::stores::get_state;
use crate::searchbar::Logo as Searchbar;
use crate::searchbar::card::Page as SearchContent;
use crate::theme::switch::Button as ToggleButton;
use crate::time::LocalTimer;

#[component]
/// This page represents the navbar that you see on the screen
pub fn Page() -> impl IntoView {
    let state = get_state::<CosmicTimeZoneState>("home-timezone-name");
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");
    let current_theme: RwSignal<Theme> = use_theme();
    let is_large_screen = use_media_query("(min-width: 970px)");

    view! {
        <Show when=move || is_new_user.0.get()>

            <header class="navbar py-2">
                <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
                    <div class="w-100 hstack container-fluid gap-2 d-flex justify-items-between align-items-center">
                        <A class="navbar-brand" href="">
                            <img
                                class="spinner"
                                src="/public/favicon_io/apple-touch-icon.png"
                                width=50
                                height=50
                            />
                        </A>
                        <Show
                            when=move || is_large_screen.get() == true
                            fallback=move || view! { <SmallNavbarItems/> }
                        >
                            <LargeNavbarItems/>
                        </Show>
                        <div class="vr"></div>
                        <a
                            class="focus-ring focus-ring-warning text-decoration-none rounded-2 ms-auto container-fluid navbar-item btn border btn-lg d-flex w-100 "
                            data-bs-toggle="modal"
                            href="#"
                            data-bs-target="#SearchModal"
                        >
                            <Searchbar/>
                        </a>
                        <SearchContent/>
                        <div class="vr"></div>
                        <div class="container-sm d-flex navbar-item  w-25 justify-content-end align-items-end">
                            <ToggleButton theme=current_theme/>
                        </div>
                    </div>
                </nav>
            </header>

            <div class="py-3 px-2 text-center display-6 ">
                <LocalTimer/>
            </div>
            <nav class="container-xxl bd-gutter flex-wrap flex-lg-nowrap">
                <hr class="py-1"/>
            </nav>
        </Show>
    }
}