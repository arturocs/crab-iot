use crate::error::Error;
use crate::{device::Device, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct DeviceList(Vec<Device>);

impl DeviceList {
    pub fn new(list: Vec<Device>) -> Self {
        Self(list)
    }
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| error!(&e.to_string()))
    }
    pub fn from_json(json: &str) -> Result<DeviceList, Error> {
        serde_json::from_str(json).map_err(|e| error!(&e.to_string()))
    }
}
