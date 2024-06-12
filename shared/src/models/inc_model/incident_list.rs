use std::collections::HashMap;

use crate::models::coordenates::Coordenates;

use super::incident::{self, Incident, IncidentState};

/// ## IncidentList
///
/// Estructura que representa una lista de incidentes
///
/// ### Atributos
/// - `incidents`: HashMap de incidentes
///
#[derive(Default)]
pub struct IncidentList {
    pub incidents: HashMap<u8, Incident>,
}

impl IncidentList {
    pub fn add(&mut self, location: Coordenates) -> Incident {
        let incident = Incident {
            id: self.incidents.len() as u8,
            location,
            state: IncidentState::InProgess,
            drones_covering: 0,
        };
        self.incidents.insert(incident.id, incident.clone());
        incident
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let inc_len = self.incidents.len() as u16;
        bytes.extend_from_slice(inc_len.to_be_bytes().as_ref());

        for incident in self.incidents.values() {
            bytes.extend_from_slice(incident.as_bytes().as_ref());
        }
        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> IncidentList {
        let mut incidents = HashMap::new();

        let mut index = 0;

        let incs_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..incs_len {
            let incident = Incident::from_be_bytes(bytes[index..].to_vec());
            index += incident::INCIDENT_SIZE;
            incidents.insert(incident.id, incident);
        }

        IncidentList { incidents }
    }
}
