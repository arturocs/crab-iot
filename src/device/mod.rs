pub mod rdevice;
pub mod rwdevice;
use crate::error::Error;
use serde_json::Value;
use std::net::IpAddr;
pub(crate) fn local_search() -> Result<Vec<IpAddr>, Error> {
    todo!()
}
pub(crate) trait Readable {
    fn new(name: &str, plugin_name: &str, plugin_path: &str, ip: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized;
    fn get_ip(&self) -> IpAddr;
    fn get_name(&self) -> String;
    fn get_status(&self) -> Result<Value, Error>;
}
pub(crate) trait Writable: Readable {
    fn set_status(&self, status: &Value) -> Result<Value, Error>;
}
