pub mod incident_controller {

    use mqtt::client::mqtt_client::MqttClient;
    use walkers::Position;

    use crate::{
        models::{
            coordenates::Coordenates,
            inc_model::{
                incident::{Incident, IncidentState},
                incident_list::IncidentList,
            },
        },
        views::map_views::plugins,
    };

    fn send_incident(client: &mut MqttClient, incident_received: Incident) {
        client
            .publish(incident_received.as_bytes().clone(), "inc".to_string())
            .unwrap();
    }

    pub fn add_incident(
        client: &mut MqttClient,
        historial: &mut IncidentList,
        view: &mut plugins::ImagesData,
        location: Coordenates,
    ) {
        let incident = historial.add(location);
        send_incident(client, incident.clone());
        view.add_image(Position::from_lon_lat(
            incident.location.longitude,
            incident.location.latitude,
        ));
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
