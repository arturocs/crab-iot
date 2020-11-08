use crate::plugin::Plugin;
use serde_json::Value;

use super::{Readable, Writable};
use std::error::Error;
use std::net::IpAddr;
#[derive(Debug)]
pub(crate) struct RWDevice {
    name: String,
    plugin: Plugin,
    ip: IpAddr,
}

impl Readable for RWDevice {
    fn new(
        name: &str,
        plugin_name: &str,
        plugin_path: &str,
        ip: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name: name.to_string(),
            plugin: Plugin::load(plugin_name, plugin_path)?,
            ip: "127.0.0.1".parse()?,
        })
    }
    fn get_status(&self) -> Result<Value, Box<dyn Error>> {
        self.plugin.get_status()
    }

    fn get_ip(&self) -> IpAddr {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
impl Writable for RWDevice {
    fn set_status(&self, status: &Value) -> Result<Value, Box<dyn std::error::Error>> {
        self.plugin.set_status(status)
    }
}
