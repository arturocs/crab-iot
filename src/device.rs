use crate::error::Error;
use crate::plugin::Plugin;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Device {
    name: String,
    active: bool,
    read_only: bool,
    plugin: Option<Plugin>,
    ip: IpAddr,
}

impl Device {
    fn search() -> Result<Vec<IpAddr>, Error> {
        todo!()
    }
    fn new(name: &str, read_only: bool, ip: IpAddr, plugin: Plugin) -> Device {
        Self {
            name: name.to_string(),
            active: false,
            read_only,
            plugin: Some(plugin),
            ip,
        }
    }
    pub fn without_plugin(name: &str, read_only: bool, ip: IpAddr) -> Device {
        Self {
            name: name.to_string(),
            active: false,
            read_only,
            plugin: None,
            ip,
        }
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
    fn change_status(&self, status: &str) -> Result<(), Error> {
        todo!()
    }
}
