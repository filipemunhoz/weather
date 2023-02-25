use std::time::Instant;
use reqwest::Client;
use serde::Deserialize;
use futures::{stream, StreamExt};

const CONCURRENT_REQUESTS: usize = 2;

#[derive(Deserialize)]
struct Weather {
    name: String,
    wind: Wind,
    weather: Vec<WeatherCondition>,
    main: Main,
}

#[derive(Deserialize)]
struct WeatherCondition {
    main: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize)]
struct Wind {
    speed: f32,
    deg: i32,
}

#[tokio::main]  
async fn main() -> Result<(), reqwest::Error> {

    let start = Instant::now();

    let urls = vec!["https://api.openweathermap.org/data/2.5/weather?units=metric&lat=51.509865&lon=-0.118092&appid=71e4be94cf9827a2d5657f6a140ad79e",
                                    "https:/api.openweathermap.org/data/2.5/weather?units=metric&lat=-23.5489&lon=-46.6388&appid=71e4be94cf9827a2d5657f6a140ad79e"];

    let client = Client::new();
    let bodies = stream::iter(urls)
    .map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    })
    .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => {
                    print_output(&b);
                },
                Err(_e) => eprintln!("Not connected"),
            }
        })
        .await;

    let duration = start.elapsed().as_millis();

    println!("Duration {}ms", duration);
    Ok(())
}

fn print_output(b: &[u8]){
    let weather: Weather = serde_json::from_slice(&b).unwrap();
    let temperature = weather.main.temp.round() as i32;
    let temp_min = weather.main.temp_min.round() as i32;
    let temp_max = weather.main.temp_max.round() as i32;    
    let wind_degrees = weather.wind.deg;
    let wind_speed = weather.wind.speed.round() as i32;
    let weather_condition = &weather.weather.first().ok_or("").unwrap().main;

    println!("{:<12} {:>2}° {:>2}{} {:>2}{}  {}  {}{}  {}",
        weather.name, 
        temperature, 
        temp_min, '⬇', 
        temp_max, '⬆', 
        emoji_face(temperature),
        wind_speed,
        emoji_wind_direction(wind_degrees),
        emoji_weather_condition(weather_condition),
    );    
}

fn emoji_face(temperature: i32) -> String {
    match temperature {
        -50..=16 => format!("{}{}",'🥶', '🧦'),
        17..=22 => format!("{}{}",'😀', '🧥'),
        23..=30 => format!("{}{}", '😀', '🩳'),
        31..=50 => format!("{}{}", '🥵', '🏜'),
        _ => format!("{}", '❓'),        
    }    
}

fn emoji_wind_direction(degree: i32) -> String {
    match degree {
        0..=10 => format!("{}",'⬇'),
        11..=78 => format!("{}",'↙'),
        79..=123 => format!("{}",'⬅'),
        124..=168 => format!("{}",'↖'),
        169..=213 => format!("{}",'⬆'),
        214..=258 => format!("{}",'↗'),
        259..=303 => format!("{}",'➡'),
        304..=348 => format!("{}",'↘'),
        349..=380 => format!("{}",'⬇'),
        _ => format!("{}", '❓'),        
    }   
}

fn emoji_weather_condition(c: &String) -> String {
    match c.as_str() {
        "Tornado" => format!("{}",'🌪'),
        "Fog" => format!("{}",'🌫'),
        "Thunderstorm" => format!("{}",'🌩'),
        "Snow" => format!("{}",'❄'),
        "Rain" => format!("{}",'⛈'),
        "Clear" => format!("{}",'☀'),
        "Clouds" => format!("{}",'☁'),
        "Haze" => format!("{}",'🌫'),
        "Squall" => format!("{}",'🌪'),
        "Drizzle" => format!("{}",'🌧'),
        "Mist" => format!("{}",'🌫'),
        "Smoke" => format!("{}",'🌫'),      
        "Dust" => format!("{}",'🌫'),        
        "Sand" => format!("{}",'🌫'),
        "Ash" => format!("{}",'🌫'),
        _ => format!("{}", '❓'),        
    }   
}