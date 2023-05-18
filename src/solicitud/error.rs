use std::{net::TcpStream, io::Write, fs};
use crate::solicitud::dar_respuesta;

pub fn movido_301(mut conexion: TcpStream, ruta: &str) {
    let respuesta = format!("HTTP/1.1 301 Moved Permanently\r\nContent-Type: text/html\r\nLocation: {ruta}\r\n\r\n");
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}

pub fn no_encontrado_404(conexion: TcpStream, archivo: &str) {
    conexion.peer_addr().map_or_else(|_| println!("\x1b[31m{archivo}\x1b[0m"), |dir| println!("[{}] \x1b[31m{archivo}\x1b[0m", dir.ip()));
    let archivo = String::from("404.html");
    let estatus = "HTTP/1.1 404 Not Found".to_string();
    match fs::read(&archivo) {
        Ok(contenido) => dar_respuesta(conexion, &estatus, &archivo, &contenido),
        Err(_) => dar_respuesta(conexion, &estatus, &archivo, e404().as_bytes())
    }
}

fn e404() -> String {
    String::from("<!DOCTYPE html><html lang=\"es\"><head><meta charset=\"utf-8\"><title>Error 404</title><style>*{background-color: #222;color: #DDD;text-decoration: none;}</style></head><body><a href=\"/\"><h1>Error 404</h1><p>La página web que estás buscando no está aquí.</p></a></body></html>")
}
