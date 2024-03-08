// ^ Never use &'str... unless you wanna be stuck with borrow issues )-:

/* This is a one time superviser function that does what I need it to do */

/// This function converts a hosts' year and ls into an earth date
/// (Think of this function as a counter-clockwise clock..) 
/// This function is focused to be used within leptos applications..
/// 
pub fn mega_converter(host: String, year: f64, ls: f64) -> String {
    match host.as_str() {
        "mars" => {
            println!("{:?}/{:?} -> ...", year, ls);
            //
            String::new()
        }
        _ => "".to_string(),
    }
}
