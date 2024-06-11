pub mod incident_controller {

    use std::fs;

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use rand::Error;

    use crate::models::{
        coordenates::Coordenates,
        inc_model::{
            incident::{Incident, IncidentState},
            incident_list::IncidentList,
        },
    };

    fn send_incident(client: &mut MqttClient, incident_received: Incident, logger: &Logger) {
        client
            .publish(
                incident_received.as_bytes().clone(),
                "inc".to_string(),
                logger,
            )
            .unwrap();
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        location: Coordenates,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let incident = historial.add(location);
        send_incident(client, incident.clone(), logger);
        historial.incidents.insert(incident.id, incident);

        let bytes = historial.as_bytes();
        match fs::write(db_path, bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }

    pub fn resolve_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let incident = historial.incidents.get_mut(id).unwrap();

        incident.state = IncidentState::Resolved;

        client
            .publish(incident.as_bytes().clone(), "inc".to_string(), logger)
            .unwrap();

        let bytes = historial.as_bytes();
        match fs::write(db_path, bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}
