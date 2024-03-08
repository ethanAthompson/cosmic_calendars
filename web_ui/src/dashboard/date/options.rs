use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use icu_calendar::AsCalendar;
use icu_calendar::Gregorian;
use leptos::server_fn::default;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::core::Position;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use osm_db::julian::Julian;
use osm_db::kepler::Body;
use osm_db::kepler::TimeZone;
use osm_db::planets::earth::EarthDate;
use osm_db::planets::earth::EarthDateTime;
use osm_db::planets::earth::EarthTimeZones;
use osm_db::planets::earth::RustSolarCalendar;
use osm_db::planets::mars::Mars;
use osm_db::planets::mars::Martian;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use strum::EnumProperty;
use web_sys::Event;
use web_sys::HtmlLabelElement;
use web_sys::HtmlLiElement;
use web_sys::HtmlSpanElement;
use web_sys::HtmlUListElement;
use web_sys::MouseEvent;
extern crate inflector;
use inflector::Inflector;

#[component]
pub fn HostOptions(#[prop(into)]keep_earth: bool) -> impl IntoView {
    view! {
        <option selected value="">
            "Host: <Select a Host>"
        </option>
        <Show when=move || keep_earth == true>
            <option value="Earth">Host: Earth</option>
        </Show>
        <option value="Mars">Host: Mars</option>
        <option value="Mercury">Host: Mercury</option>
        <option value="Neptune">Host: Neptune</option>
        <option value="Jupiter">Host: Jupiter</option>
        <option value="Saturn">Host: Saturn</option>
        <option value="Pluto">Host: Pluto</option>
        <option value="Venus">Host: Venus</option>
        <option value="Uranus">Host: Uranus</option>
        <option value="Luna">Host: Luna</option>
        <option value="Europa">Host: Europa</option>
        <option value="Io">Host: Io</option>
        <option value="Ganymede">Host: Ganymede</option>
        <option value="Titan">Host: Titan</option>
        <option value="Ceres">Host: Ceres</option>
        <option value="4-Vesta">Host: 4 Vesta</option>
        <option value="2-Pallas">Host: 2 Pallas</option>
        <option value="10-Hygiea">Host: 10 Hygiea</option>
        <option value="52-Europa">Host: 52 Europa</option>
        <option value="511-Davida">Host: 511 Davida</option>
        <option value="3-Juno">Host: 3 Juno</option>
        <option value="433-Eros">Host: 433 Eros</option>
        <option value="6-Hebe">Host: 6 Hebe</option>
        <option value="Halley">Host: Halley</option>
        <option value="Hale-Bopp">Host: Hale-Bopp</option>
        <option value="Kepler-442b">Host: Kepler-442b</option>
        <option value="Kepler-22b">Host: Kepler-22b</option>
        <option value="Kepler-186f">Host: Kepler-186f</option>
    }
}

#[component]
pub fn CalendarOptions() -> impl IntoView {
    view! {
        <option selected value="">
            "Calendar: <Select a Calendar>"
        </option>
        <option value="gregorian">Calendar: Gregorian</option>
        <option value="julian">Calendar: Julian</option>
        <option value="indian">Calendar: Indian</option>
        <option value="iso">Calendar: Iso</option>
        <option value="chinese">Calendar: Chinese</option>
        <option value="roc">Calendar: Republic of China (Roc)</option>
        <option value="ethiopian_alem">Calendar: Ethiopian Alem</option>
        <option value="ethiopian_mihret">Calendar: Ethiopian Mihret</option>
        <option value="japanese_extended">Calendar: Japanese Extended</option>
        <option value="japanese_heisei">Calendar: Japanese Heisei</option>
        <option value="japanese_reiwa">Calendar: Japanese Reiwa</option>
        <option value="japanese_meiji">Calendar: Japanese Meiji</option>
        <option value="japanese_taisho">Calendar: Japanese Taisho</option>
        <option value="japanese_showa">Calendar: Japanese Showa</option>
        <option value="persian">Calendar: Persian</option>
        <option value="coptic">Calendar: Coptic</option>
        <option value="hebrew">Calendar: Hebrew</option>
        <option value="buddhist">Calendar: Buddhist</option>
        <option value="islamic_civil">Calendar: Islamic Civil</option>
        <option value="islamic_observational">Calendar: Islamic Observational</option>
        <option value="islamic_tabular">Calendar: Islamic Tabular</option>
        <option value="islamic_umm_al_qura">Calendar: Islamic Umm Al Qura</option>
    }
}
