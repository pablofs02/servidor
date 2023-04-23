mod clases;
pub use clases::*;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

pub fn abrir_servidor_http(opciones: Opciones) {
    let puerto: TcpListener;
    let dir: String;
    let num_puerto = "9999";
    if opciones.local {
        dir = "127.0.0.1".to_owned() + ":" + num_puerto;
    } else {
        dir = dir_privada() + ":" + num_puerto;
    }
    puerto = TcpListener::bind(&dir).expect("No se pudo iniciar el puerto");
    if opciones.navegador || opciones.local {
        abrir_en_navegador(&dir);
    }
    let piscina = Piscina::new(16);
    for conexion in puerto.incoming() {
        let conexion = conexion.expect("Conexión incorrecta");
        piscina.execute(move || {
            tratar_conexion(conexion, opciones);
        });
    }
}

fn abrir_en_navegador(dir: &String) {
    let url = "http://".to_owned() + &dir[..];
    webbrowser::open(&url[..]).unwrap();
}

fn tratar_conexion(mut conexion: TcpStream, opciones: Opciones) {
    let lector = BufReader::new(&mut conexion);
    if let Some(Ok(solicitud)) = lector.lines().next() {
        if opciones.verboso {
            match conexion.peer_addr() {
                Ok(dir) => println!("[{}] {solicitud}", dir.ip()),
                Err(_) => println!("{solicitud}")
            }
        }
        let (tipo, archivo, mut estatus) = desmontar_solicitud(&solicitud);
        estatus.push_str(" 200 OK");
        match &tipo[..] {
            "GET" => solicitud_get(conexion, archivo, &estatus),
            _ => solicitud_desconocida(conexion)
        }
    }
}

fn solicitud_get(conexion: TcpStream, mut archivo: String, estatus: &str) {
    if archivo.is_empty() {
        archivo.push('.');
    }
    fs::metadata(&archivo).ok().map_or((), |metadata| {
        if metadata.is_dir() {
            archivo.push_str("/index.html");
        }
    });
    match fs::read(&archivo) {
        Ok(contenido) => dar_respuesta(conexion, estatus, &archivo, &contenido),
        Err(_) => error_404(conexion)
    }
}

fn error_404(conexion: TcpStream) {
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

fn solicitud_desconocida(mut conexion: TcpStream) {
    let respuesta = "HTTP/1.1 501 Not Implemented";
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}

fn dar_respuesta(mut conexion: TcpStream, estatus: &str, archivo: &str, contenido: &[u8]) {
    let longitud = contenido.len();
    let tipo = sacar_tipo(archivo).to_string();
    let respuesta = format!("{estatus}\r\nContent-Type: {tipo}\r\nContent-Length: {longitud}\r\n\r\n");
    conexion.write_all(respuesta.as_bytes()).unwrap();
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

fn sacar_tipo(archivo: &str) -> &str {
    let tipo = sacar_extension(archivo);
    match &tipo[..] {
        "css" => "text/css",
        "gif" => "image/gif",
        "html" => "text/html",
        "jpg" | "jpeg" => "image/jpeg",
        "js" => "application/javascript",
        "json" => "application/json",
        "mp3" | "mpeg" => "audio/mpeg",
        "mp4" => "video/mp4",
        "pdf" => "application/fdf",
        "png" => "image/png",
        "svg" => "image/svg+xml; charset=utf-8",
        "obj" => "model/obj",
        "ogg" | "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "otf" => "font/otf",
        "ttf" => "font/ttf",
        "weba" | "webm" => "audio/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "zip" => "application/zip",
        _ => ""
    }
}

fn sacar_extension(archivo: &str) -> String {
    let mut tipo = String::new();
    for c in archivo.chars() {
        if c == '.' {
            tipo = String::new();
        } else {
            tipo.push(c);
        }
    }
    tipo
}

fn dir_privada() -> String {
    let direcciones = if_addrs::get_if_addrs().unwrap();
    direcciones.get(1).unwrap().ip().to_string()
}
