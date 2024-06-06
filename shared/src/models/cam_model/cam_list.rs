use std::fmt::Display;

use crate::models::coordenates::Coordenates;

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

    pub fn generate_ramdoms_cams(_number_of_camaras: i32) -> Self {
        let mut cams = Vec::new();
        /*
        let mut rng = rand::thread_rng();
        for i in 0..number_of_camaras {
            cams.push(Cam {
                id: i as u8,
                location: Coordenates {
                    latitude: rng.gen_range(-90.0..90.0),
                    longitude: rng.gen_range(-180.0..180.0),
                },
                state: CamState::SavingEnergy,
            });
        } */

        let cam1 = Cam {
            id: 1,
            location: Coordenates {
                latitude: -34.581568266649754,
                longitude: -58.4744644927824,
            },
            state: CamState::SavingEnergy,
        };

        let cam2 = Cam {
            id: 2,
            location: Coordenates {
                latitude: -34.631345851866776,
                longitude: -58.41585822580699,
            },
            state: CamState::SavingEnergy,
        };

        let cam3 = Cam {
            id: 3,
            location: Coordenates {
                latitude: -34.61863371802939,
                longitude: -58.45012545762901,
            },
            state: CamState::SavingEnergy,
        };

        let cam4 = Cam {
            id: 4,
            location: Coordenates {
                latitude: -34.58153624609583,
                longitude: -58.42089675544147,
            },
            state: CamState::SavingEnergy,
        };

        let cam5 = Cam {
            id: 5,
            location: Coordenates {
                latitude: -34.608203436360505,
                longitude: -58.37366305468922,
            },
            state: CamState::SavingEnergy,
        };

        cams.push(cam1);
        cams.push(cam2);
        cams.push(cam3);
        cams.push(cam4);
        cams.push(cam5);

        CamList { cams }
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.cams
            .iter()
            .map(|cam| Position::from_lat_lon(cam.location.latitude, cam.location.longitude))
            .collect()
    }
}
