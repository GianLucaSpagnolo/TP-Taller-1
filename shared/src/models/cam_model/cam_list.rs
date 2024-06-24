use std::fmt::Display;

use super::cam::Cam;

//use rand::Rng;
use walkers::Position;

/// ## CamList
///
/// Estructura que representa una lista de cámaras
///
/// ### Atributos
/// - `cams`: Vector de cámaras
///
#[derive(Default)]
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
    /// ## as_bytes
    ///
    /// Convierte la lista de cámaras a un vector de bytes
    ///
    /// ### Retorno
    /// - `Vec<u8>`: Vector de bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let cams_len = self.cams.len() as u16;
        bytes.extend_from_slice(cams_len.to_be_bytes().as_ref());

        for cam in &self.cams {
            bytes.extend_from_slice(&cam.as_bytes());
        }

        bytes
    }

    /// ## from_be_bytes
    ///
    /// Convierte un vector de bytes en una lista de cámaras
    ///
    /// ### Parametros
    /// - `bytes`: Vector de bytes
    ///
    /// ### Retorno
    /// - `CamList`: Lista de cámaras creada
    ///
    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
        let mut cams = Vec::new();
        let mut index = 0;

        let cams_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..cams_len {
            let cam = Cam::from_be_bytes(bytes[index..index + Cam::len_in_bytes()].into());
            index += Cam::len_in_bytes();
            cams.push(cam);
        }

        CamList { cams }
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.cams.iter().map(|cam| cam.location).collect()
    }

    pub fn init(db_path: &str) -> Self {
        let bytes = match std::fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            CamList { cams: Vec::new() }
        } else {
            CamList::from_be_bytes(bytes)
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let bytes = self.as_bytes();
        std::fs::write(path, bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::cam_model::cam::CamState;

    #[test]
    fn test_serialization() {
        let cam1 = Cam {
            id: 0,
            location: Position::from_lat_lon(-34.581568266649754, -58.4744644927824),
            state: CamState::SavingEnergy,
            incidents_covering: 0,
        };

        let cam2 = Cam {
            id: 1,
            location: Position::from_lat_lon(-34.631345851866776, -58.41585822580699),
            state: CamState::Removed,
            incidents_covering: 0,
        };

        let cam3 = Cam {
            id: 2,
            location: Position::from_lat_lon(-34.61863371802939, -58.45012545762901),
            state: CamState::Alert,
            incidents_covering: 0,
        };

        let cam_list = CamList {
            cams: vec![cam1, cam2, cam3],
        };

        let bytes = cam_list.as_bytes();
        let new_cam_list = CamList::from_be_bytes(bytes);

        assert_eq!(cam_list.cams[0].id, new_cam_list.cams[0].id);
        assert_eq!(cam_list.cams[0].location, new_cam_list.cams[0].location);
        assert_eq!(cam_list.cams[0].state, new_cam_list.cams[0].state);
        assert_eq!(
            cam_list.cams[0].incidents_covering,
            new_cam_list.cams[0].incidents_covering
        );

        assert_eq!(cam_list.cams[1].id, new_cam_list.cams[1].id);
        assert_eq!(cam_list.cams[1].location, new_cam_list.cams[1].location);
        assert_eq!(cam_list.cams[1].state, new_cam_list.cams[1].state);
        assert_eq!(
            cam_list.cams[1].incidents_covering,
            new_cam_list.cams[1].incidents_covering
        );

        assert_eq!(cam_list.cams[2].id, new_cam_list.cams[2].id);
        assert_eq!(cam_list.cams[2].location, new_cam_list.cams[2].location);
        assert_eq!(cam_list.cams[2].state, new_cam_list.cams[2].state);
        assert_eq!(
            cam_list.cams[2].incidents_covering,
            new_cam_list.cams[2].incidents_covering
        );
    }
}
