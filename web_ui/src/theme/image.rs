use crate::stores::get_state;
use leptos::*;
use leptos_theme::{theme, use_theme, Theme};
use leptos_use::use_media_query;

#[component]
/// This component sets the background given light/dark mode
pub fn Background() -> impl IntoView {
    let theme: RwSignal<Theme> = use_theme();
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    view! {
        <Show when=move || is_new_user.0.get() == true fallback=move || view! { <span></span> }>
            <Show
                when=move || theme.get() == Theme::System
                fallback=move || {
                    view! {
                        <Show when=move || theme.get() == Theme::Dark fallback=LightImage>
                            <DarkImage/>
                        </Show>
                    }
                }
            >

                <Show when=move || is_dark_preferred_signal.get() == true fallback=LightImage>
                    <DarkImage/>
                </Show>
            </Show>
        </Show>
    }
}

#[component]
/// This component sets the background given light/dark mode
pub fn AdjustThemedImage(light: String, dark: String) -> impl IntoView {
    let theme: RwSignal<Theme> = use_theme();
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    let light_env = create_rw_signal(light.clone());
    let dark_env = create_rw_signal(dark.clone());

    view! {
        <Show when=move || is_new_user.0.get() == true fallback=move || view! { <span></span> }>
            <Show
                when=move || theme.get() == Theme::System
                fallback=move || {
                    view! {
                        <Show
                            when=move || theme.get() == Theme::Dark
                            fallback=move || view! { <NormalImage url=light_env.get()/> }
                        >
                            <NormalImage url=dark_env.get()/>
                        </Show>
                    }
                }
            >

                <Show
                    when=move || is_dark_preferred_signal.get() == true
                    fallback=move || view! { <NormalImage url=light_env.get()/> }
                >
                    <NormalImage url=dark_env.get()/>
                </Show>
            </Show>
        </Show>
    }
}

#[component]
/// Represents the image for any implementation
pub fn NormalImage(url: String) -> impl IntoView {
    view! { <img data-bs-theme="dark" id=format!("dark-{}-image", url) class="img-fluid " src=url/> }
}

#[component]
/// Represents the image for darkmoode
pub fn DarkImage() -> impl IntoView {
    view! {
        <img
            data-bs-theme="dark"
            id="dark-theme-image"
            class="img-fluid main-background-image"
            src="/public/images/background/2dark-ver-pawel-czerwinski-F0yDf-GBjMQ-unsplash.jpg"
        />
    }
}

#[component]
/// Represents the image for lightmode
pub fn LightImage() -> impl IntoView {
    view! {
        <img
            data-bs-theme="light"
            id="light-theme-image"
            class="img-fluid main-background-image"
            src="/public/images/background/pawel-czerwinski-F0yDf-GBjMQ-unsplash.jpg"
        />
    }
}
