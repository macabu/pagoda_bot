/**
 * Structs for matching the json data response.
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
#[serde(default)]
pub struct OWM {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub wind: Wind,
    pub clouds: Clouds,
    pub rain: Rain,
    pub snow: Snow,
    pub dt: u32,
    pub sys: Sys,
    pub id: i32,
    pub name: String,
    pub cod: i32
}

impl Default for OWM {
    fn default() -> Self {
        OWM {
            coord: Coord {
                lon: 0.0,
                lat: 0.0
            },
            weather: default_weather(),
            base: String::new(),
            main: Main {
                temp: 0.0,
                pressure: 0.0,
                humidity: 0.0,
                temp_min: 0.0,
                temp_max: 0.0,
                sea_level: 0.0,
                grnd_level: 0.0
            },
            wind: Wind {
                speed: 0.0,
                deg: 0.0
            },
            clouds: Clouds {
                all: 0.0
            },
            rain: Rain {
                _3h: 0.0
            },
            snow: Snow {
                _3h: 0.0
            },
            dt: 0,
            sys: Sys {
                _type: 0,
                id: 0,
                message: 0.0,
                country: String::new(),
                sunrise: 0,
                sunset: 0        
            },
            id: 0,
            name: String::new(),
            cod: 0        
        }
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
