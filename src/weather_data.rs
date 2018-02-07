extern crate reqwest;
extern crate serde;
extern crate serde_json;

use json_struct::OWM;

/**
 * Fetch weather data from the given json-API.
 * 
 * Uses `reqwest` to fetch URL data, then match data with possible responses,
 * to finally unwrap data with `serde_json`.
 * 
 * Returns a Result trait containing the struct and an error string, if existing.
 * 
 **/
pub fn fetch_weather_data<'a>(url: &'a str) -> Result<OWM, &'a str> {
    let mut r = reqwest::get(url).unwrap();

    let jr : OWM = r.json().unwrap();

    Ok(jr)
}
