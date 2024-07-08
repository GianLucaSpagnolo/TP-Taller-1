use std::fmt::Display;

use walkers::Position;

/// ## Camstate
///
/// Enumeración que representa el modo de una cámara
///
/// ### Valores
/// - `SavingEnergy`: Ahorro de energía
/// - `Alert`: Alerta
///
#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert,
    Disconnected,
    Removed,
}

impl Display for CamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CamState::SavingEnergy => "\x1B[32mSavingEnergy\x1B[0m".to_string(),
            CamState::Alert => "\x1B[31mAlert\x1B[0m".to_string(),
            CamState::Disconnected => "\x1B[33mDisconnected\x1B[0m".to_string(),
            CamState::Removed => "\x1B[33mRemoved\x1B[0m".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// ## Cam
///
/// Estructura que representa una cámara
///
/// ### Atributos
/// - `id`: Identificador de la cámara
/// - `location`: Coordenadas de la cámara
/// - `state`: Modo de la cámara
/// - 'state': Estado de la cámara
///
#[derive(Debug, PartialEq, Clone)]

pub struct Cam {
    pub id: u8,
    pub location: Position,
    pub state: CamState,
    pub incidents_covering: u8,
}

impl Display for Cam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cam {{ id: {}, state: {}, location: ({};{})}}",
            self.id,
            self.state,
            self.location.lat(),
            self.location.lon()
        )
    }
}

impl Cam {
    pub fn new(id: u8, location: Position) -> Self {
        Cam {
            id,
            location,
            state: CamState::SavingEnergy,
            incidents_covering: 0,
        }
    }

    pub fn remove(&mut self) {
        self.state = CamState::Removed;
    }

    pub fn to_alert(&mut self) {
        self.state = CamState::Alert;
    }

    pub fn to_saving_energy(&mut self) {
        self.state = CamState::SavingEnergy;
    }

    pub fn disconnect(&mut self) {
        self.state = CamState::Disconnected;
    }

    pub fn len_in_bytes() -> usize {
        1 + 8 + 8 + 1 + 1
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.id);
        bytes.extend_from_slice(self.location.lat().to_be_bytes().as_ref());
        bytes.extend_from_slice(self.location.lon().to_be_bytes().as_ref());
        let state = match self.state {
            CamState::SavingEnergy => 0,
            CamState::Alert => 1,
            CamState::Disconnected => 2,
            CamState::Removed => 3,
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
            2 => CamState::Disconnected,
            3 => CamState::Removed,
            _ => panic!("Invalid state {}", bytes[index]),
        };
        index += 1;
        let incidents_covering = bytes[index];
        Cam {
            id,
            location: Position::from_lat_lon(latitude, longitude),
            state,
            incidents_covering,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let cam = Cam::new(0, Position::from_lat_lon(1.0, 1.0));
        let mut cam2 = Cam::new(1, Position::from_lat_lon(1.0, 10.0));
        cam2.to_alert();
        let cam3 = Cam::new(2, Position::from_lat_lon(10.0, 1.0));

        let bytes = cam.as_bytes();

        let cam_from_bytes = Cam::from_be_bytes(bytes);

        assert_eq!(cam.id, cam_from_bytes.id);
        assert_eq!(cam.location, cam_from_bytes.location);
        assert_eq!(cam.state, cam_from_bytes.state);
        assert_eq!(cam.incidents_covering, cam_from_bytes.incidents_covering);

        let bytes = cam2.as_bytes();

        let cam_from_bytes = Cam::from_be_bytes(bytes);

        assert_eq!(cam2.id, cam_from_bytes.id);
        assert_eq!(cam2.location, cam_from_bytes.location);
        assert_eq!(cam2.state, cam_from_bytes.state);
        assert_eq!(cam2.incidents_covering, cam_from_bytes.incidents_covering);

        let bytes = cam3.as_bytes();

        let cam_from_bytes = Cam::from_be_bytes(bytes);

        assert_eq!(cam3.id, cam_from_bytes.id);
        assert_eq!(cam3.location, cam_from_bytes.location);
        assert_eq!(cam3.state, cam_from_bytes.state);
        assert_eq!(cam3.incidents_covering, cam_from_bytes.incidents_covering);
    }
}
