use crate::models::coordenates::Coordenates;

#[derive(Debug, PartialEq, Clone)]
pub enum IncidentState {
    InProgess,
    Resolved,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Incident {
    pub id: String,
    pub location: Coordenates,
    pub state: IncidentState,
}

impl Incident {

    /// ## as_bytes
    /// 
    /// Convierte el incidente a un vector de bytes
    /// 
    /// ### Retorno
    /// - `Vec<u8>`: Vector de bytes
    /// 
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let id_len: u16 = self.id.len() as u16;
        bytes.extend_from_slice(id_len.to_be_bytes().as_ref());
        bytes.extend_from_slice(self.id.as_bytes());

        bytes.extend_from_slice(self.location.latitude.to_be_bytes().as_ref());
        bytes.extend_from_slice(self.location.longitude.to_be_bytes().as_ref());

        let state = match self.state {
            IncidentState::InProgess => 0,
            IncidentState::Resolved => 1,
        };

        bytes.push(state);

        bytes
    }

    /// ## from_be_bytes
    /// 
    /// Convierte un vector de bytes en un incidente
    /// 
    /// ### Parametros
    /// - `bytes`: Vector de bytes
    /// 
    /// ### Retorno
    /// - `Incident`: Incidente creado
    /// 
    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
        let mut index = 0;

        let id_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as usize;
        index += 2;

        let id = String::from_utf8(bytes[index..index + id_len].to_vec()).unwrap();
        index += id_len;

        let latitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let longitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let state = match bytes[index] {
            0 => IncidentState::InProgess,
            1 => IncidentState::Resolved,
            _ => panic!("Invalid state"),
        };

        Incident {
            id,
            location: Coordenates {
                latitude,
                longitude,
            },
            state,
        }
    }
}
