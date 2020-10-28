pub mod device_attributes;
pub mod rdevice;
pub mod rwdevice;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use self::{rdevice::RDevice, rwdevice::RWDevice};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum Device {
    ReadOnly(RDevice),
    ReadWrite(RWDevice),
}
impl Device {
    pub fn local_search() -> Result<Vec<IpAddr>, crate::error::Error> {
        todo!()
    }
}