use std::process::exit;

#[derive(Debug, Copy, Clone)]
pub struct Opciones {
    pub local: bool,
    pub verboso: bool,
    pub puerto: u16
}

impl Default for Opciones {
    fn default() -> Self {
        Self {
            local: true,
            verboso: false,
            puerto: 9999
        }
    }
}

impl Opciones {
    pub fn configurar(&mut self, argumentos: &[String]) {
        for argumento in argumentos {
            if es_comando(argumentos, argumento) || argumento.is_empty() {
                continue;
            }
            // Formato UNIX '-v' o BSD 'v'
            if let Some(argumento) = argumento.strip_prefix('-') {
                self.tratar_caracteres(argumento);
            } else {
                self.tratar_caracteres(argumento);
            }
        }
    }

    fn tratar_caracteres(&mut self, caracteres: &str) {
        for letra in caracteres.chars() {
            match letra {
                'o' => {
                    mensaje_de_ayuda();
                    exit(0);
                }
                'l' => self.local = true,
                'p' => self.local = false,
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

fn es_comando(argumentos: &[String], argumento: &String) -> bool {
    argumento == argumentos.get(0).expect("No está el comando en los argumentos")
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
