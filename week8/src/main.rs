use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData {
    temp: f32,
    humidity: f32,
    wind_speed: f32,
    // Add other fields as needed
}
use std::io;

fn main() {
    println!("Enter a location:");
    let mut location = String::new();
    io::stdin().read_line(&mut location).unwrap();
    location = location.trim().to_string();

    let api_key = "your_api_key_here";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        location, api_key
    );

    let response = reqwest::blocking::get(&url).unwrap().json::<WeatherData>().unwrap();

    println!("{:#?}", response);

    let api_key = "55cdbfc70199682ae1a9beda0bf689a6";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        location, api_key
    );

    let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherData>()
        .unwrap();

    println!("Temperature: {} °C", response.temp);
    println!("Humidity: {}%", response.humidity);
    println!("Wind Speed: {} m/s", response.wind_speed);
}