use crate::{device::Device, *};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub(crate) struct DeviceList(Vec<Device>);

impl DeviceList {
    pub fn new(list : Vec<Device>) -> Self{
        Self(list)
    }
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| {
            let err_str = &e.to_string();
            error!(err_str)
        })
    }
    pub fn from_json(json: &str) -> Result<DeviceList, Error> {
        serde_json::from_str(json).map_err(|e| {
            let err_str = &e.to_string();
            error!(err_str)
        })
    }
}
