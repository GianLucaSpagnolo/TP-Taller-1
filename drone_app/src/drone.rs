use std::io::Error;

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::inc_model::incident::Incident;

pub struct Drone {
    distancia_maxima_alcance: f64,
    duracion_de_bateria: f64,
    initial_lat: f64,
    initial_lon: f64,
    charging_station_lat: f64,
    charging_station_lon: f64,
} 

impl Drone {
    pub fn init (
        distancia_maxima_alcance: f64,
        duracion_de_bateria: f64,
        initial_lat: f64,
        initial_lon: f64,
        charging_station_lat: f64,
        charging_station_lon: f64,
    ) -> Result<Self, Error> {
        Ok(Drone {
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_lat,
            initial_lon,
            charging_station_lat,
            charging_station_lon,
        })
    }

    pub fn process_incident(&self, client: &mut MqttClient , incident: Incident, logger: &Logger ) {
            if (incident.location.latitude - self.initial_lat).abs() < self.distancia_maxima_alcance
                && (incident.location.longitude - self.initial_lon).abs() < self.distancia_maxima_alcance && incident.drones_covering < 2
            {
                //client.publish(, "drone".to_string(), logger);
                println!("Incidente procesado: {:?}", incident);
            }
    }
}