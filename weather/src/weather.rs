use reqwest;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherCondition {
    pub(crate) text: String
}

#[derive(Deserialize)]
pub struct WeatherCurrent {
    pub(crate) temp_c: f32,
    pub(crate) condition: WeatherCondition
}

#[derive(Deserialize)]
pub struct WeatherLocation {
    pub(crate) name: String,
    pub(crate) region: String
}

#[derive(Deserialize)]
pub struct Weather {
    pub(crate) current: WeatherCurrent,
    pub(crate) location: WeatherLocation
}

pub struct ApiHandler {
    key: String,
    city: String
}

impl ApiHandler {
    pub fn new(key: String, city: String) -> Self {
        Self {
            key,
            city
        }
    }

    pub fn request_weather(self) -> Result<Weather, String> {
        let base_url: &str = "https://api.weatherapi.com/v1";
        let key: String = self.key;
        let city: String = self.city;
        let full_url: String = format!("{}/current.json?key={}&q={}&aqi=no",
                                       base_url, key, city);

        let mut request: Response = reqwest::get(&full_url).unwrap();
        if request.status() != reqwest::StatusCode::OK {
            return Err(String::from(request.status().as_str()));
        }

        Ok(request.json().unwrap())
    }
}
