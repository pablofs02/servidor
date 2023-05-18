extern crate if_addrs;
extern crate webbrowser;
mod hebras;
mod opciones;
mod registro;
mod solicitud;
use hebras::Piscina;
pub use opciones::Opciones;
use solicitud::tratar;
use registro::Registro;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

pub fn iniciar_servidor_http(opciones: Opciones) {
    let dir: SocketAddr = sacar_dir(opciones);
    let entrada: TcpListener = TcpListener::bind(dir).expect("No se pudo iniciar el puerto");
    let mut registro = Registro::iniciar();
    registro.escribir("¡Servidor iniciado!");
    if opciones.local {
        abrir_en_navegador(dir.to_string().as_str());
    }
    let piscina = Piscina::new(16);
    for conexion in entrada.incoming() {
        let conexion = conexion.expect("Conexión incorrecta");
        piscina.arrancar(move || {
            tratar_conexion(conexion, opciones);
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
    webbrowser::open(&url[..]).unwrap();
}

fn tratar_conexion(mut conexion: TcpStream, opciones: Opciones) {
    let lector = BufReader::new(&mut conexion);
    if let Some(Ok(solicitud)) = lector.lines().next() {
        solicitud(conexion, &solicitud, opciones);
    }
}

fn dir_privada() -> IpAddr {
    let direcciones = if_addrs::get_if_addrs().unwrap();
    direcciones.get(1).unwrap().ip()
}
