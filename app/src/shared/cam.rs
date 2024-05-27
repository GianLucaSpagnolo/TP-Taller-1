use std::fmt::Display;

use super::coordenates::Coordenates;

#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert,
}

impl Display for CamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CamState::SavingEnergy => "\x1B[32mSavingEnergy\x1B[0m".to_string(),
            CamState::Alert => "\x1B[31mAlert\x1B[0m".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]

pub struct Cam {
    pub id: u8,
    pub location: Coordenates,
    pub state: CamState,
}

impl Display for Cam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cam {{ id: {}, state: {}, location: {} }}",
            self.id, self.state, self.location
        )
    }
}
