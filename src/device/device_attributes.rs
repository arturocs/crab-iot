use crate::plugin::Plugin;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct DeviceAttributes {
    pub(crate) name: String,
    pub(crate) active: bool,
    pub(crate) plugin: Option<Plugin>,
    pub(crate) ip: IpAddr,
}
