#![allow(warnings)]
use leptos::{view, mount_to_body, logging};
use app::App;

/* Directories*/
mod dashboard;
mod data; 
mod error; 
mod feedback; 
mod home; 
mod loading; 
mod navbar; 
mod stores; 
mod tutorials; 
mod utils; 
mod time;
mod searchbar;
mod theme;
mod user;

/* Files */
mod app;
mod env;
mod context;


pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App/> }
    });
}
