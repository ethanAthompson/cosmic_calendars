use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::{use_theme, ThemeProvider};
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn};

#[component]
/// This component represents each a table item within the [`supported bodies`] table
pub fn TableItem(bg: &'static str, item: &'static str, class: &'static str) -> impl IntoView {
    let correct_class = match class {
        "planet" => "planets",
        "moon" => "moons",
        "asteroid" => "asteroids",
        "exo-planet" => "exo-planets",
        "comet" => "comets",
        _ => "unknown",
    };

    view! {
        <td class=bg>
            <A
                class=" btn btn-lg w-100 fs-4"
                href=format!("data/{}/{}", correct_class, item.to_lowercase())
            >
                {item}
            </A>
        </td>
    }
}

#[component]
/// This is a table of all supported and un-supported planets, moons, asteroids, comets, and exo-planets
pub fn Table() -> impl IntoView {
    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col" class="fs-5">
                        #
                    </th>
                    <th scope="col" class="fs-5">
                        Planets
                    </th>
                    <th scope="col" class="fs-5">
                        Moons
                    </th>
                    <th scope="col" class="fs-5">
                        Asteroids
                    </th>
                    <th scope="col" class="fs-5">
                        Exo Planets
                    </th>
                    <th scope="col" class="fs-5">
                        Comets
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row">1</th>
                    <TableItem class="planet" bg="bg-success" item="Earth"/>
                    <TableItem class="moon" bg="bg-success" item="Europa"/>
                    <TableItem class="asteroid" bg="bg-success" item="Ceres"/>
                    <TableItem class="exo-planet" bg="bg-danger" item="Kepler-62b"/>
                    <TableItem class="comet" bg="bg-danger" item="Hailey"/>
                </tr>
                <tr>
                    <th scope="row">2</th>
                    <TableItem class="planet" bg="bg-success" item="Jupiter"/>
                    <TableItem class="moon" bg="bg-success" item="Ganymede"/>
                    <TableItem class="asteroid" bg="bg-danger" item="511-Davida"/>
                    <TableItem class="exo-planet" bg="bg-danger" item="Kepler-186f"/>
                    <TableItem class="comet" bg="bg-danger" item="Hale-Bopp"/>
                </tr>
                <tr>
                    <th scope="row">3</th>
                    <TableItem class="planet" bg="bg-success" item="Mars"/>
                    <TableItem class="moon" bg="bg-success" item="Io"/>
                    <TableItem class="asteroid" bg="bg-success" item="433-Eros"/>
                    <TableItem class="exo-planet" bg="bg-danger" item="Kepler-62e"/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
                <tr>
                    <th scope="row">4</th>
                    <TableItem class="planet" bg="bg-success" item="Mercury"/>
                    <TableItem class="moon" bg="bg-success" item="Luna"/>
                    <TableItem class="asteroid" bg="bg-danger" item="52-Europa"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
                <tr>
                    <th scope="row">5</th>
                    <TableItem class="planet" bg="bg-success" item="Neptune"/>
                    <TableItem class="moon" bg="bg-success" item="Titan"/>
                    <TableItem class="asteroid" bg="bg-success" item="6-Hebe"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
                <tr>
                    <th scope="row">6</th>
                    <TableItem class="planet" bg="bg-success" item="Pluto"/>
                    <TableItem class="moon" bg="" item=""/>
                    <TableItem class="asteroid" bg="bg-danger" item="10-Hygiea"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
                <tr>
                    <th scope="row">7</th>
                    <TableItem class="planet" bg="bg-success" item="Saturn"/>
                    <TableItem class="moon" bg="" item=""/>
                    <TableItem class="asteroid" bg="bg-danger" item="3-Juno"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>

                </tr>
                <tr>
                    <th scope="row">8</th>
                    <TableItem class="planet" bg="bg-success" item="Uranus"/>
                    <TableItem class="moon" bg="" item=""/>
                    <TableItem class="asteroid" bg="bg-danger" item="2-Pallas"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
                <tr>
                    <th scope="row">9</th>
                    <TableItem class="planet" bg="bg-success" item="Venus"/>
                    <TableItem class="moon" bg="" item=""/>
                    <TableItem class="asteroid" bg="bg-success" item="4-Vesta"/>
                    <TableItem class="exo-planet" bg="" item=""/>
                    <TableItem class="comet" bg="" item=""/>
                </tr>
            </tbody>
        </table>
    }
}
