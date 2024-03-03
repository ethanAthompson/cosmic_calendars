use crate::theme::image::AdjustThemedImage;
use crate::time::martian::set_mars_tz;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};
use rust_solar::kepler::Body;
use rust_solar::orbit::{self, Perihelion};

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
    let dark_topology_map = create_rw_signal("".to_string());
    let light_topology_map = create_rw_signal("".to_string());
    let eccentric_anomaly_data: RwSignal<Vec<f64>> = create_rw_signal(Vec::new());

    match name {
        "mars" => {
            let sol = set_mars_tz().date.day;
            // let mars_anomaly =
            obe.set(rust_solar::planets::mars::Mars.orbital_eccentricity());
            orbital_period.set(rust_solar::planets::mars::Mars.orbital_period());
            rotational_period.set(rust_solar::planets::mars::Mars.rotational_period());
            perihelion.set(rust_solar::planets::mars::Mars.perihelion());
            semimajor.set(rust_solar::planets::mars::Mars.semimajor());
            semiminor.set(rust_solar::planets::mars::Mars.semiminor());
            mean_motion.set(rust_solar::planets::mars::Mars.mean_motion(sol));
            orbit_type.set(format!(
                "{:?}",
                rust_solar::orbit::Type::shape(&orbit::Type::Unknown, obe.get())
            ));
            dark_topology_map.set("/public/images/solar/dark-mars-tz.jpeg".to_string());
            light_topology_map.set("/public/images/solar/light-mars-tz.png".to_string());
            // eccentric_anomaly_data.set();
        }
        _ => {
            //  todo!()
        }
    }

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
                            <AdjustThemedImage
                                light=light_topology_map.get()
                                dark=dark_topology_map.get()
                            />
                        </div>
                        <hr class="py-2"/>
                        <div class="container-fluid">
                            <ol class="list-group w-100 container-fluid ">
                                <div>
                                    <p class="lead">
                                        Data provided by
                                        <a
                                            href="https://crates.io/crates/rust_solar"
                                            target="_blank"
                                            class="text-decoration-none"
                                        >
                                            Rust Solar Library
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
