use libloading::{Library, Symbol};
use serde_json::Value;
use std::{ffi::CString, os::raw::c_char};

use crate::{error::Error, *};
#[derive(Debug)]
pub(crate) struct Plugin {
    device_name: String,
    dylib: Library,
}

impl Plugin {
    pub(crate) fn get_status(&self) -> Result<Value, Error> {
        let cstr = unsafe {
            let func: Symbol<unsafe extern "C" fn() -> *mut c_char> =
                self.dylib.get(b"get_status").map_err(|e| error!(e))?;
            CString::from_raw(func())
        };
        let s = cstr.to_str().map_err(|e| error!(e))?;
        let json: Value = serde_json::from_str(s).map_err(|e| error!(e))?;
        Ok(json)
    }
    pub(crate) fn set_status(&self, status: &Value) -> Result<Value,  Error> {
        let status_str = CString::new(status.to_string()).unwrap().into_raw();
        let cstr = unsafe {
            let func: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> =
                self.dylib.get(b"set_status").map_err(|e| error!(e))?;
            CString::from_raw(func(status_str))
        };
        let s = cstr.to_str().map_err(|e| error!(e))?;
        let json: Value = serde_json::from_str(s).map_err(|e| error!(e))?;
        Ok(json)
    }
    pub(crate) fn load(name: &str, path: &str) -> Result<Self, Error> {
        Ok(Self {
            device_name: name.to_string(),
            dylib: Library::new(path).map_err(|e| error!(e))?,
        })
    }
}
