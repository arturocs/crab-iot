use once_cell::sync::Lazy;
use serde::Deserialize;
use utils::weather_json_schema;
use std::{ffi::CString, os::raw::c_char};

static WEATHER: Lazy<weather_json_schema::Root> = Lazy::new(|| {
    let json = include_str!("../1769_es.json");
    serde_json::from_str(json).unwrap()
});

#[derive(Deserialize)]
enum Query {
    Forecast { day: u8 },
    Climate { month: u8 },
}

/// # Safety
///
/// query_str should come from from calling into_raw() on a CString.

#[no_mangle]
pub unsafe extern "C" fn get_status(query_str: *mut c_char) -> *mut c_char {
    let query_cstr = CString::from_raw(query_str);
    let query: Result<Query, serde_json::Error> =
        serde_json::from_str(query_cstr.to_str().unwrap());
    match query {
        Ok(q) => match q {
            Query::Forecast { day } => {
                if day <= 3 {
                    let forecast_day = &WEATHER.city.forecast.forecast_day[day as usize];
                    let forecast_string = serde_json::to_string(forecast_day).unwrap();
                    CString::new(forecast_string).unwrap().into_raw()
                } else {
                    CString::new(r#"{"error": "Maximum number of days allowed is 3"}"#)
                        .unwrap()
                        .into_raw()
                }
            }
            Query::Climate { month } => {
                if month <= 11 {
                    let climate_month = &WEATHER.city.climate.climate_month[month as usize];
                    let climate_string = serde_json::to_string(climate_month).unwrap();
                    CString::new(climate_string).unwrap().into_raw()
                } else {
                    CString::new(r#"{"error": "Maximum number of months allowed is 11"}"#)
                        .unwrap()
                        .into_raw()
                }
            }
        },
        Err(e) => CString::new(format!(r#"{{"error": "{}"}}"#, e))
            .unwrap()
            .into_raw(),
    }
}
