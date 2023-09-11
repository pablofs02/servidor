use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
};

#[derive(Clone, Copy)]
pub struct Opciones {
    pub local: bool,
    pub registro: bool,
    pub verboso: bool,
    pub errores: bool,
    pub puerto: u16,
}

impl Default for Opciones {
    fn default() -> Self {
        Self {
            local: true,
            registro: false,
            verboso: false,
            errores: false,
            puerto: 1492,
        }
    }
}

impl Opciones {
    pub fn configurar(&mut self, argumentos: &[String]) {
        if let Some(argumentos) = argumentos.get(1) {
            // Formato UNIX '-v' o BSD 'v'
            if let Some(argumento) = argumentos.strip_prefix('-') {
                self.tratar_caracteres(argumento);
            } else {
                self.tratar_caracteres(argumentos);
            }
        } else {
            mensaje_de_ayuda();
            exit(0);
        }
    }

    pub fn sacar_dir(&self) -> SocketAddr {
        if self.local {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), self.puerto)
        } else {
            SocketAddr::new(dir_privada(), self.puerto)
        }
    }

    fn tratar_caracteres(&mut self, caracteres: &str) {
        for letra in caracteres.chars() {
            match letra {
                'l' => self.local = true,
                'g' => self.local = false,
                'r' => self.registro = true,
                'e' => self.errores = true,
                'v' => self.verboso = true,
                _ => {
                    eprintln!("Argumento desconocido: '{letra}'");
                    mensaje_de_ayuda();
                    exit(22);
                }
            }
        }
    }
}

fn dir_privada() -> IpAddr {
    let direcciones = if_addrs::get_if_addrs().expect("Error al sacar direcciones ip");
    direcciones
        .get(1)
        .expect("Error al detectar la dirección privada")
        .ip()
}

fn mensaje_de_ayuda() {
    println!(
        "Modo de empleo: servidor <OPCIONES>
    l    abrir en local
    g    abrir en global
    r    registrar peticiones
    e    mostrar errores
    v    mostrar peticiones"
    );
}
