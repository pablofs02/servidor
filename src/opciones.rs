use std::process::exit;

#[derive(Debug, Copy, Clone)]
pub struct Opciones {
    pub local: bool,
    pub verboso: bool,
    pub puerto: u16,
}

impl Default for Opciones {
    fn default() -> Self {
        Self {
            local: true,
            verboso: false,
            puerto: 9999,
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

    fn tratar_caracteres(&mut self, caracteres: &str) {
        for letra in caracteres.chars() {
            match letra {
                'l' => self.local = true,
                'g' => self.local = false,
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

fn mensaje_de_ayuda() {
    println!(
        "Modo de empleo: servidor [OPCIONES...]
    l    servidor local
    g    servidor global
    v    más información"
    );
}
