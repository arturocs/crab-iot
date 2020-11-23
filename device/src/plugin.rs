use crate::{error::Error, *};
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{ffi::CString, os::raw::c_char, path::PathBuf};
#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    device_name: String,
    #[serde(skip_serializing, skip_deserializing)]
    dylib: Option<Library>,
    libary_path: PathBuf,
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.libary_path == other.libary_path && self.device_name == other.device_name
    }
}

impl Plugin {
    pub(crate) fn get_status(&self, query: &Value) -> Result<Value, Error> {
        let query_str = CString::new(query.to_string()).unwrap().into_raw();
        if let Some(lib) = &self.dylib {
            let cstr = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> =
                    lib.get(b"get_status").map_err(|e| error!(e))?;
                CString::from_raw(func(query_str))
            };
            let s = cstr.to_str().map_err(|e| error!(e))?;
            let json: Value = serde_json::from_str(s).map_err(|e| error!(e))?;
            Ok(json)
        } else {
            Err(error!("Unloaded plugin, reload_after_deserialize()"))
        }
    }
    pub(crate) fn set_status(&self, status: &Value) -> Result<Value, Error> {
        let status_str = CString::new(status.to_string()).unwrap().into_raw();
        if let Some(lib) = &self.dylib {
            let cstr = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> =
                    lib.get(b"set_status").map_err(|e| error!(e))?;
                CString::from_raw(func(status_str))
            };
            let s = cstr.to_str().map_err(|e| error!(e))?;
            let json: Value = serde_json::from_str(s).map_err(|e| error!(e))?;
            Ok(json)
        } else {
            Err(error!("Unloaded plugin, call reload_after_deserialize()"))
        }
    }

    pub(crate) fn load(name: &str, path: &str) -> Result<Self, Error> {
        Ok(Self {
            device_name: name.to_string(),
            dylib: Some(Library::new(path).map_err(|e| error!(e))?),
            libary_path: PathBuf::from(path),
        })
    }

    pub fn reload_after_deserialize(&mut self) -> Result<(), Error> {
        self.dylib = Some(Library::new(&self.libary_path).map_err(|e| error!(e))?);
        Ok(())
    }
}
