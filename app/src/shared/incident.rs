use super::coordenates::Coordenates;

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

pub fn serialize_incident(incident: Incident) -> Vec<u8> {
    let mut bytes = Vec::new();

    let id_len: u16 = incident.id.len() as u16;
    bytes.extend_from_slice(id_len.to_be_bytes().as_ref());
    bytes.extend_from_slice(incident.id.as_bytes());

    bytes.extend_from_slice(incident.location.latitude.to_be_bytes().as_ref());
    bytes.extend_from_slice(incident.location.longitude.to_be_bytes().as_ref());

    let state = match incident.state {
        IncidentState::InProgess => 0,
        IncidentState::Resolved => 1,
    };

    bytes.push(state);

    bytes
}

pub fn deserialize_incident(bytes: Vec<u8>) -> Incident {
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
