/**
 * Structs for matching the json data response.
 * Functions are in case the response doesn't contain such values.
 *
 **/

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub id: i32,
    pub name: Option<String>,
    pub country: Option<String>,
    pub coord: Coord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    #[serde(skip)]
    pub sea_level: f32,
    #[serde(skip)]
    pub grnd_level: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
    #[serde(rename = "3h")]
    pub _3h: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snow {
    #[serde(rename = "3h")]
    pub _3h: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    #[serde(skip)]
    pub _type: i32,
    #[serde(skip)]
    pub id: i32,
    pub message: f32,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OWM {
    #[serde(default = "default_coord")]
    pub coord: Coord,
    #[serde(default = "default_weather")]
    pub weather: Vec<Weather>,
    #[serde(default = "default_basew")]
    pub base: String,
    #[serde(default = "default_mainw")]
    pub main: Main,
    #[serde(default = "default_wind")]
    pub wind: Wind,
    #[serde(default = "default_clouds")]
    pub clouds: Clouds,
    #[serde(default = "default_rain")]
    pub rain: Rain,
    #[serde(default = "default_snow")]
    pub snow: Snow,
    #[serde(default = "default_dt")]
    pub dt: u32,
    #[serde(default = "default_sys")]
    pub sys: Sys,
    #[serde(default = "default_id")]
    pub id: i32,
    #[serde(default = "default_name")]
    pub name: String,
    pub cod: i32
}

// TODO: Is it possible to apply generics?
fn default_rain() -> Rain {
    Rain {
        _3h: 0.0
    }
}

fn default_snow() -> Snow {
    Snow {
        _3h: 0.0
    }
}

fn default_coord() -> Coord {
    Coord {
        lon: 0.0,
        lat: 0.0
    }
}

fn default_weather() -> Vec<Weather> {
    let mut r = Vec::new();
    r.push(Weather {
        id: 0,
        main: String::new(),
        description: String::new(),
        icon: String::new()
    });
    r
}

fn default_clouds() -> Clouds {
    Clouds {
        all: 0.0
    }
}

fn default_sys() -> Sys {
    Sys {
        _type: 0,
        id: 0,
        message: 0.0,
        country: String::new(),
        sunrise: 0,
        sunset: 0        
    }
}

fn default_wind() -> Wind {
    Wind {
        speed: 0.0,
        deg: 0.0
    }
}

fn default_mainw() -> Main {
    Main {
        temp: 0.0,
        pressure: 0.0,
        humidity: 0.0,
        temp_min: 0.0,
        temp_max: 0.0,
        sea_level: 0.0,
        grnd_level: 0.0
    }
}

fn default_basew() -> String {
    String::new()
}

fn default_dt() -> u32 {
    0
}

fn default_id() -> i32 {
    0
}

fn default_name() -> String {
    String::new()
}
