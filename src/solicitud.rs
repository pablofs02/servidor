mod error;
mod get;
mod head;
mod tipo;

use crate::Opciones;
use std::io::prelude::Write;
use std::io::stdout;
use std::net::TcpStream;
use urlencoding::decode;

pub fn tratar(conexion: TcpStream, solicitud: &str, opciones: Opciones) {
    let (tipo, archivo, estatus) = desmontar_solicitud(solicitud);
    let archivo = decodificar_archivo(&archivo);
    if opciones.verboso {
        conexion.peer_addr().map_or_else(
            |_| println!("{tipo} {archivo} {estatus}"),
            |dir| println!("[{}] {tipo} {archivo} {estatus}", dir.ip()),
        );
    }
    stdout().flush().unwrap();
    let estatus = "HTTP/1.1 200 OK".to_owned();
    match &tipo[..] {
        "GET" => get::solicitar(conexion, archivo, &estatus),
        "HEAD" => head::solicitar(conexion, archivo, &estatus),
        _ => solicitud_desconocida(conexion),
    }
}

fn decodificar_archivo(archivo: &str) -> String {
    let archivo = decode(archivo).unwrap();
    let mut arc = vec![];
    let mut nivel = 0;
    for dir in archivo.split('/') {
        if dir == ".." {
            nivel -= 1;
            arc.pop();
        } else if dir == "." {
        } else {
            nivel += 1;
            arc.push(dir);
        }
        if nivel < 0 {
            arc = vec![];
        }
    }
    arc.join("/")
}

fn solicitud_desconocida(mut conexion: TcpStream) {
    let respuesta = "HTTP/1.1 501 Not Implemented\r\n";
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}

fn dar_respuesta(mut conexion: TcpStream, estatus: &str, archivo: &str, contenido: &[u8]) {
    let longitud = contenido.len();
    let tipo = tipo::sacar(archivo).to_string();
    let respuesta =
        format!("{estatus}\r\nContent-Type: {tipo}\r\nContent-Length: {longitud}\r\n\r\n");
    conexion.write_all(respuesta.as_bytes()).unwrap();
    // En esta línea pasa algo cuando intento acceder a un archivo que no existe.
    // Se produce cuando la petición no acaba en \n o \r\n.
    conexion.write_all(contenido).unwrap();
    conexion.flush().unwrap();
}

fn desmontar_solicitud(solicitud: &str) -> (String, String, String) {
    let mut estado = 0;
    let mut tipo = String::new();
    let mut contenido = String::new();
    let mut version = String::new();
    for c in solicitud.chars() {
        if estado == 0 {
            if c == ' ' {
                estado += 1;
            } else {
                tipo.push(c);
            }
        } else if estado == 1 {
            if c == ' ' {
                estado += 1;
            } else {
                contenido.push(c);
            }
        } else {
            version.push(c);
        }
    }
    (tipo, contenido, version)
}
