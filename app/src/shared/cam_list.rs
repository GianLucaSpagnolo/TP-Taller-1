use super::coordenates::Coordenates;

#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert,
}

#[derive(Debug, PartialEq, Clone)]

pub struct Cam {
    pub id: u8,
    pub location: Coordenates,
    pub state: CamState,
}

pub struct CamList {
    pub cams: Vec<Cam>,
}

pub fn serialize_cams_vec(cams: Vec<Cam>) -> Vec<u8> {
    let mut bytes = Vec::new();

    let cams_len = cams.len() as u16;
    bytes.extend_from_slice(cams_len.to_be_bytes().as_ref());

    for cam in cams {
        bytes.push(cam.id);
        bytes.extend_from_slice(cam.location.latitude.to_be_bytes().as_ref());
        bytes.extend_from_slice(cam.location.longitude.to_be_bytes().as_ref());

        let state = match cam.state {
            CamState::SavingEnergy => 0,
            CamState::Alert => 1,
        };
        bytes.push(state);
    }

    bytes
}

pub fn deserialize_cams_vec(bytes: Vec<u8>) -> Vec<Cam> {
    let mut cams = Vec::new();
    let mut index = 0;

    let cams_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
    index += 2;

    for _ in 0..cams_len {
        let id = bytes[index];
        index += 1;

        let latitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let longitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let state = match bytes[index] {
            0 => CamState::SavingEnergy,
            1 => CamState::Alert,
            _ => panic!("Invalid state"),
        };
        index += 1;

        cams.push(Cam {
            id,
            location: Coordenates {
                latitude,
                longitude,
            },
            state,
        });
    }

    cams
}
