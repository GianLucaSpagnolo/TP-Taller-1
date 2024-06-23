use std::collections::HashMap;

use walkers::Position;

use super::incident::{Incident, IncidentState};

/// ## IncidentList
///
/// Estructura que representa una lista de incidentes
///
/// ### Atributos
/// - `incidents`: HashMap de incidentes
///
#[derive(Default, Debug, Clone)]
pub struct IncidentList {
    pub incidents: HashMap<u8, Incident>,
}

impl IncidentList {
    pub fn add(&mut self, location: Position) -> Incident {
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
            index += Incident::len_in_bytes();
            incidents.insert(incident.id, incident);
        }

        IncidentList { incidents }
    }

    pub fn init(db_path: &str) -> std::io::Result<IncidentList> {
        let bytes = match std::fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            Ok(IncidentList::default())
        } else {
            let incidents = IncidentList::from_be_bytes(bytes);
            Ok(incidents)
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
        let mut incident_list = IncidentList::default();
        let incident = Incident {
            id: 0,
            location: Position::from_lat_lon(0.0, 0.0),
            state: IncidentState::InProgess,
            drones_covering: 0,
        };
        let incident2 = Incident {
            id: 1,
            location: Position::from_lat_lon(10.0, 10.0),
            state: IncidentState::Resolved,
            drones_covering: 0,
        };
        incident_list.incidents.insert(incident.id, incident);
        incident_list.incidents.insert(incident2.id, incident2);
        let bytes = incident_list.as_bytes();
        let incident_list2 = IncidentList::from_be_bytes(bytes);

        assert_eq!(incident_list.incidents, incident_list2.incidents);
    }
}
