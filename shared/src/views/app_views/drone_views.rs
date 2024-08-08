use eframe::egui::Ui;

use egui_extras::{Column, TableBuilder};

use crate::{
    interfaces::drone_interface::DroneInterface,
    models::drone_model::drone::{Drone, DroneState},
};

static COORDENATE_PRECISION: usize = 4;

fn drone_row(mut row: egui_extras::TableRow, drone: &Drone) {
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(drone.id.to_string());
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            if let DroneState::Available = drone.state {
                ui.label(egui::RichText::new("Disponible").color(egui::Color32::WHITE));
            } else if let DroneState::GoingToIncident = drone.state {
                ui.label(egui::RichText::new("Atacando incidente").color(egui::Color32::RED));
            } else if let DroneState::GoingBack = drone.state {
                ui.label(
                    egui::RichText::new("Volviendo a posicion").color(egui::Color32::DARK_GREEN),
                );
            } else if let DroneState::ResolvingIncident = drone.state {
                ui.label(egui::RichText::new("Resolviendo incidente").color(egui::Color32::YELLOW));
            } else {
                ui.label(egui::RichText::new("Volviendo a central").color(egui::Color32::GRAY));
            }
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(&format!(
                "{:.1$}",
                drone.current_pos.lat(),
                COORDENATE_PRECISION
            ));
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(&format!(
                "{:.1$}",
                drone.current_pos.lon(),
                COORDENATE_PRECISION
            ));
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            if let DroneState::LowBattery = drone.state {
                ui.label(egui::RichText::new("Batería baja").color(egui::Color32::DARK_RED));
            } else if let DroneState::Charging = drone.state {
                ui.label(egui::RichText::new("Cargando").color(egui::Color32::GREEN));
            } else {
                ui.label(egui::RichText::new("Con Batería").color(egui::Color32::WHITE));
            }
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            if drone.connected {
                ui.label(egui::RichText::new("Conectado").color(egui::Color32::GREEN));
            } else {
                ui.label(egui::RichText::new("Desconectado").color(egui::Color32::DARK_GRAY));
            }
        });
    });
}

fn drones_list(ui: &mut Ui, interface: &mut DroneInterface) {
    let drone_list = &mut interface.drone_list;

    if drone_list.drones.is_empty() {
        ui.label("No hay drones");
    } else {
        TableBuilder::new(ui)
            .column(Column::exact(100.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("ID");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Estado");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Latitud");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Longitud");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Conexión");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Bateria");
                    });
                });
            })
            .body(|mut body| {
                for drone in drone_list.drones.values() {
                    body.row(20.0, |row| {
                        drone_row(row, drone);
                    });
                }
            });
    }
}

pub fn show_drones(ui: &mut Ui, dron_interface: &mut DroneInterface) {
    ui.heading("Listado de drones");
    ui.separator();
    ui.add_space(10.0);
    drones_list(ui, dron_interface);
    ui.add_space(10.0);
}
