pub mod incident_controller {
    use app::shared::{coordenates::Coordenates, incident::{Incident, IncidentState}};

    use crate::app::MonitoringApp;

    fn send_incident(app: &mut MonitoringApp, incident_received: Incident){
        app.client.publish(incident_received.as_bytes().clone(), "inc".to_string()).unwrap();
    }

    pub fn add_incident(app: &mut MonitoringApp, location: Coordenates) {
        let incident = app.historial.add(location);
        send_incident(app, incident.clone());
        app.historial.incidents.insert(incident.id.clone(), incident);
    }
    
    pub fn resolve_incident(app: &mut MonitoringApp, id: &String) {
        let incident = app.historial.incidents.get_mut(id).unwrap();
        incident.state = IncidentState::Resolved;
        app.client.publish(incident.as_bytes().clone(), "inc".to_string()).unwrap();
    }
    
}