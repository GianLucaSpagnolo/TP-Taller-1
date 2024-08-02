pub mod incident_controller {

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use std::io::Error;
    use walkers::Position;

    use crate::models::inc_model::incident_list::IncidentList;

    fn send_incident(
        client: &mut MqttClient,
        inc_list: &IncidentList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let inc = inc_list.get_inc(id).unwrap().clone();

        client.publish(inc.as_bytes().clone(), "inc".to_string(), logger)?;

        inc_list.save(db_path)
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        location: Position,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let inc_id = historial.add_inc(location);
        send_incident(client, historial, &inc_id, logger, db_path)
    }

    pub fn resolve_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        historial.resolve_inc(id);
        send_incident(client, historial, id, logger, db_path)
    }
}
