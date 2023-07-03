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
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

pub fn iniciar_servidor_http(opciones: Opciones) {
    let dir: SocketAddr = sacar_dir(opciones);
    let entrada: TcpListener = TcpListener::bind(dir).expect("No se pudo iniciar el puerto");
    let registro = Arc::new(Mutex::new(Registro::iniciar()));
    registro.lock().unwrap().escribir("¡Servidor iniciado!");
    if opciones.local {
        abrir_en_navegador(dir.to_string().as_str());
    }
    let piscina = Piscina::new(16);
    for conexion in entrada.incoming() {
        let conexion = conexion.expect("Conexión incorrecta");
        let registro = Arc::clone(&registro);
        piscina.arrancar(move || {
            let ip = conexion.peer_addr().unwrap().ip();
            let lector = BufReader::new(&conexion);
            if let Some(Ok(solicitud)) = lector.lines().next() {
                registro.lock().unwrap().escribir(&format!("[{ip}] {solicitud}"));
                solicitud::tratar(conexion, &solicitud, opciones);
            }
        });
    }
}

fn sacar_dir(opciones: Opciones) -> SocketAddr {
    if opciones.local {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), opciones.puerto)
    } else {
        SocketAddr::new(dir_privada(), opciones.puerto)
    }
}

fn abrir_en_navegador(dir: &str) {
    let url = "http://".to_owned() + dir;
    if open::that(&url[..]).is_err() {
        eprintln!("No se pudo abrir en navegador");
    }
}

fn dir_privada() -> IpAddr {
    let direcciones = if_addrs::get_if_addrs().expect("Error al sacar direcciones ip");
    direcciones
        .get(1)
        .expect("Error al detectar la dirección privada")
        .ip()
}
