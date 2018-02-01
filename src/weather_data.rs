extern crate reqwest;
extern crate serde;
extern crate serde_json;

use json_struct::OWM;
use reqwest::{StatusCode};

/**
 * Fetch weather data from the given json-API.
 * 
 * Uses `reqwest` to fetch URL data, then match data with possible responses,
 * to finally unwrap data with `serde_json`, or in the case of not ok response,
 * make a dummy json response.
 * 
 * For some reason, the OWM API responds with strings for the value in `cod` as String,
 * instead of i32, so we have to deal with that.
 * 
 * Returns a Result trait containing the struct and an error string, if existing.
 * 
 **/
pub fn fetch_weather_data<'a>(url: &'a str) -> Result<OWM, &'a str> {
    let mut r = reqwest::get(url).unwrap();

    // TODO : think of a better way to solve this. Generics and traits? Enum/unions?
    let data = match r.status() {
        StatusCode::NotFound    => r#"{ "cod": 404, "message": "city not found" }"#,
        StatusCode::BadRequest  => r#"{ "cod": 400, "message": "bad request" }"#,
        _ => ""
    };

    let jr : OWM = if data.is_empty() { r.json().unwrap() } else { serde_json::from_str(data).unwrap() };
    Ok(jr)
}
