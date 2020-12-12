use device::{device_list::DeviceList, rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use pretty_assertions::assert_eq;
use serde_json::json;
use utils::weather_json_schema::{ClimateMonth, ForecastDay};

#[test]
fn deserialize_device_list() {
    let devices_from_json = DeviceList::from_json(r#"{"Empty_device":"127.0.0.1"}"#).unwrap();
    let devices = DeviceList::new(vec![(
        "Empty_device".to_string(),
        "127.0.0.1".parse().unwrap(),
    )]);
    assert_eq!(devices, devices_from_json);
}

#[test]
fn set_device_status() {
    let mut mockup_device = RWDevice::new(
        "fake_device",
        "fake_plugin",
        "./target/debug/libexperimental_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device.set_status(&json!({"on":true})).unwrap();
    assert_eq!(status.get("on").unwrap(), true);
}
#[test]
fn get_experimental_device_status() {
    let mockup_device = RDevice::new(
        "fake_device",
        "fake_plugin",
        "./target/debug/libexperimental_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device.get_status(&json!({})).unwrap();
    assert_eq!(status.get("on").unwrap(), false);
}

#[test]
fn set_status_after_device_deserialization() {
    let mut device = RWDevice::from_json(
        r#"{"name":"device",
        "plugin":{
            "device_name":"plugin",
            "libary_path":"./target/debug/libfake_plugin.so"
        },
        "ip":"127.0.0.1"}"#,
    )
    .unwrap();
    let status = device.set_status(&mut json!({"on":true})).unwrap();
    let data = status.get("on").unwrap().as_bool().unwrap();
    assert_eq!(data, true);
}

#[test]
fn get_climate_month() {
    let mockup_device = RDevice::new(
        "weather_device",
        "weather_fake_plugin",
        "./target/debug/libweather_fake_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device
        .get_status(&json!({"Climate":{"month":5}}))
        .unwrap();
    let status: ClimateMonth = serde_json::from_value(status).unwrap();

    assert_eq!(
        status,
        ClimateMonth {
            month: 6,
            max_temp: "31.0".to_string(),
            min_temp: "13.6".to_string(),
            mean_temp: None,
            max_temp_f: "87.8".to_string(),
            min_temp_f: "56.5".to_string(),
            mean_temp_f: None,
            raindays: "2.8".to_string(),
            rainfall: "11.2".to_string(),
            climate_from_mem_date: "2012-03-12".to_string()
        }
    );
}

#[test]
fn get_forecast_day() {
    let mockup_device = RDevice::new(
        "weather_device",
        "weather_fake_plugin",
        "./target/debug/libweather_fake_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device
        .get_status(&json!({"Forecast":{"day":2}}))
        .unwrap();
    let status: ForecastDay = serde_json::from_value(status).unwrap();

    assert_eq!(
        status,
        ForecastDay {
            forecast_date: "2020-11-25".to_string(),
            wxdesc: "".to_string(),
            weather: "Lluvia".to_string(),
            min_temp: "7".to_string(),
            max_temp: "19".to_string(),
            min_temp_f: "45".to_string(),
            max_temp_f: "66".to_string(),
            weather_icon: 1401,
        }
    );
}
