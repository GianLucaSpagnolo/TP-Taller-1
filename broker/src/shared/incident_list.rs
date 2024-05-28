use std::collections::HashMap;

use super::{
    coordenates::Coordenates,
    incident::{Incident, IncidentState},
};

#[derive(Default)]
pub struct IncidentList {
    pub incidents: HashMap<String, Incident>,
}

impl IncidentList {
    pub fn add(&mut self, location: Coordenates) -> Incident {
        let incident = Incident {
            id: self.incidents.len().to_string(),
            location,
            state: IncidentState::InProgess,
        };
        self.incidents.insert(incident.id.clone(), incident.clone());
        incident
    }
}
