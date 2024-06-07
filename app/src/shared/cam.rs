use std::fmt::Display;

use super::coordenates::Coordenates;

#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert,
    Removed,
}

impl Display for CamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CamState::SavingEnergy => "\x1B[32mSavingEnergy\x1B[0m".to_string(),
            CamState::Alert => "\x1B[31mAlert\x1B[0m".to_string(),
            CamState::Removed => "\x1B[33mRemoved\x1B[0m".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]

pub struct Cam {
    pub id: u8,
    pub location: Coordenates,
    pub state: CamState,
    pub incidents_covering: u8,
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

impl Cam {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.id);
        bytes.extend_from_slice(self.location.latitude.to_be_bytes().as_ref());
        bytes.extend_from_slice(self.location.longitude.to_be_bytes().as_ref());
        let state = match self.state {
            CamState::SavingEnergy => 0,
            CamState::Alert => 1,
            CamState::Removed => 2,
        };
        bytes.push(state);
        bytes.push(self.incidents_covering);
        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> Cam {
        let mut index = 0;

        let id = bytes[index];
        index += 1;

        let latitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let longitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let state = match bytes[index] {
            0 => CamState::SavingEnergy,
            1 => CamState::Alert,
            2 => CamState::Removed,
            _ => panic!("Invalid state"),
        };
        index += 1;

        let incidents_covering = bytes[index];
        Cam {
            id,
            location: Coordenates {
                latitude,
                longitude,
            },
            state,
            incidents_covering,
        }
    }
}
