extern crate if_addrs;
extern crate webbrowser;
use servidor::{abrir_servidor_http, Opciones};
use std::env::args;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

fn main() {
    let mut registro = OpenOptions::new().create(true).write(true).append(true).open("registro.pfs").unwrap();
    if let Err(e) = writeln!(registro, "¡Información!") {
        eprintln!("No se pudo escribir en registro: {e}");
    }
    let mut opciones = Opciones {
        ayuda: false,
        local: true,
        verboso: false
    };
    opciones = tratar_argumentos(opciones);
    if opciones.ayuda {
        mensaje_de_ayuda();
    } else {
        abrir_servidor_http(opciones);
    }
}

fn tratar_argumentos(mut opciones: Opciones) -> Opciones {
    let argumentos: Vec<String> = args().collect();
    for argumento in &argumentos {
        // Saltar nombre del comando o cadena vacía
        if &argumento[..] == argumentos.get(0).unwrap() || argumento.is_empty() {
            continue;
        }
        // Formato UNIX '-v' o BSD 'v'
        if let Some(argumento) = argumento.strip_prefix('-') {
            opciones = tratar_caracteres(argumento, opciones);
        } else {
            opciones = tratar_caracteres(argumento, opciones);
        }
    }
    opciones
}

fn tratar_caracteres(caracteres: &str, mut opciones: Opciones) -> Opciones {
    for letra in caracteres.chars() {
        match letra {
            'o' => opciones.ayuda = true,
            'l' => opciones.local = true,
            'p' => opciones.local = false,
            'v' => opciones.verboso = true,
            _ => {
                eprintln!("Argumento desconocido: '{letra}'");
                mensaje_de_ayuda();
                exit(22);
            }
        }
    }
    opciones
}

fn mensaje_de_ayuda() {
    println!(
        "Modo de empleo: servidor [OPCIONES...]
    o    mostrar opciones
    l    servidor local (por defecto)
    p    servidor público
    v    más información"
    );
}
