pub mod incident_controller {

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use std::io::Error;
    use walkers::Position;

    use crate::models::inc_model::{incident::Incident, incident_list::IncidentList};

    fn send_incident(
        client: &mut MqttClient,
        inc: &Incident,
        logger: &Logger,
    ) -> Result<(), Error> {
        client.publish(inc.as_bytes().clone(), "inc".to_string(), logger)
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        location: Position,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let inc = Incident::new(historial.generate_id(), location);
        match send_incident(client, &inc, logger) {
            Ok(_) => {
                historial.add_inc(location);
                historial.save(db_path)
            }
            Err(e) => Err(e),
        }
    }

    pub fn resolve_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let mut inc = historial.get_inc(&id).unwrap().clone();
        inc.resolve();

        match send_incident(client, &inc, logger) {
            Ok(_) => {
                historial.resolve_inc(id);
                historial.save(db_path)
            }
            Err(e) => Err(e),
        }
    }
}
