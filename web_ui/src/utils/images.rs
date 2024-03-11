use leptos::{create_resource, create_signal, logging, ReadSignal, Resource, WriteSignal};

use crate::stores::sheets::Row;
use crate::stores::sheets::SheetData;
use crate::utils::str::rustify_str;
use crate::stores::states::FeedbackSearchFilterStates;
use crate::utils::colors::direct_feedback_types;
use crate::utils::sheets::has_resolved;
use crate::stores::states::FeedbackSearchStates;


 /// This function matches the response and returns an src link
 pub fn direct_profile_picture(pfp: String) -> String {
    match pfp.as_ref() {
        "Luna" => "https://science.nasa.gov/_ipx/animated_true&w_1280&f_webp/https://images-assets.nasa.gov/image/as11-40-5875/as11-40-5875~large.jpg%3Fw=1856%26h=1920%26fit=clip%26crop=faces%252Cfocalpoint",
        "Mars" => "https://mars.nasa.gov/system/resources/detail_files/21360_mars-twisted-tail.jpg",
        "Earth" => "https://explorer1.jpl.nasa.gov/assets/images/galleries/1972_BlueMarble_115334main_image_feature_329_ys_full.jpg",
        "Saturn" => "https://science.nasa.gov/_ipx/animated_true&w_1280&f_webp/https://images-assets.nasa.gov/image/PIA03152/PIA03152~orig.jpg%3Fw=849%26h=900%26fit=clip%26crop=faces%252Cfocalpoint",
        "Jupiter" => "https://www.nasa.gov/wp-content/uploads/2023/03/52303461859_0db4d9b739_o.png",
        "Neptune" => "https://www.nasa.gov/wp-content/uploads/2023/03/pia01492-main.jpg",
        "Ganymede" => "https://photojournal.jpl.nasa.gov/jpegMod/PIA01666_modest.jpg",
        "Mercury" => "https://science.nasa.gov/wp-content/uploads/2023/04/EW0108829708G4release_mercury-jpg.webp?w=1280&format=webp",
        _ => "https://www.nasa.gov/wp-content/uploads/2024/02/53495965723-5c5c821f78-o.png?resize=1536,859"
    }.to_string()
}