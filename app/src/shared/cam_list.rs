use std::fmt::Display;

use super::coordenates::Coordenates;

use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert,
}

impl Display for CamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CamState::SavingEnergy => "\x1B[32mSavingEnergy\x1B[0m".to_string(),
            CamState::Alert => "\x1B[31mAlert\x1B[0m".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]

pub struct Cam {
    pub id: u8,
    pub location: Coordenates,
    pub state: CamState,
}

impl Display for Cam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cam {{ id: {}, state: {}, location: {} }}",
            self.id, self.state, self.location
        )
    }
}

pub struct CamList {
    pub cams: Vec<Cam>,
}

impl Display for CamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for cam in &self.cams {
            str.push_str(&format!("{}\n", cam));
        }
        write!(f, "{}", str)
    }
}

impl CamList {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let cams_len = self.cams.len() as u16;
        bytes.extend_from_slice(cams_len.to_be_bytes().as_ref());

        for cam in &self.cams {
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

    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
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

        CamList { cams }
    }

    pub fn generate_ramdoms_cams(number_of_camaras: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cams = Vec::new();
        for i in 0..number_of_camaras {
            cams.push(Cam {
                id: i as u8,
                location: Coordenates {
                    latitude: rng.gen_range(-90.0..90.0),
                    longitude: rng.gen_range(-180.0..180.0),
                },
                state: CamState::SavingEnergy,
            });
        }
        CamList { cams }
    }
}
