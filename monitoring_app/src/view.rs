pub mod view{
    use app::shared::{cam_list::{CamList, CamState}, coordenates::Coordenates, incident::{Incident, IncidentState}};
    use eframe::egui::{self, Margin};
    use egui_extras::{TableBuilder, Column};
    /* use walkers::{Tiles, MapMemory, sources::OpenStreetMap}; */

    pub struct MyApp {
        system: CamList,
        incident: Vec<Incident>,
        /* tiles: Tiles,
        map_memory: MapMemory, */
    }
    
    impl MyApp {

        fn new() -> Self {
            let system = CamList::generate_ramdoms_cams(10);

            Self {
                system,
                incident: Vec::new(),
                /* tiles: Tiles::new(OpenStreetMap, egui_ctx),
                map_memory: MapMemory::default(), */
            }
        }
        pub fn add_incident(&mut self, latitud: f64, longitud: f64) {
            self.incident.push(Incident {
                id: self.incident.len().to_string(),
                location: Coordenates {
                    latitude: latitud,
                    longitude: longitud,
                },
                state: IncidentState::InProgess,
            });
        }
    }

    fn integer_edit_field(ui: &mut egui::Ui, value: &mut f64) -> egui::Response {
        let mut tmp_value = format!("{}", value);
        let res = ui.text_edit_singleline(&mut tmp_value);
        if let Ok(result) = tmp_value.parse() {
            *value = result;
        }
        res
    }
    
    

    impl eframe::App for MyApp {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let (mut latitud, mut longitud) = (0.0,0.0);

        let side_frame = egui::Frame {
            inner_margin: Margin { left: 30.0, ..Default::default() },
            ..Default::default()
        }; 

        egui::SidePanel::left("menu").frame(side_frame).show(ctx, |ui| {
            ui.heading("Listado de cámaras");
            TableBuilder::new(ui)
                .column(Column::auto_with_initial_suggestion(50.0))
                .column(Column::auto_with_initial_suggestion(150.0))
                .column(Column::auto_with_initial_suggestion(600.0))
                .column(Column::auto_with_initial_suggestion(600.0))
                .header(20.0, |mut header| {
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
                })
                .body(|mut body| {
                    for cam in &self.system.cams {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&format!("{}", cam.id));
                            });
                            row.col(|ui| {
                                if CamState::Alert == cam.state {
                                    ui.label(egui::RichText::new("Alerta").color(egui::Color32::RED));
                                } else {
                                    ui.label(egui::RichText::new("Ahorro de energía").color(egui::Color32::GREEN));
                                }
                            });
                            row.col(|ui| {
                                ui.label(&format!("{}", cam.location.latitude));
                            });
                            row.col(|ui| {
                                ui.label(&format!("{}", cam.location.longitude));
                            });
                        });
                    }
                });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Carga de incidentes");
            ui.horizontal(|ui| {
                let name_label = ui.label("Nueva latitud: ");
                integer_edit_field(ui, &mut latitud)
                .labelled_by(name_label.id);
            ui.horizontal(|ui| {
                let name_label = ui.label("Nueva longitud: ");
                integer_edit_field(ui, &mut longitud)
                .labelled_by(name_label.id);
            if ui.button("Agregar incidente").clicked() {
                self.add_incident(latitud, longitud);
            }
            
            ui.separator();

            
        });
        /* egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            ui.heading("Apliación de monitoreo");
            ui.add(Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lon_lat(17.03664, 51.09916)
            ));
        }); */
    });
    
    });
    }
    }

    pub fn run_interface() -> Result<(), eframe::Error> {

        let options = eframe::NativeOptions{
            viewport: egui::ViewportBuilder::default().with_fullsize_content_view(true),
            ..Default::default()
        };

        eframe::run_native(
            "Apliación de monitoreo",
            options,
            Box::new(|_cc|
                Box::new(MyApp::new()
            )),
        )
    }
}