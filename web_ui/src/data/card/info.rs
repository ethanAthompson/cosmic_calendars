use crate::data::card::all::data_abstraction;
use crate::theme::image::AdjustThemedImage;
use crate::time::martian::set_mars_tz;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use osm_db::orbit::Perihelion;

#[component]
/// This component contains an overview of moons supported.
pub fn Page(name: &'static str) -> impl IntoView {
    let obe = create_rw_signal(-1.0);
    let orbital_period = create_rw_signal(-1.0);
    let rotational_period = create_rw_signal(-1.0);
    let perihelion = create_rw_signal(Perihelion {
        month: (0.0, 0.0),
        ls: (0.0, 0.0),
        perihelion: 0.0,
    });
    let semimajor = create_rw_signal(-1.0);
    let semiminor = create_rw_signal(-1.0);
    let mean_motion = create_rw_signal(-1.0);
    let orbit_type = create_rw_signal("".to_string());
    let map_src = create_rw_signal("".to_string());

    data_abstraction(
        name,
        obe,
        orbital_period,
        rotational_period,
        perihelion,
        semimajor,
        semiminor,
        mean_motion,
        orbit_type,
        map_src
    );

    view! {
        <div class="bg-transparent container-xxl bd-gutter flex-wrap flex-lg-nowrap py-5">
            <div class="card text-center">
                <div class="card-header">
                    <span class="fs-5 fw-bold">
                        Featuring
                        {format!("{}{}", (&name[..1].to_string()).to_uppercase(), &name[1..])} "'s"
                        Data
                    </span>
                </div>
                <div class="card-body">
                    <div class="container-fluid gap-3">
                        <div class="container-fluid">
                            <p class="fs-2 lead p-2">{map_src}</p>
                        </div>
                        <hr class="py-2"/>
                        <div class="container-fluid">
                            <ol class="list-group w-100 container-fluid ">
                                <div>
                                    <p class="lead">
                                        Data provided by
                                        <a
                                            href="https://github.com/ethanAthompson/cosmic_calendars/tree/main/osm_db"
                                            target="_blank"
                                            class="text-decoration-none"
                                        >
                                            OSM Database Library
                                            <i class="bi bi-box-arrow-in-left"></i>
                                        </a>
                                    </p>
                                    <p class="p-2 badge rounded-pill text-bg-danger">
                                        -1 means (WIP)
                                    </p>
                                </div>
                                <li class="list-group-item d-flex justify-content-between align-items-start">
                                    <div class="vstack">
                                        <div class="fw-bold me-auto">Orbital Eccentricity</div>
                                        <span class="px-2 text-start me-auto">{obe}</span>
                                    </div>
                                    <span class="badge bg-primary">{orbit_type}</span>
                                </li>
                                <li class="list-group-item d-flex justify-content-between align-items-start">
                                    <div class="vstack">
                                        <div class="fw-bold me-auto">Orbital Period</div>
                                        <span class="px-2 text-start">{orbital_period}</span>
                                    </div>
                                    <span class="badge bg-primary rounded-pill">Days</span>
                                </li>
                                <li class="list-group-item d-flex justify-content-between align-items-start">
                                    <div class="vstack">
                                        <div class="fw-bold me-auto">Rotational Period</div>
                                        <span class="px-2 text-start">{rotational_period}</span>
                                    </div>
                                    <span class="badge bg-primary rounded-pill">Seconds</span>
                                </li>
                                <li class="list-group-item d-flex justify-content-between align-items-start">
                                    <div class="vstack">
                                        <div class="fw-bold me-auto">Semi Major</div>
                                        <span class="px-2 text-start">{semimajor}</span>
                                    </div>
                                    <span class="badge bg-primary rounded-pill">Axis</span>
                                </li>
                                <li class="list-group-item d-flex justify-content-between align-items-start">
                                    <div class="vstack">
                                        <div class="fw-bold me-auto">Semi Minor</div>
                                        <span class="px-2 text-start">{semiminor}</span>
                                    </div>
                                    <span class="badge bg-primary rounded-pill">Axis</span>
                                </li>
                            </ol>
                        </div>
                    </div>
                </div>
                <div class="card-footer text-body-secondary">
                    <A href="../" class="btn btn-lg btn-outline-danger container-fluid">
                        <span>Go back</span>
                    </A>
                </div>
            </div>
        </div>
    }
}
