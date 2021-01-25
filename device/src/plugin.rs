use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    device_name: String,
    #[serde(skip_serializing, skip_deserializing)]
    dylib: Option<Library>,
    libary_path: PathBuf,
}
impl Clone for Plugin {
    fn clone(&self) -> Self {
        Self::load(
            self.device_name.as_str(),
            self.libary_path.to_str().unwrap(),
        )
        .unwrap()
    }
}
impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.libary_path == other.libary_path && self.device_name == other.device_name
    }
}

impl Plugin {
    pub(crate) fn get_status(&self, query: &Value) -> Result<Value, String> {
        if let Some(lib) = &self.dylib {
            let status = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const Value) -> *mut Result<Value, String>> =
                    lib.get(b"get_status").map_err(|e| e.to_string())?;
                Box::from_raw(func(query)).map_err(|e| e.to_string())?
            };
            Ok(status)
        } else {
            Err("Unloaded plugin, call reload_after_deserialize()".to_string())
        }
    }

    pub(crate) fn set_status(&self, status: &Value) -> Result<Value, String> {
        if let Some(lib) = &self.dylib {
            let plugin_answer = unsafe {
                let func: Symbol<unsafe extern "C" fn(*const Value) -> *mut Result<Value, String>> =
                    lib.get(b"set_status").map_err(|e| e.to_string())?;
                Box::from_raw(func(status)).map_err(|e| e.to_string())?
            };
            Ok(plugin_answer)
        } else {
            Err("Unloaded plugin, call reload_after_deserialize()".to_string())
        }
    }

    pub(crate) fn load(name: &str, path: &str) -> Result<Self, String> {
        Ok(Self {
            device_name: name.to_string(),
            dylib: Some(Library::new(path).map_err(|e| e.to_string())?),
            libary_path: PathBuf::from(path),
        })
    }

    pub fn reload_after_deserialize(&mut self) -> Result<(), String> {
        self.dylib = Some(Library::new(&self.libary_path).map_err(|e| e.to_string())?);
        Ok(())
    }
}
