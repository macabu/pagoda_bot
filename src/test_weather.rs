extern crate dotenv;

use dotenv::*;
use std::env;
use weather_data::fetch_weather_data;
use json_struct::OWM;

/**
 * Helper function for some tests, to simply return a OWM struct 
 **/
#[allow(dead_code)]
fn helper_test_city_id<'a>(city_name: &'a str, api_key: &'a str) -> OWM {
    let f = format!("http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}", city_name, api_key);
    fetch_weather_data(&f).unwrap()
}

/**
 * This tests if the the city id matches with the one being searched,
 * in the case where we get a successful response.
 **/
#[test]
fn pravilno_nazvanie() {
    dotenv().ok();

    let ret = helper_test_city_id("moscow,ru", env::var("OWM_KEY").unwrap().as_ref());
    assert_eq!(524901, ret.id);
    assert_eq!(200, ret.cod);
}

/**
 * This tests if the the city id matches with the one being searched,
 * in the case where the city doesn't exist.
 *
 * Attention!
 * This test checks for the return code to equal "404" (string) because
 * that's what the OWM API returns.
 **/
#[test]
fn nepravilno_nazvanie() {
    dotenv().ok();

    let ret = helper_test_city_id("asdasdsa", env::var("OWM_KEY").unwrap().as_ref());
    assert_eq!(0, ret.id);
    assert_eq!("404", ret.cod);
}

/**
 * This tests if the the city id matches with the one being searched,
 * in the case where the API key is invalid.
 **/
#[test]
fn net_api() {
    let ret = helper_test_city_id("moscow,ru", "123");
    assert_eq!(0, ret.id);
    assert_eq!(401, ret.cod);
}
