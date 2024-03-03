use crate::stores::get_state;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{theme, use_theme, Theme};
use leptos_use::use_media_query;
use web_sys::HtmlVideoElement;

#[component]
/// This component sets the animated background depending on the light/dark theme
pub fn Background() -> impl IntoView {
    let theme: RwSignal<Theme> = use_theme();
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let is_new_user = get_state::<bool>("new-user31a85bde66b7d713c8743a12d427f3a5");

    // activates the background image
    create_effect(move |_| {
        // logging::log!("{:?}", is_dark_preferred_signal.get());

        if is_new_user.0.get() == false {
            let background_video = document()
                .get_element_by_id("background-video")
                .expect("a video element to exist")
                .dyn_into::<HtmlVideoElement>()
                .expect("Video to be slowed");
            background_video.set_playback_rate(0.75);
            // background_video.set_muted(true);
            // background_video.set_autoplay(true);
        }
    });

    view! {
        <Show when=move || is_new_user.0.get() == false fallback=move || view! { <span></span> }>
            <Show
                when=move || theme.get() == Theme::System
                fallback=move || {
                    view! {
                        <Show when=move || theme.get() == Theme::Dark fallback=LightVideo>
                            <DarkVideo/>
                        </Show>
                    }
                }
            >

                <Show when=move || is_dark_preferred_signal.get() == true fallback=LightVideo>
                    <DarkVideo/>
                </Show>
            </Show>
        </Show>
    }
}

#[component]
/// Represents the video for darkmoode
pub fn DarkVideo() -> impl IntoView {
    view! {
        <video
            id="background-video"
            autoplay=true
            loop=true
            muted="true"
            playsinline=true
            data-bs-theme="dark"
        >
            <source src="/public/videos/dark-tunnel_-_26475 (1080p).MP4" type="video/mp4"/>
        </video>
    }
}

#[component]
/// Represents the video for lightmode
pub fn LightVideo() -> impl IntoView {
    view! {
        <video
            id="background-video"
            autoplay=true
            loop=true
            muted="true"
            playsinline=true
            data-bs-theme="light"
        >
            <source src="/public/videos/light-tunnel_-_26475 (1080p).mp4" type="video/mp4"/>
        </video>
    }
}
