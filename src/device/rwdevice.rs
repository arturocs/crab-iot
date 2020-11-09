use crate::plugin::Plugin;
use serde_json::Value;
use crate::{error::Error, *};
use super::{Readable, Writable};
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
    ) -> Result<Self,  Error> {
        Ok(Self {
            name: name.to_string(),
            plugin: Plugin::load(plugin_name, plugin_path).map_err(|e| error!(e))?,
            ip: "127.0.0.1".parse().map_err(|e| error!(e))?,
        })
    }
    fn get_status(&self) -> Result<Value, Error> {
        self.plugin.get_status()
    }

    fn get_ip(&self) -> IpAddr {
        *&self.ip
    }

    fn get_name(&self) -> String {
        (&self).name.clone()
    }
}
impl Writable for RWDevice {
    fn set_status(&self, status: &Value) -> Result<Value, Error> {
        self.plugin.set_status(status)
    }
}
