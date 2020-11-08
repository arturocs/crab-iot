#![allow(dead_code, unused_variables, unused_macros)]
#[path = "../src/device/mod.rs"]
mod device;
#[path = "../src/device_list.rs"]
mod device_list;
#[path = "../src/error.rs"]
mod error;
#[path = "../src/plugin.rs"]
mod plugin;

use device::{rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use device_list::DeviceList;

#[test]
fn deserialize_device_list() {
    let devices_from_json = DeviceList::from_json(r#"{"Empty_device":"127.0.0.1"}"#).unwrap();
}
#[test]
fn set_device_status() {
    let mockup_device = RWDevice::new(
        "mockup_device",
    )
    .unwrap();
    let status = mockup_device.set_status(&json!({"on":true})).unwrap();
    let data = status.get("data").unwrap();
    assert_eq!(data, &json!({"on":true}));
}

#[test]
fn get_status_from_api() {
    let response : serde_json::Value = reqwest::blocking::get("http://127.0.0.1:3030/api").unwrap().json().unwrap();
    assert_eq!(response.to_string(),r#"{"on":false}"#)
}
