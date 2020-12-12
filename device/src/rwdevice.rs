use super::{Readable, Writable};
use crate::plugin::Plugin;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use utils::{error::Error, *};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RWDevice {
    name: String,
    plugin: Plugin,
    ip: IpAddr,
}

impl<'a> Readable<'a> for RWDevice {
    fn new(name: &str, plugin_name: &str, plugin_path: &str, ip: &str) -> Result<Self, Error> {
        Ok(Self {
            name: name.to_string(),
            plugin: Plugin::load(plugin_name, plugin_path).map_err(|e| error!(e))?,
            ip: "127.0.0.1".parse().map_err(|e| error!(e))?,
        })
    }

    fn get_ip(&self) -> IpAddr {
        self.ip
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_plugin(&self) -> &Plugin {
        &self.plugin
    }

    fn get_mut_plugin(&mut self) -> &mut Plugin {
        &mut self.plugin
    }
}
impl<'a> Writable<'a> for RWDevice {}
