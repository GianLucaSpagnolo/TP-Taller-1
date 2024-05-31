use std::collections::HashMap;

use crate::models::coordenates::Coordenates;

use super::incident::{Incident, IncidentState};

/// ## IncidentList
/// 
/// Estructura que representa una lista de incidentes
/// 
/// ### Atributos
/// - `incidents`: HashMap de incidentes
/// 
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
