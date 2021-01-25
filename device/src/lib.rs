#![allow(dead_code, unused_variables, unused_macros)]
pub mod device_list;
pub mod plugin;
pub mod rdevice;
pub mod rwdevice;
use plugin::Plugin;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::IpAddr;
use utils::{error::Error};
pub fn local_search() -> Result<Vec<IpAddr>, Error> {
    todo!()
}
pub trait Readable<'a>: PartialEq + Serialize + Deserialize<'a> {
    fn new(name: &str, plugin_name: &str, plugin_path: &str, ip: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized;
    fn get_ip(&self) -> IpAddr;
    fn get_name(&self) -> &str;
    fn get_plugin(&self) -> &Plugin;
    fn get_mut_plugin(&mut self) -> &mut Plugin;
    fn get_status(&self, query: &Value) -> Result<Value, String> {
        let query = json!({
            "query":query,
            "ip": self.get_ip()
        });
        self.get_plugin().get_status(&query)
    }
    fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self).map_err(|e| e.to_string())
    }
    fn from_json(json: &'a str) -> Result<Self, String> {
        let mut device: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        device.get_mut_plugin().reload_after_deserialize()?;
        Ok(device)
    }
}
pub trait Writable<'a>: Readable<'a> {
    fn set_status(&mut self, status: &Value) -> Result<Value, String> {
        self.get_mut_plugin().set_status(status)
    }
}




