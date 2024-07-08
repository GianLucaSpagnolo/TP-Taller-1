use eframe::egui::Ui;

use egui_extras::{Column, TableBuilder};

use crate::models::drone_model::{
    drone::{Drone, DroneState},
    drone_list::DroneList,
};

static COORDENATE_PRECISION: usize = 4;

fn drone_row(mut row: egui_extras::TableRow, drone: &Drone) {
    row.col(|ui| {
        ui.label(drone.id.to_string());
    });
    row.col(|ui| {
        if let DroneState::Available = drone.state {
            ui.label(egui::RichText::new("Disponible").color(egui::Color32::WHITE));
        } else if let DroneState::GoingToIncident = drone.state {
            ui.label(egui::RichText::new("Atacando incidente").color(egui::Color32::RED));
        } else if let DroneState::GoingBack = drone.state {
            ui.label(egui::RichText::new("Volviendo a posicion").color(egui::Color32::DARK_GREEN));
        } else if let DroneState::ResolvingIncident = drone.state {
            ui.label(egui::RichText::new("Resolviendo incidente").color(egui::Color32::YELLOW));
        } else if let DroneState::Disconnected = drone.state {
            ui.label(egui::RichText::new("Desconectado").color(egui::Color32::DARK_GRAY));
        } else {
            ui.label(egui::RichText::new("Volviendo a central").color(egui::Color32::GRAY));
        }
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            drone.current_pos.lat(),
            COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            drone.current_pos.lon(),
            COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        if let DroneState::LowBattery = drone.state {
            ui.label(egui::RichText::new("Batería baja").color(egui::Color32::DARK_RED));
        } else if let DroneState::Charging = drone.state {
            ui.label(egui::RichText::new("Cargando").color(egui::Color32::GREEN));
        } else {
            ui.label(egui::RichText::new("Con Batería").color(egui::Color32::WHITE));
        }
    });
}

fn drones_list(ui: &mut Ui, drone_list: &DroneList) {
    TableBuilder::new(ui)
        .column(Column::exact(100.0))
        .column(Column::exact(250.0))
        .column(Column::exact(250.0))
        .column(Column::exact(250.0))
        .column(Column::exact(200.0))
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading("ID");
            });
            header.col(|ui| {
                ui.heading("Estado");
            });
            header.col(|ui| {
                ui.heading("Latitud");
            });
            header.col(|ui| {
                ui.heading("Longitud");
            });
            header.col(|ui| {
                ui.heading("Bateria");
            });
        })
        .body(|mut body| {
            if drone_list.drones.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay drones");
                    });
                });
            } else {
                for drone in drone_list.drones.values() {
                    body.row(20.0, |row| {
                        drone_row(row, drone);
                    });
                }
            }
        });
}

pub fn show_drones(ui: &mut Ui, drone_list: &DroneList) {
    ui.heading("Listado de drones");
    ui.separator();
    ui.add_space(10.0);
    drones_list(ui, drone_list);
}
