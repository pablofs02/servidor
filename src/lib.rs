mod clases;

pub use clases::*;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub fn tratar_conexion(mut conexion: TcpStream) {
    let lector = BufReader::new(&mut conexion);
    if let Some(Ok(solicitud)) = lector.lines().next() {
        println!("{solicitud}");
        let (tipo, archivo, mut estatus) = desmontar_solicitud(solicitud);
        estatus.push_str(" 200 OK");
        match &tipo[..] {
            "GET" => solicitud_get(conexion, archivo, estatus),
            _ => solicitud_desconocida(conexion)
        }
    }
}

fn solicitud_get(conexion: TcpStream, mut archivo: String, estatus: String) {
    if archivo == "" {
        archivo.push('.');
    }
    match fs::metadata(&archivo).ok() {
        Some(metadata) => {
            if metadata.is_dir() {
                archivo.push_str("/index.html");
            }
        }
        None => ()
    }
    match fs::read(&archivo) {
        Ok(contenido) => dar_respuesta(conexion, estatus, archivo, contenido),
        Err(_) => error_404(conexion)
    }
}

fn error_404(conexion: TcpStream) {
    let archivo = String::from("404.html");
    let estatus = "HTTP/1.1 404 Not Found".to_string();
    match fs::read(&archivo) {
        Ok(contenido) => dar_respuesta(conexion, estatus, archivo, contenido),
        Err(_) => dar_respuesta(conexion, estatus, archivo, e404().into())
    }
}

fn e404() -> String {
    return String::from("<!DOCTYPE html><html lang=\"es\"><head><meta charset=\"utf-8\"><title>Error 404</title><style>*{background-color: #222;color: #DDD;text-decoration: none;}</style></head><body><a href=\"/\"><h1>Error 404</h1><p>La página web que estás buscando no está aquí.</p></a></body></html>");
}

fn solicitud_desconocida(mut conexion: TcpStream) {
    let respuesta = format!("HTTP/1.1 501 Not Implemented");
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}

fn dar_respuesta(mut conexion: TcpStream, estatus: String, archivo: String, contenido: Vec<u8>) {
    let longitud = contenido.len();
    let tipo = sacar_tipo(&archivo).to_string();
    let respuesta = format!("{estatus}\r\nContent-Type: {tipo}\r\nContent-Length: {longitud}\r\n\r\n");
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.write_all(&contenido).unwrap();
    conexion.flush().unwrap();
}

fn desmontar_solicitud(solicitud: String) -> (String, String, String) {
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
            estado += 1;
        } else if estado == 2 {
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

fn sacar_tipo(archivo: &String) -> &str {
    let tipo = sacar_extension(&archivo);
    match &tipo[..] {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml; charset=utf-8",
        "ogg" | "oga" => "audio/ogg",
        "mp3" | "mpeg" => "audio/mpeg",
        "mp4" => "video/mp4",
        "gif" => "image/gif",
        "ogv" => "video/ogg",
        "zip" => "application/zip",
        "weba" | "webm" => "audio/webm",
        "webp" => "image/webp",
        "otf" => "font/otf",
        "ttf" => "font/ttf",
        _ => ""
    }
}

fn sacar_extension(archivo: &String) -> String {
    let mut tipo = String::new();
    for c in archivo.chars() {
        if c != '.' {
            tipo.push(c);
        } else {
            tipo = String::new();
        }
    }
    tipo
}
