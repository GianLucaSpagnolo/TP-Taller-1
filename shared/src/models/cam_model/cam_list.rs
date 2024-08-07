use std::{collections::HashMap, fmt::Display, thread, time::Duration};

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

    pub fn update_cams_state(
        &mut self,
        incident_location: Position,
        new_state: CamState,
        range_alert: &f64,
        range_alert_between_cameras: &f64,
    ) -> Vec<Cam> {
        let mut modified_cams = Vec::new();

        if new_state == CamState::SavingEnergy {
            thread::sleep(Duration::from_secs(3));
        }

        for cam in self.cams.values_mut() {
            if cam.is_near(incident_location, range_alert) {
                cam.change_state(&new_state);
                modified_cams.push(cam.clone());
            }
        }

        let mut close_cameras_modified = Vec::new();

        for cam in self.cams.values_mut() {
            for modified_cam in &modified_cams {
                if cam.is_near(modified_cam.location, range_alert_between_cameras) {
                    if !modified_cams.contains(cam) && cam.change_state(&new_state) {
                        close_cameras_modified.push(cam.clone());
                    }
                    break;
                }
            }
        }
        for cam in close_cameras_modified {
            modified_cams.push(cam);
        }

        modified_cams
    }

    pub fn connect_all(&mut self) {
        for cam in self.cams.values_mut() {
            cam.connect();
        }
    }

    pub fn disconnect_all(&mut self) {
        for cam in self.cams.values_mut() {
            cam.disconnect();
        }
    }

    pub fn delete_cam(&mut self, id: &u8) -> Option<Cam> {
        let cam = self.cams.get(id)?;
        if cam.state == CamState::Alert {
            return None;
        }
        self.cams.remove(id)
    }

    pub fn is_cam_in_alert(&self, id: &u8) -> bool {
        if let Some(cam) = self.cams.get(id) {
            return cam.state == CamState::Alert;
        }
        false
    }

    pub fn edit_cam_position(&mut self, id: &u8, new_pos: Position) -> Option<Cam> {
        if let Some(cam) = self.cams.get_mut(id) {
            cam.location = new_pos;
            return Some(cam.clone());
        }
        None
    }

    pub fn get_cam(&self, id: &u8) -> Option<&Cam> {
        self.cams.get(id)
    }

    fn generate_id(&self) -> u8 {
        let id = self.cams.keys().max().map(|id| id + 1);
        id.unwrap_or(0)
    }

    pub fn add_cam(&mut self, pos: Position) -> Cam {
        let id = self.generate_id();
        let cam = Cam::new(id, pos);
        // No puede devolver None ya que el id generado no está en uso
        self.cams.insert(id, cam.clone());
        cam
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
