#[macro_use]
extern crate lazy_static;

//pub mod weather_json_schema;

use coap::{CoAPRequest, CoAPResponse, CoAPServer, Method};
use std::{sync::Mutex, thread, time::Duration};
use utils::weather_json_schema::{Forecast, Root};

lazy_static! {
    static ref INFO: Mutex<WeatherInfo> = Mutex::new(WeatherInfo::Invalid);
}

#[derive(Clone)]
enum WeatherInfo {
    Valid(Forecast),
    Invalid,
}

fn request_handler(request: CoAPRequest) -> Option<CoAPResponse> {
    println!("recived request {}", request.get_path());
    if request.get_method() == &Method::Get {
        let json_response = match INFO.lock().unwrap().clone() {
            WeatherInfo::Valid(forecast) => match request.get_path().as_str() {
                "forecast" => serde_json::to_string(&forecast.forecast_day).unwrap(),
                "forecast/" => serde_json::to_string(&forecast.forecast_day).unwrap(),
                "forecast/0" => serde_json::to_string(&forecast.forecast_day[0]).unwrap(),
                "forecast/1" => serde_json::to_string(&forecast.forecast_day[1]).unwrap(),
                "forecast/2" => serde_json::to_string(&forecast.forecast_day[2]).unwrap(),
                "forecast/3" => serde_json::to_string(&forecast.forecast_day[3]).unwrap(),
                "forecast/4" => serde_json::to_string(&forecast.forecast_day[4]).unwrap(),
                _ => r#"{ "error" : "Invalid path" }"#.to_string(),
            },
            WeatherInfo::Invalid => r#"{ "error" : "Unable to get forecast" }"#.to_string(),
        };
        match request.response {
            Some(mut message) => {
                message.message.payload = json_response.as_bytes().to_vec();
                Some(message)
            }
            _ => None,
        }
    } else {
        None
    }
}

fn get_climate() -> WeatherInfo {
    let request = || -> Result<Root, Box<dyn std::error::Error>> {
        Ok(reqwest::blocking::get("http://wwis.aemet.es/es/json/1769_es.xml")?.json::<Root>()?)
    };
    match request() {
        Ok(j) => WeatherInfo::Valid(j.city.forecast),
        Err(_) => WeatherInfo::Invalid,
    }
}

fn main() {
    {
        let mut a = INFO.lock().unwrap();
        *a = get_climate();
    }
    let addr = "127.0.0.1:5683";

    let mut threads = vec![];
    threads.push(thread::spawn(move || {
        println!("Mockup device running on {}", addr);
        let mut server = CoAPServer::new(addr).unwrap();
        server.handle(request_handler).unwrap();
        loop {}
    }));

    threads.push(thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(3600));
        println!("Updaing forecast");
        let mut a = INFO.lock().unwrap();
        *a = get_climate();
    }));

    threads.into_iter().for_each(|t| t.join().unwrap());
}
