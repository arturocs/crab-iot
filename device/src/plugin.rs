use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use utils::{error::Error, *};
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
        if let Some(lib) = &self.dylib {
            let status = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const Value) -> *mut Result<Value, Error>> =
                    lib.get(b"get_status").map_err(|e| error!(e))?;
                Box::from_raw(func(query)).map_err(|e| error!(e))?
            };
            Ok(status)
        } else {
            Err(error!("Unloaded plugin, call reload_after_deserialize()"))
        }
    }

    pub(crate) fn set_status(&self, status: &Value) -> Result<Value, Error> {
        if let Some(lib) = &self.dylib {
            let plugin_answer = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const Value) -> *mut Result<Value, Error>> =
                    lib.get(b"set_status").map_err(|e| error!(e))?;
                Box::from_raw(func(status)).map_err(|e| error!(e))?
            };
            Ok(plugin_answer)
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
