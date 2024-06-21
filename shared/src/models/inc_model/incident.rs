use walkers::Position;

#[derive(Debug, PartialEq, Clone)]
pub enum IncidentState {
    InProgess,
    Resolved,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Incident {
    pub id: u8,
    pub location: Position,
    pub state: IncidentState,
    pub drones_covering: u8,
}

impl Incident {
    pub fn len_in_bytes() -> usize {
        1 + 8 + 8 + 1 + 1
    }

    /// ## as_bytes
    ///
    /// Convierte el incidente a un vector de bytes
    ///
    /// ### Retorno
    /// - `Vec<u8>`: Vector de bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.id);
        bytes.extend_from_slice(self.location.lat().to_be_bytes().as_ref());
        bytes.extend_from_slice(self.location.lon().to_be_bytes().as_ref());

        let state = match self.state {
            IncidentState::InProgess => 0,
            IncidentState::Resolved => 1,
        };

        bytes.push(state);
        bytes.push(self.drones_covering);

        bytes
    }

    /// ## from_be_bytes
    ///
    /// Convierte un vector de bytes en un incidente
    ///
    /// ### Parametros
    /// - `bytes`: Vector de bytes
    ///
    /// ### Retorno
    /// - `Incident`: Incidente creado
    ///
    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
        let mut index = 0;

        let id = bytes[index];
        index += 1;

        let latitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        let longitude = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let state = match bytes[index] {
            0 => IncidentState::InProgess,
            1 => IncidentState::Resolved,
            _ => panic!("Invalid state"),
        };

        index += 1;
        let drones_covering = bytes[index];

        Incident {
            id,
            location: Position::from_lat_lon(latitude, longitude),
            state,
            drones_covering,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let incident = Incident {
            id: 0,
            location: Position::from_lat_lon(1.0, 1.0),
            state: IncidentState::InProgess,
            drones_covering: 0,
        };

        let bytes = incident.as_bytes();
        let incident2 = Incident::from_be_bytes(bytes);

        assert_eq!(incident, incident2);
    }
}
