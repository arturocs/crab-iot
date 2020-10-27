use crate::device::Device;
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DeviceList(Vec<Device>);

impl DeviceList {
    fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| {
            let err_str = &e.to_string();
            error!(err_str)
        })
    }
    fn from_json(json: &str) -> Result<String, Error> {
        serde_json::from_str(json).map_err(|e| {
            let err_str = &e.to_string();
            error!(err_str)
        })
    }
}
