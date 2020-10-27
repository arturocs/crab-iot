use crate::error::Error;
use crate::plugin::Plugin;
use std::net::IpAddr;

pub(crate) struct Dispositivo {
    nombre: String,
    activo: bool,
    solo_lectura: bool,
    plugin: Plugin,
    ip: IpAddr,
}

impl Dispositivo {
    fn buscar() -> Result<Vec<IpAddr>, Error<String>> {
        todo!()
    }
    fn nuevo(
        nombre: &str,
        solo_lectura: bool,
        ip: IpAddr,
        plugin: Plugin,
    ) -> Result<Dispositivo, Error<String>> {
        todo!()
    }
    fn encender(&self) -> Result<(), Error<String>> {
        todo!()
    }
    fn apagar(&self) -> Result<(), Error<String>> {
        todo!()
    }
    fn obtener_estado(&self) -> Result<String, Error<String>> {
        todo!()
    }
    fn cambiar_estado(&self, estado: &str) -> Result<(), Error<String>> {
        todo!()
    }
}
