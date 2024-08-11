use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Dado el path de una imagen (relativo o absoluto),
/// devuelve un booleano que indica si la imagen corresponde
/// o no a un incidente.
pub fn is_incident(image_path: &str) -> bool {
    process_incident_image(image_path).unwrap_or(false)
}

/// Dados los tags de una imagen en un vector de Strings,
/// devuelve un booleano que indica si los tags corresponden
/// o no a un incidente.
fn is_incident_tag(image_tags: &[String]) -> bool {
    let mut crash_tags: Vec<String> = Vec::new();
    let mut fire_tags: Vec<String> = Vec::new();

    crash_tags.append(&mut Vec::<String>::from([
        "land vehicle".to_string(),
        "wheel".to_string(),
        "car".to_string(),
        "vehicle".to_string(),
    ]));
    fire_tags.append(&mut Vec::<String>::from([
        "fire".to_string(),
        "smoke".to_string(),
        "explosion".to_string(),
    ]));

    contains_tags(image_tags, &crash_tags) || contains_tags(image_tags, &fire_tags)
}

fn contains_tags(image_tags: &[String], tags: &Vec<String>) -> bool {
    for tag in tags {
        if !image_tags.contains(tag) {
            return false;
        }
    }
    true
}

fn process_incident_image(image_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Clave de suscripción y endpoint
    let subscription_key = "e6b332c5c2414053a067d5fdfdb9971e";
    let endpoint =
        "https://aifinaltaller.cognitiveservices.azure.com/vision/v3.0/analyze?visualFeatures=Tags";
    const VALID_CONFIDENCE: f64 = 0.50;

    if !Path::new(image_path).exists() {
        return Err(Box::from("Archivo no encontrado"));
    }

    let client = Client::new();
    let image_data: Vec<u8> = fs::read(image_path)?;
    let mut headers = HeaderMap::new();

    headers.insert(
        "Ocp-Apim-Subscription-Key",
        HeaderValue::from_str(subscription_key).unwrap(),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    let response = client
        .post(endpoint)
        .headers(headers)
        .body(image_data)
        .send()?;

    // recoleccion de tags
    let res_text = response.text()?;
    let json: Value = serde_json::from_str(&res_text)?;
    let mut image_tags = Vec::new();
    let mut i = 0;

    let arr_tags = match json["tags"].as_array() {
        Some(r) => r,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Without tags",
            )))
        }
    };

    while i < arr_tags.len() {
        let tag = &json["tags"][i];
        let mut aux = tag["name"].to_string();
        aux = aux[1..(aux.len() - 1)].to_string();

        if tag["confidence"].as_f64() > Some(VALID_CONFIDENCE) {
            image_tags.push(aux);
        }
        i += 1;
    }

    Ok(is_incident_tag(&image_tags))
}

#[cfg(test)]
mod test {
    use crate::vision::vision_ai::is_incident;
    use std::time;

    #[test]
    fn no_incidents() {
        print!("Wait by limit ...");
        std::thread::sleep(time::Duration::from_millis(60000));

        println!("FALSOS POSITIVOS - CALLE:");
        assert!(!is_incident("../data/images/transito/calle1.jpg"));
        assert!(!is_incident("../data/images/transito/calle2.jpg"));
        println!("TRANSITO:");
        assert!(!is_incident("../data/images/transito/transito1.jpg"));
        assert!(!is_incident("../data/images/transito/transito3.jpg"));
        assert!(!is_incident("../data/images/transito/transito4.jpg"));
        assert!(!is_incident("../data/images/transito/transito7.jpg"));
        assert!(!is_incident("../data/images/transito/transito8.jpg"));
        assert!(!is_incident("../data/images/transito/transito9.jpg"));
        assert!(!is_incident("../data/images/transito/transito10.jpg"));
    }

    #[test]
    fn true_incidents() {
        print!("Wait by limit ...");
        std::thread::sleep(time::Duration::from_millis(60000));

        println!("INCIDENTES VERDADEROS - CHOQUES:");
        assert!(is_incident("../data/images/choque/choque1.jpg"));
        assert!(is_incident("../data/images/choque/choque2.jpg"));
        assert!(is_incident("../data/images/choque/choque3.jpg"));
        assert!(is_incident("../data/images/choque/choque6.jpg"));
        assert!(is_incident("../data/images/choque/choque7.jpg"));
        assert!(is_incident("../data/images/choque/choque8.jpg"));
        assert!(is_incident("../data/images/choque/choque9.jpg"));
        assert!(is_incident("../data/images/choque/choque10.jpg"));
        assert!(is_incident("../data/images/choque/choque11.jpg"));
        assert!(is_incident("../data/images/choque/choque12.jpg"));

        print!("Wait by limit ...");
        std::thread::sleep(time::Duration::from_millis(60000));
        println!("VUELCOS:");
        assert!(is_incident("../data/images/choque/vuelco1.jpg"));
        assert!(is_incident("../data/images/choque/vuelco2.jpg"));
        assert!(is_incident("../data/images/choque/vuelco3.jpg"));
        println!("INCENDIOS: ");
        assert!(is_incident("../data/images/otros/incendio1.jpg"));
        assert!(is_incident("../data/images/otros/incendio3.jpg"));
        println!("MANIFESTACION: ");
        assert!(is_incident("../data/images/otros/manifestacion2.jpg"));
    }
}
