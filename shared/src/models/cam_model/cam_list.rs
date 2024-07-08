use std::{collections::HashMap, fmt::Display};

use super::cam::{Cam, CamState};

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
    pub cams: HashMap<u8, Cam>,
}

impl Display for CamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for cam in self.cams.values() {
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

        for cam in self.cams.values() {
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
        let mut cams = HashMap::new();
        let mut index = 0;

        let cams_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..cams_len {
            let cam = Cam::from_be_bytes(bytes[index..index + Cam::len_in_bytes()].into());
            index += Cam::len_in_bytes();
            cams.insert(cam.id, cam);
        }

        CamList { cams }
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.cams.values().map(|cam| cam.location).collect()
    }

    pub fn update_cam(&mut self, new_cam: Cam) {
        if CamState::Removed == new_cam.state {
            self.cams.remove(&new_cam.id);
        } else {
            self.cams.insert(new_cam.id, new_cam);
        }
    }

    pub fn disconnect_all(&mut self) {
        for cam in self.cams.values_mut() {
            cam.disconnect();
        }
    }

    pub fn init(db_path: &str) -> Self {
        let bytes = match std::fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            CamList {
                cams: HashMap::new(),
            }
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

    #[test]
    fn test_serialization() {
        let cam1 = Cam::new(
            0,
            Position::from_lat_lon(-34.581568266649754, -58.4744644927824),
        );

        let mut cam2 = Cam::new(
            1,
            Position::from_lat_lon(-34.631345851866776, -58.41585822580699),
        );
        cam2.to_alert();
        let mut cam3 = Cam::new(
            2,
            Position::from_lat_lon(-34.61863371802939, -58.45012545762901),
        );
        cam3.remove();

        let mut cams = HashMap::new();
        cams.insert(cam1.id, cam1);
        cams.insert(cam2.id, cam2);
        cams.insert(cam3.id, cam3);

        let cam_list = CamList { cams };

        let bytes = cam_list.as_bytes();
        let new_cam_list = CamList::from_be_bytes(bytes);

        assert_eq!(
            cam_list.cams.get(&0).unwrap().id,
            new_cam_list.cams.get(&0).unwrap().id
        );
        assert_eq!(
            cam_list.cams.get(&0).unwrap().location,
            new_cam_list.cams.get(&0).unwrap().location
        );
        assert_eq!(
            cam_list.cams.get(&0).unwrap().state,
            new_cam_list.cams.get(&0).unwrap().state
        );
        assert_eq!(
            cam_list.cams.get(&0).unwrap().incidents_covering,
            new_cam_list.cams.get(&0).unwrap().incidents_covering
        );

        assert_eq!(
            cam_list.cams.get(&1).unwrap().id,
            new_cam_list.cams.get(&1).unwrap().id
        );
        assert_eq!(
            cam_list.cams.get(&1).unwrap().location,
            new_cam_list.cams.get(&1).unwrap().location
        );
        assert_eq!(
            cam_list.cams.get(&1).unwrap().state,
            new_cam_list.cams.get(&1).unwrap().state
        );
        assert_eq!(
            cam_list.cams.get(&1).unwrap().incidents_covering,
            new_cam_list.cams.get(&1).unwrap().incidents_covering
        );

        assert_eq!(
            cam_list.cams.get(&2).unwrap().id,
            new_cam_list.cams.get(&2).unwrap().id
        );
        assert_eq!(
            cam_list.cams.get(&2).unwrap().location,
            new_cam_list.cams.get(&2).unwrap().location
        );
        assert_eq!(
            cam_list.cams.get(&2).unwrap().state,
            new_cam_list.cams.get(&2).unwrap().state
        );
        assert_eq!(
            cam_list.cams.get(&2).unwrap().incidents_covering,
            new_cam_list.cams.get(&2).unwrap().incidents_covering
        );
    }
}
