use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    humidity: u32,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "YOUR_API_KEY"; // Replace with your OpenWeatherMap API key
    let city = env::args().nth(1).expect("Please provide a city name");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response: WeatherResponse = reqwest::get(&url).await?.json().await?;

    println!("Current weather in {}:", city);
    println!("Temperature: {}°C", response.main.temp);
    println!("Humidity: {}%", response.main.humidity);
    println!("Condition: {}", response.weather[0].description);

    Ok(())
}
