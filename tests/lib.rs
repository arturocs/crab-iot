#![allow(dead_code, unused_variables, unused_macros)]
#[path = "../src/device.rs"]
mod device;
#[path = "../src/device_list.rs"]
mod device_list;
#[path = "../src/error.rs"]
mod error;
#[path = "../src/plugin.rs"]
mod plugin;
use std::net::{IpAddr, Ipv4Addr};

use device::Device;
use device_list::DeviceList;

#[test]
fn test_vacio() {
    assert_eq!((), ());
}
#[test]
fn deserialize_device_list() {
    let devices_from_json = DeviceList::from_json(
        r#"[{"name":"prueba","active":false,"read_only":false,"plugin":null,"ip":"127.0.0.1"}]"#,
    )
    .unwrap();
    let devices = DeviceList::new(vec![Device::without_plugin("prueba",false,
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    )]);
    dbg!(&devices);
    assert_eq!(devices, devices_from_json);
}
