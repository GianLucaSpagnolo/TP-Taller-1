use std::{fmt::Display, io::Error};

#[derive(Default, Clone, PartialEq, Debug)]

pub struct Coordenates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordenates {
    pub fn from_strings(lat: &str, long: &str) -> Result<Self, Error> {
        let (latitude, longitude) = match (lat.parse(), long.parse()) {
            (Ok(lat), Ok(long)) => (lat, long),
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error - No se pudo parsear alguna coordenada",
                ))
            }
        };
        Ok(Coordenates {
            latitude,
            longitude,
        })
    }
}

impl Display for Coordenates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(lat: {}, long: {})", self.latitude, self.longitude)
    }
}
