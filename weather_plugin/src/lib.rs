use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

use coap::CoAPClient;

#[derive(Deserialize, Debug)]
enum Query {
    Forecast { day: u8 },
    Climate { month: u8 },
}

fn process_query(query: &Value) -> Result<Value, String> {
    let ip = query
        .get("ip")
        .ok_or("Query missing 'ip' field")?
        .as_str()
        .ok_or("Ip must be a string")?;
    let query = query.get("query").ok_or("Query missing 'query' field")?;

    let query = serde_json::from_value(query.clone()).map_err(|e| e.to_string())?;
    match query {
        Query::Forecast { day } => {
            if day <= 4 {
                let url = &format!("coap://{}/forecast/{}", ip, day);
                let response = CoAPClient::get_with_timeout(url, Duration::from_secs(15))
                    .map_err(|e| e.to_string())?;
                let s = String::from_utf8(response.message.payload).map_err(|e| e.to_string())?;
                Ok(serde_json::from_str(&s).map_err(|e| e.to_string())?)
            } else {
                Err("Maximum number of days allowed is 4".to_string())
            }
        }
        _ => Err("Invalid query".to_string()),
    }
}

/// # Safety
///
/// query argument should come from from calling Box::into_raw() in a boxed serde_json::Value.

#[no_mangle]
pub unsafe extern "C" fn get_status(query: *const Value) -> *mut Result<Value, String> {
    let query = query.as_ref();
    let status = match query {
        Some(v) => process_query(v),
        None => Err("Unable to dereference query".to_string()),
    };
    Box::into_raw(Box::new(status))
}
