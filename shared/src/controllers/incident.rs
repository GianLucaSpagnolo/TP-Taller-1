pub mod incident_controller {

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use rand::Error;
    use walkers::Position;

    use crate::models::inc_model::{
        incident::Incident,
        incident_list::IncidentList,
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
        location: Position,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let incident = historial.add(location);
        send_incident(client, incident.clone(), logger);
        historial.incidents.insert(incident.id, incident);

        match historial.save(db_path){
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

        incident.resolve();

        client
            .publish(incident.as_bytes().clone(), "inc".to_string(), logger)
            .unwrap();

        match historial.save(db_path){
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}
