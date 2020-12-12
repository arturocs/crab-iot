use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use utils::weather_json_schema;
use utils::{error::Error, *};

static WEATHER: Lazy<weather_json_schema::Root> = Lazy::new(|| {
    let json = include_str!("../1769_es.json");
    serde_json::from_str(json).unwrap()
});

#[derive(Deserialize, Debug)]
enum Query {
    Forecast { day: u8 },
    Climate { month: u8 },
}

fn process_query(query: &Value) -> Result<Value, Error> {
    let query = query
        .get("query")
        .ok_or(error!("Query missing 'query' field"))?;
    let query = serde_json::from_value(query.clone()).map_err(|e| error!(e))?;
    match query {
        Query::Forecast { day } => {
            if day <= 3 {
                let forecast_day = &WEATHER.city.forecast.forecast_day[day as usize];
                serde_json::to_value(forecast_day).map_err(|e| error!(e))
            } else {
                Err(error!("Maximum number of days allowed is 3"))
            }
        }
        Query::Climate { month } => {
            if month <= 11 {
                let climate_month = &WEATHER.city.climate.climate_month[month as usize];
                serde_json::to_value(climate_month).map_err(|e| error!(e))
            } else {
                Err(error!("Maximum number of months allowed is 11"))
            }
        }
    }
}

/// # Safety
///
/// query should come from from calling Box::into_raw() in a boxed serde_json::Value.

#[no_mangle]
pub unsafe extern "C" fn get_status(query: *const Value) -> *mut Result<Value, Error> {
    let query = query.as_ref();
    let status = match query {
        Some(v) => process_query(v),
        None => Err(error!("Unable to dereference query")),
    };
    Box::into_raw(Box::new(status))
}
