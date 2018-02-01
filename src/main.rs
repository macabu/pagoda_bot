extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate dotenv;

mod json_struct;
mod weather_data;
mod test_weather;

use dotenv::*;
use std::env;

use futures::{Future, Stream};
use tokio_core::reactor::{Core, Handle};
use telegram_bot::{Api, Message, ParseMode, MessageKind, UpdateKind};
use telegram_bot::prelude::*;

use weather_data::fetch_weather_data;

fn send_weather_data(api: Api, message: Message, handle: &Handle) {
    let msg = match message.kind {
        MessageKind::Text {ref data, ..} => data,
        _ => return
    };

    let parameters: Vec<&str> = msg.split(" ").collect();

    let city_name = match parameters.get(1) {
        Some(x) => x,
        _ => ""
    };

    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&lang=pt&APPID={}", city_name, env::var("OWM_KEY").unwrap());
    let ret = fetch_weather_data(&url).unwrap();

    let emoji_main = match ret.weather.get(0).unwrap().main.as_ref() {
        "Clear" => "☀️",
        "Mist" | "Clouds" => "☁️",
        "Rain" => "☔️",
        "Thunder" => "⚡️",
        "Snow" => "❄️",
        _ => ""
    };

    let country_flag = match ret.sys.country.as_ref() {
        "BR" => "🇧🇷",
        "RU" => "🇷🇺",
        "GB" => "🇬🇧",
        "US" => "🇺🇸",
        _ => ret.sys.country.as_ref()
    };

    let rth = format!("{} {} {}\n{:.1} °C ({:.0} ~ {:.0})", emoji_main, ret.name, country_flag, ret.main.temp, ret.main.temp_min, ret.main.temp_max);

    if (parameters.len() > 1) && (ret.cod == 200) {
        let response = api.send(message.chat.text(rth).
                parse_mode(ParseMode::Markdown)
        );

        handle.spawn({
            response.map_err(|_| ()).map(|_| ())
        });
    }
}

fn send_help(api: Api, message: Message, handle: &Handle) {
    let response = api.send(message.chat.text("💡 *Help - Pagoda Bot*\nUse /w followed by the city, and country code if needed.\ne.g.: moscow,ru")
        .parse_mode(ParseMode::Markdown)
    );

    handle.spawn({
        response.map_err(|_| ()).map(|_| ())
    })
}

fn popo(api: Api, message: Message, handle: &Handle) {
    let function: fn(Api, Message, &Handle) = match message.kind {
        MessageKind::Text {ref data, ..} => {
            let cmd: Vec<&str> = data.split(" ").collect();
            match cmd.first().unwrap().as_ref() {
                "/tempo" | "/w" => send_weather_data,
                "/ajuda" | "/help" => send_help,
                _ => return
            }
        }
        _ => return
    };

    function(api, message, handle)
}

fn run() -> () {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let tkey = env::var("TGRM_KEY").unwrap();
    let api = Api::configure(tkey).build(core.handle()).unwrap();

    let f = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            popo(api.clone(), message, &handle)
        }
        Ok(())
    });

    core.run(f).unwrap();
}

fn main() {
    dotenv().ok();
    run()
}
