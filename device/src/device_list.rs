use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, net::IpAddr};
use utils::{error::Error, *};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceList(BTreeMap<String, IpAddr>);

impl DeviceList {
    pub fn new(list: Vec<(String, IpAddr)>) -> Self {
        Self(list.into_iter().collect())
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| error!(e))
    }
    pub fn from_json(json: &str) -> Result<DeviceList, Error> {
        serde_json::from_str(json).map_err(|e| error!(e))
    }
}
