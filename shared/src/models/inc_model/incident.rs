use chrono::offset::Utc;
use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime}; // Add this line to import SystemTime and Duration
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
    pub creation_time: u64,
    pub resolution_time: Option<u64>,
}

impl Incident {
    pub fn new(id: u8, location: Position) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Incident {
            id,
            location,
            state: IncidentState::InProgess,
            drones_covering: 0,
            creation_time: now.as_secs(),
            resolution_time: None,
        }
    }

    pub fn cover(&mut self) {
        self.drones_covering += 1;
    }

    pub fn get_creation_time(&self) -> String {
        let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(self.creation_time);
        let time: DateTime<Utc> = system_time.into();
        format!("{}", time.with_timezone(&Local).format("%d/%m/%Y %T"))
    }

    pub fn get_resolve_time(&self) -> String {
        if let Some(resolution_time) = self.resolution_time {
            let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(resolution_time);
            let time: DateTime<Utc> = system_time.into();
            return format!("{}", time.with_timezone(&Local).format("%d/%m/%Y %T"));
        }
        "--/--/-- --:--:--".to_string()
    }

    pub fn resolve(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        self.state = IncidentState::Resolved;
        self.drones_covering = 0;
        self.resolution_time = Some(now.as_secs());
    }

    pub fn unresolve(&mut self) {
        self.state = IncidentState::Resolved;
        self.resolution_time = None;
    }

    pub fn len_in_bytes() -> usize {
        1 + 8 + 8 + 1 + 1 + 8 + 8
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

        bytes.extend_from_slice(self.creation_time.to_be_bytes().as_ref());

        bytes.extend_from_slice(self.resolution_time.unwrap_or(0).to_be_bytes().as_ref());

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
        index += 1;

        let creation_time = u64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let resolution_secs = u64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        let resolution_time = if resolution_secs == 0 {
            None
        } else {
            Some(resolution_secs)
        };

        Incident {
            id,
            location: Position::from_lat_lon(latitude, longitude),
            state,
            drones_covering,
            creation_time,
            resolution_time,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let incident = Incident::new(0, Position::from_lat_lon(1.0, 1.0));

        let bytes = incident.as_bytes();
        let incident2 = Incident::from_be_bytes(bytes);

        assert_eq!(incident, incident2);

        let mut incident = Incident::new(1, Position::from_lat_lon(10.0, 10.0));
        incident.resolve();

        let bytes = incident.as_bytes();
        let incident2 = Incident::from_be_bytes(bytes);

        assert_eq!(incident, incident2);
    }
}
