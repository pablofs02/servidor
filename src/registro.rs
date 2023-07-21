use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Registro {
    archivo: Option<File>,
}

impl Registro {
    pub fn iniciar() -> Self {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("registro.ldf")
            .map_or_else(
                |_| {
                    eprintln!("Error al abrir el registro");
                    eprintln!("No se escribirá en registro");
                    Self { archivo: None }
                },
                |archivo| Self {
                    archivo: Some(archivo),
                },
            )
    }

    pub fn notificar(&mut self) {
        self.escribir("¡Servus iniciado!");
    }

    pub fn solicitud(&mut self, ip: &IpAddr, solicitud: &str) {
        let tempus = current_tempus();
        let texto = format!("|{tempus}|{ip}|{solicitud}|");
        self.escribir(&texto);
    }

    pub fn escribir(&mut self, texto: &str) {
        if let Some(archivo) = &mut self.archivo {
            if let Err(e) = writeln!(archivo, "{texto}") {
                eprintln!("Error al escribir en el registro: {e}");
            }
        }
    }
}

fn current_tempus() -> String {
    let mut secunda = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("No se pudo obtener el tiempo del sistema")
        .as_secs();
    let mut minutis = secunda / 60;
    let mut horis = minutis / 60;
    let mut dies = horis / 24;
    secunda = secunda % 60;
    minutis = minutis % 60;
    horis = horis % 24;

    // +/4 -/100 +/400
    let annus = dies / 365 + 1970;
    let mut dies_r = dies % 365;
    let mut mensis = 1;
    let mut dem = 31;
    while dies_r >= dem {
        dies_r -= dem;
        mensis += 1;
        dem = match mensis {
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
    }
    dies = dies / 365 - (dies / 365 / 4) - dem + 1;

    // Cambio de hora (CEST)
    if mensis > 3 && mensis < 10 || mensis == 10 && dies < 29 || mensis == 3 && dies >= 2 {
        horis += 2;
    } else {
        horis += 1;
    }

    format!("{annus:^02}-{mensis:^02}-{dies:^02}|{horis:^02}:{minutis:^02}:{secunda:^02}")
}
