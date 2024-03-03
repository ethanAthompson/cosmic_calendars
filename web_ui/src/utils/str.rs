/// This function removes the first and last quotes from a string.
/// 
/// Why would you need this? an experience with serde_json::Value..
///  
pub fn rustify_str(input: String) -> String {

    // logging::log!("{}", input);

    let body = input.as_str();
    
    let body = &body[1..&body.len() - 1];

    body.to_string()
}