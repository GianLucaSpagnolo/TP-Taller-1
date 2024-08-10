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
    ) -> Result<(), Error> {
        let inc = Incident::new(historial.generate_id(), location);
        send_incident(client, &inc, logger)
    }

    pub fn resolve_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        id: &u8,
        logger: &Logger,
    ) -> Result<(), Error> {
        let mut inc = historial.get_inc(id).unwrap().clone();
        inc.resolve();
        send_incident(client, &inc, logger)
    }
}
