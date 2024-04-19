extern crate if_addrs;
extern crate open;

mod hebras;
mod opciones;
mod registro;
mod solicitud;

use hebras::Piscina;
pub use opciones::Opciones;
use registro::Registro;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

pub fn iniciar_servidor_http(opciones: Opciones) {
    let dir: SocketAddr = opciones.sacar_dir();
    let entrada: TcpListener = TcpListener::bind(dir).expect("No se pudo iniciar el puerto");
    let piscina = Piscina::new(16);
    let registro = Arc::new(Mutex::new(Registro::iniciar()));
    for conexion in entrada.incoming().flatten() {
        let registro = Arc::clone(&registro);
        piscina.ejecutar(move || {
            let ip = conexion.peer_addr().unwrap().ip();
            let lector = BufReader::new(&conexion);
            if let Some(Ok(solicitud)) = lector.lines().next() {
                if opciones.registro {
                    registro.lock().unwrap().solicitud(&ip, &solicitud);
                }
                solicitud::tratar(conexion, &solicitud, opciones);
            }
        });
    }
}
