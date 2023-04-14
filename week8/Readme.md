# Rust mini project Week 8

This is a rust mini project for a Command-line tools that retrieves the current weather data for a specified location

## Implementation
First, we need to add some dependencies to our Cargo.toml file. We'll be using the reqwest and serde_json crates to make HTTP requests and parse JSON responses, respectively.

```
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde_json = "1.0"
```

We'll need to prompt the user for a location. We can use the std::io module to read input from the command line. We can now make an HTTP request to retrieve the weather data for the specified location. We'll be using the OpenWeather API for this example, which requires an API key. You can get a free API key by signing up on their website.

**Please note that you need to change the API to your own if you want to connect it to your own weather data!**

## How to use

`make all`
