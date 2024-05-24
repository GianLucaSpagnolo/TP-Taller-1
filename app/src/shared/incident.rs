use super::coordenates::Coordenates;

pub enum IncidentState {
    InProgess,
    Resolved,
}

pub struct Incident {
    pub id: String,
    pub description: String,
    pub location: Coordenates,
    pub state: IncidentState,
}
