use leptos::{create_resource, create_signal, logging, ReadSignal, Resource, WriteSignal};

use crate::stores::sheets::Row;
use crate::stores::sheets::SheetData;
use crate::stores::states::FeedbackSearchFilterStates;
use crate::stores::states::FeedbackSearchStates;
use crate::utils::sheets::has_resolved;
use crate::utils::str::rustify_str;

/// A function that changes the class css that's rendered by a celestial body type...
pub fn colored_class(class_type: String) -> &'static str {
    match class_type.as_str() {
        "Planet" => "text-start badge rounded-pill text-bg-primary",
        "Moon" => "text-start badge rounded-pill text-bg-secondary",
        "Asteroid" => "text-start badge rounded-pill text-bg-warning",
        "Comet" => "text-start badge rounded-pill text-bg-info",
        "Exo Planet" => "text-start badge rounded-pill text-bg-success",
        _ => "text-end badge rounded-pill text-bg-dark",
    }
}

/// A function that changes the class css that's rendered by a celestial body type...
pub fn colored_class_opt(class_type: String) -> &'static str {
    match class_type.as_str() {
        "Planet" => "badge rounded-0 rounded-top w-100 text-bg-primary",
        "Moon" => "badge rounded-0 rounded-top text-bg-secondary",
        "Asteroid" => "badge rounded-0 rounded-top text-bg-warning",
        "Comet" => "badge rounded-0 rounded-top text-bg-info",
        "Exo Planet" => "badge rounded-0 rounded-top text-bg-success",
        _ => " badge rounded-0 rounded-top text-bg-dark",
    }
}

/// A function that changes the class css that's rendered by a celestial body type...
pub fn colored_class_btn(class_type: String) -> &'static str {
    // logging::log!("{:?}", class_type);

    match class_type.as_str() {
        "Planet" => "list-group-item btn btn-primary btn-lg rounded-0 rounded-bottom",
        "Moon" => "list-group-item btn btn-secondary btn-lg rounded-0 rounded-bottom",
        "Asteroid" => "list-group-item btn btn-warning btn-lg rounded-0 rounded-bottom",
        "Comet" => "list-group-item btn btn-info btn-lg rounded-0 rounded-bottom",
        "Exo Planet" => "list-group-item btn btn-success btn-lg rounded-0 rounded-bottom",
        _ => " list-group-item btn btn-dark btn-lg rounded-0 rounded-bottom",
    }
}

/// This function matches the feedback types and formats the pill colors
pub fn direct_feedback_types(kind: String) -> String {
    // logging::log!("{}", kind);
    match kind.as_ref() {
        "Comments" => "badge rounded-pill text-bg-primary",
        "Questions" => "badge rounded-pill text-bg-warning",
        "Bug Reports" => "badge rounded-pill text-bg-success",
        "Feature Request" => "badge rounded-pill text-bg-info",
        _ => "badge rounded-pill text-bg-secondary",
    }
    .to_string()
}

/// This function matches the response and returns an src link
pub fn colored_class_badge(pfp: String) -> String {
    /*
       Planet => primary,
       Moon => secondary,
       Asteroid => warning,
       Comet => info,
       Exoplanet => success
    */
    match pfp.as_ref() {
        // Planets => bg-primary
        "Earth" | "Mars" | "Mercury" | "Neptune" | "Jupiter" | "Saturn" | "Pluto" | "Venus" | "Uranus" => {
            "bg-primary"
        }

        // Moons => bg-secondary
        "Luna" | "Europa" | "Io" | "Ganymede" | "Titan" => "bg-secondary",

        // Asteroids => bg-warning
        "Ceres" | "4 Vesta" | "2 Pallas" | "10 Hygiea" | "52 Europa" | "511 Davida" | "3 Juno"
        | "433 Eros" | "6 Hebe" => "bg-asteroid",

        // Comets => bg-info
        "Halley" | "Hale-Bopp" => "bg-info",

        // Exoplanet => bg-success
        "Kepler-442b" | "Kepler-22b " | "Kepler-186f" => "bg-success",

        _ => "bg-dark",
    }
    .to_string()
}
