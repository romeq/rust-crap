mod weather;

use weather::ApiHandler;
use std::{env, process};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ./weather <city>");
        process::exit(1);
    }

    // insert your api key here
    let key: &str = "";
    let city: &String = &args[1];

    let api_handle: ApiHandler = ApiHandler::new(
        String::from(key),
        String::from(city)
    );

    match api_handle.request_weather() {
        Ok(weather_data) => println!("It's {}, {}°C in {}, {}.",
             weather_data.current.condition.text.to_lowercase(), weather_data.current.temp_c,
             weather_data.location.name, weather_data.location.region),

        Err(error) => {
            eprintln!("Oops; Weather API returned error: {}", error);
            std::process::exit(1);
        }
    }
}
