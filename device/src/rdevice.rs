use super::Readable;
use crate::plugin::Plugin;
use crate::{error::Error, *};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::IpAddr;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RDevice {
    name: String,
    plugin: Plugin,
    ip: IpAddr,
}

impl<'a> Readable<'a> for RDevice {
    fn new(name: &str, plugin_name: &str, plugin_path: &str, ip: &str) -> Result<Self, Error> {
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

    fn get_mut_plugin(&mut self) -> &mut Plugin {
        &mut self.plugin
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
