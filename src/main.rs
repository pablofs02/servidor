extern crate if_addrs;
extern crate webbrowser;
use servidor::{abrir_servidor_http, Opciones};
use std::env::{args, set_current_dir};
use std::process::exit;

fn main() {
    set_current_dir("/home/servidor/").unwrap();
    let mut opciones = Opciones { verboso: false, local: true };
    let argumentos: Vec<String> = args().collect();
    for argumento in &argumentos {
        if &argumento[..] == argumentos.get(0).unwrap() || argumento.is_empty() {
            continue;
        }
        // -v=ruta / -v ruta / v=ruta / v ruta
        // Formato UNIX '-v' o BSD 'v'
        if &argumento[0..1] == "-" {
            opciones = tratar_caracteres(&argumento[1..], opciones);
        } else {
            opciones = tratar_caracteres(argumento, opciones);
        }
    }
    abrir_servidor_http(opciones);
}

fn tratar_caracteres(caracteres: &str, mut opciones: Opciones) -> Opciones {
    for letra in caracteres.chars() {
        match letra {
            'v' => opciones.verboso = true,
            'p' | 'g' => opciones.local = false,
            'l' => opciones.local = true,
            // seleccionar directorio base
            'd' => (),
            _ => {
                eprintln!("Argumento desconocido: '{letra}'");
                exit(22);
            }
        }
    }
    opciones
}
