pub mod rdevice;
pub mod rwdevice;
use serde_json::Value;
use std::error::Error;
use std::net::IpAddr;

pub fn local_search() -> Result<Vec<IpAddr>, Box<dyn Error>> {
    todo!()
}
pub(crate) trait Readable {
    fn new(
        name: &str,
        plugin_name: &str,
        plugin_path: &str,
        ip: &str,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;
    fn get_ip(&self) -> IpAddr;
    fn get_name(&self) -> String;
    fn get_status(&self) -> Result<Value, Box<dyn Error>>;
}
pub(crate) trait Writable: Readable {
    fn set_status(&self, status: &Value) -> Result<Value, Box<dyn Error>>;
}
