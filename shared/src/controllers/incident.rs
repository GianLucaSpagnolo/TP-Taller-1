pub mod incident_controller {

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;

    use crate::models::{
        coordenates::Coordenates,
        inc_model::{
            incident::{Incident, IncidentState},
            incident_list::IncidentList,
        },
    };

    fn send_incident(client: &mut MqttClient, incident_received: Incident, logger: &Logger) {
        client
            .publish(incident_received.as_bytes().clone(), "inc".to_string(), logger)
            .unwrap();
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        location: Coordenates,
        logger: &Logger,
    ) {
        let incident = historial.add(location);
        send_incident(client, incident.clone(), logger);
        historial.incidents.insert(incident.id.clone(), incident);
    }

    pub fn resolve_incident(client: &mut MqttClient, historial: &mut IncidentList, id: &String, logger: &Logger) {
        let incident = historial.incidents.get_mut(id).unwrap();
        incident.state = IncidentState::Resolved;
        client
            .publish(incident.as_bytes().clone(), "inc".to_string(), logger)
            .unwrap();
    }
}
