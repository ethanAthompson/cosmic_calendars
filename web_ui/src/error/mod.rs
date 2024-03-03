use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::ThemeProvider;
use leptos_use::core::Position;

#[component]
/// This component represents the error page or [not found page]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class="bg-danger-subtle">
            <div class="container-fluid p-5">
                <figure class="text-center">
                    <blockquote class="blockquote">
                        <p>
                            The page <strong>{unknown}</strong> is not available.
                            <i class="mx-2 bi bi-emoji-frown"></i>
                        </p>
                    </blockquote>
                    <figcaption class="blockquote-footer py-2 d-flex text-center justify-content-center align-items-center">
                        <cite>Navigate to</cite>
                        <A class="pulse btn btn-sm fst-italic" href="/">
                            Home
                        </A>
                    </figcaption>
                </figure>
                <img
                    class="img-fluid rounded-5 main-background-image"
                    src="public/images/interactive/juan-rumimpunu-nLXOatvTaLo-unsplash.jpg"
                />
            </div>
        </main>
    }
}
