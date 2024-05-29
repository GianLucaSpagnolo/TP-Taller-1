pub mod incident_controller {

    use mqtt::client::mqtt_client::MqttClient;

    use crate::model::{
        coordenates::Coordenates,
        incident::{Incident, IncidentState},
        incident_list::IncidentList,
    };

    fn send_incident(client: &mut MqttClient, incident_received: Incident) {
        client
            .publish(incident_received.as_bytes().clone(), "inc".to_string())
            .unwrap();
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        location: Coordenates,
    ) {
        let incident = historial.add(location);
        send_incident(client, incident.clone());
        historial.incidents.insert(incident.id.clone(), incident);
    }

    pub fn resolve_incident(client: &mut MqttClient, historial: &mut IncidentList, id: &String) {
        let incident = historial.incidents.get_mut(id).unwrap();
        incident.state = IncidentState::Resolved;
        client
            .publish(incident.as_bytes().clone(), "inc".to_string())
            .unwrap();
    }
}
