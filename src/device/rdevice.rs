use crate::error::Error;
use crate::plugin::Plugin;
use super::device_attributes::DeviceAttributes;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct RDevice (DeviceAttributes);

impl RDevice {
    fn new(name: &str, ip: IpAddr, plugin: Plugin) -> Self {
        Self (DeviceAttributes{
            name: name.to_string(),
            active: false,
            plugin: Some(plugin),
            ip,
        })
    }
    pub fn without_plugin(name: &str, ip: IpAddr) -> Self {
        Self (DeviceAttributes{
            name: name.to_string(),
            active: false,
            plugin: None,
            ip,
        })
    }
    fn on(&self) -> Result<(), Error> {
        todo!()
    }
    fn off(&self) -> Result<(), Error> {
        todo!()
    }
    fn get_status(&self) -> Result<String, Error> {
        todo!()
    }
}