use super::coordenates::Coordenates;

pub enum CamState {
    Inactive,
    Active,
}

pub struct Cam {
    pub id: String,
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
        let id_len = cam.id.len() as u16;
        bytes.extend_from_slice(id_len.to_be_bytes().as_ref());
        bytes.extend_from_slice(cam.id.as_bytes());
        bytes.extend_from_slice(cam.location.lat.to_be_bytes().as_ref());
        bytes.extend_from_slice(cam.location.long.to_be_bytes().as_ref());

        let state = match cam.state {
            CamState::Inactive => 0,
            CamState::Active => 1,
        };
        bytes.push(state);
    }

    bytes
}

pub fn deserialize_cams_vec(bytes: Vec<u8>) -> Vec<Cam> {
    let mut cams = Vec::new();
    let mut index = 0;

    let cams_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as u16;
    index += 2;

    for _ in 0..cams_len {
        let id_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as u16;
        index += 2;

        let id = String::from_utf8(bytes[index..index + id_len as usize].to_vec()).unwrap();
        index += id_len as usize;

        let lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let long = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let state = match bytes[index] {
            0 => CamState::Inactive,
            1 => CamState::Active,
            _ => panic!("Invalid state"),
        };
        index += 1;

        cams.push(Cam {
            id,
            location: Coordenates { lat, long },
            state,
        });
    }

    cams
}
