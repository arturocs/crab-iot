use actix_web::{web,  HttpResponse};
use device::{rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
#[derive(Debug, Clone, Deserialize)]
pub struct SwitchStatus {
    on: bool,
}

pub async fn get_real_forecast(web::Path((day,)): web::Path<(String,)>) -> HttpResponse {

    let day = match day.parse::<u8>() {
        Ok(d) => d,
        Err(e) => return HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    };
    let status = REAL_WEATHER.get_status(&json!({"Forecast":{"day":day}}));
    match status {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}

pub async fn get_fake_forecast(web::Path((day,)): web::Path<(String,)>) -> HttpResponse {

    let day = match day.parse::<u8>() {
        Ok(d) => d,
        Err(e) => return HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    };
    let status = FAKE_WEATHER.get_status(&json!({"Forecast":{"day":day}}));
    match status {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}

pub async fn get_fake_climate(web::Path((month,)): web::Path<(String,)>) -> HttpResponse {
    let month = match month.parse::<u8>() {
        Ok(m) => m,
        Err(e) => return HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    };
    let status = FAKE_WEATHER.get_status(&json!({"Climate":{"month":month}}));
    match status {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
    }
}

pub async fn turn_switch(request: web::Json<SwitchStatus>) -> HttpResponse {
    let status: SwitchStatus = request.to_owned();
    FAKE_SWITCH
        .lock()
        .unwrap()
        .set_status(&json!({"on":status.on}))
        .unwrap();
    let message = if status.on {
        "switch turned on"
    } else {
        "switch turned off"
    };
    HttpResponse::Ok().json(message)
}

pub async fn get_switch_status() -> HttpResponse {
    let status = FAKE_SWITCH.lock().unwrap().get_status(&json!({})).unwrap();
    println!("{}", status);
    HttpResponse::Ok().json(status)
}

pub async fn create_rdevice(request: web::Json<RDevice>) -> HttpResponse {
    let status: RDevice = request.to_owned();
    let old_rdevice = RDEVICE_LIST
        .lock()
        .unwrap()
        .insert(status.get_name().to_string(), status);

    let message = if old_rdevice.is_some() {
        "Overwritten old device"
    } else {
        "Created new device"
    };
    HttpResponse::Ok().json(message)
}

pub async fn create_rwdevice(request: web::Json<RWDevice>) -> HttpResponse {
    let status: RWDevice = request.to_owned();
    let old_rdevice = RWDEVICE_LIST
        .lock()
        .unwrap()
        .insert(status.get_name().to_string(), status);

    let message = if old_rdevice.is_some() {
        "Overwritten old device"
    } else {
        "Created new device"
    };
    HttpResponse::Ok().json(message)
}

pub async fn delete_rdevice(web::Path((rdevice,)): web::Path<(String,)>) -> HttpResponse {
    if RDEVICE_LIST.lock().unwrap().remove(&rdevice).is_some() {
        HttpResponse::Ok().json(json!({ "message": "device removed successfully" }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "device doesnt exist" }))
    }
}

pub async fn delete_rwdevice(web::Path((rdevice,)): web::Path<(String,)>) -> HttpResponse {
    if RWDEVICE_LIST.lock().unwrap().remove(&rdevice).is_some() {
        HttpResponse::Ok().json(json!({ "message": "device removed successfully" }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "device doesnt exist" }))
    }
}

pub async fn get_rdevices() -> HttpResponse {
    let devices = RDEVICE_LIST.lock().unwrap();
    HttpResponse::Ok().json(devices.clone())
}

pub async fn get_rwdevices() -> HttpResponse {
    let devices = RWDEVICE_LIST.lock().unwrap();
    HttpResponse::Ok().json(devices.clone())
}

lazy_static! {
    static ref RDEVICE_LIST: Mutex<HashMap<String, RDevice>> = Mutex::new(HashMap::new());
    static ref RWDEVICE_LIST: Mutex<HashMap<String, RWDevice>> = Mutex::new(HashMap::new());
    static ref REAL_WEATHER: RDevice = RDevice::new(
        "weather_device",
        "weather_plugin",
        "./target/debug/libweather_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    static ref FAKE_SWITCH: Mutex<RWDevice> = Mutex::new(
        RWDevice::new(
            "fake_device",
            "fake_plugin",
            "./target/debug/libfake_plugin.so",
            "127.0.0.1",
        )
        .unwrap()
    );
    static ref FAKE_WEATHER: RDevice = RDevice::new(
        "weather_device",
        "weather_fake_plugin",
        "./target/debug/libweather_fake_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
}
