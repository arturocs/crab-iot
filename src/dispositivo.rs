use crate::error::Error;
use crate::plugin::Plugin;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Dispositivo {
    nombre: String,
    activo: bool,
    solo_lectura: bool,
    plugin: Plugin,
    ip: IpAddr,
}

impl Dispositivo {
    fn buscar() -> Result<Vec<IpAddr>, Error> {
        todo!()
    }
    fn nuevo(
        nombre: &str,
        solo_lectura: bool,
        ip: IpAddr,
        plugin: Plugin,
    ) -> Result<Dispositivo, Error> {
        todo!()
    }
    fn encender(&self) -> Result<(), Error> {
        todo!()
    }
    fn apagar(&self) -> Result<(), Error> {
        todo!()
    }
    fn obtener_estado(&self) -> Result<String, Error> {
        todo!()
    }
    fn cambiar_estado(&self, estado: &str) -> Result<(), Error> {
        todo!()
    }
}
