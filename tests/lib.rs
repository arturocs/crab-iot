use device::{device_list::DeviceList, rdevice::RDevice, rwdevice::RWDevice, Readable, Writable};
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn deserialize_device_list() {
    let devices_from_json = DeviceList::from_json(r#"{"Empty_device":"127.0.0.1"}"#).unwrap();
    let devices = DeviceList::new(vec![(
        "Empty_device".to_string(),
        "127.0.0.1".parse().unwrap(),
    )]);
    dbg!(&devices.to_json());
    assert_eq!(devices, devices_from_json);
}

#[test]
fn get_device_status() {
    let mockup_device = RDevice::new(
        "mockup_device",
        "fake_plugin",
        "./target/debug/libfake_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device.get_status(&json!({})).unwrap();
    let data = status.get("data").unwrap();
    assert_eq!(data, &json!({"on":false}));
}

#[test]
fn set_device_status() {
    let mut mockup_device = RWDevice::new(
        "mockup_device",
        "fake_plugin",
        "./target/debug/libfake_plugin.so",
        "127.0.0.1",
    )
    .unwrap();
    let status = mockup_device.set_status(&json!({"on":true})).unwrap();
    let data = status.get("data").unwrap();
    assert_eq!(data, &json!({"on":true}));
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
    let status = device.set_status(&json!({"on":true})).unwrap();
    let data = status.get("data").unwrap();
    assert_eq!(data, &json!({"on":true}));
}
