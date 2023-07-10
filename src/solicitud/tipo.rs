pub fn sacar(archivo: &str) -> &str {
    let tipo = sacar_extension(archivo);
    match &tipo[..] {
        "css" => "text/css; charset=utf-8",
        "gif" => "image/gif",
        "html" => "text/html; charset=utf-8",
        "jpg" | "jpeg" => "image/jpeg",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
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
        _ => "text/plain; charset=utf-8",
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

#[cfg(test)]
mod archivo {
    use super::*;

    #[test]
    fn simple() {
        let archivo = "datos.csv";
        let esperado = "csv".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }

    #[test]
    fn sin_tipo() {
        let archivo = "datos";
        let esperado = "".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }

    #[test]
    fn archivo_oculto() {
        let archivo = ".datos.csv";
        let esperado = "csv".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }

    #[test]
    fn varios_tipos() {
        let archivo = "datos.ldf.csv";
        let esperado = "csv".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }

    #[test]
    fn dos_punto() {
        let archivo = "datos..csv";
        let esperado = "csv".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }
}

#[cfg(test)]
mod mime {
    use super::*;

    #[test]
    fn buen_tipo() {
        let archivo = "funciones.js";
        let esperado = "application/javascript".to_string();
        let obtenido = sacar(archivo);
        assert_eq!(esperado, obtenido);
    }
}
