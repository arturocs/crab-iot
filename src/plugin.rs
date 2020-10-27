use crate::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Plugin {
    device_name: String,
    device_id: String,
    plugin_path: PathBuf,
}

impl Plugin {
    fn load(nombre: &str, ruta: &str) -> Plugin {
        todo!()
    }
    fn execute(&self) -> Result<(), Error> {
        todo!()
    }
    fn stop_execution(&self) -> Result<(), Error> {
        todo!()
    }
}
