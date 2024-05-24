use super::coordenates::Coordenates;

pub enum CamState {
    Active,
    Inactive,
}

pub struct Cam {
    pub id: String,
    pub location: Coordenates,
    pub state: CamState,
}

pub struct CamList {
    pub cams: Vec<Cam>,
}
