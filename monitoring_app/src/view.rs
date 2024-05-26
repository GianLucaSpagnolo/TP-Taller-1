pub mod view{
    use app::shared::{cam_list::{CamList, CamState}, coordenates::Coordenates, incident::{Incident, IncidentState}};
    use eframe::egui::{self, Margin};
    use egui_extras::{Column, TableBuilder};
    /* use walkers::{Tiles, MapMemory, sources::OpenStreetMap}; */

    #[derive(Default)]
    pub struct MyApp {
        pub system: CamList,
        pub incident: Vec<Incident>,
        pub coordenates: Coordenates,
        /* tiles: Tiles,
        map_memory: MapMemory, */
    }
    
    impl MyApp {

        fn new() -> Self {
            let system = CamList::generate_ramdoms_cams(10);

            Self {
                system,
                incident: Vec::new(),
                coordenates: Coordenates::default(),
                /* tiles: Tiles::new(OpenStreetMap, egui_ctx),
                map_memory: MapMemory::default(), */
            }
        }
        pub fn add_incident(&mut self, location: Coordenates) {
            
            println!("Incidente agregado en latitud: {}, longitud: {}", location.latitude, location.longitude);
            
            self.incident.push(Incident {
                id: self.incident.len().to_string(),
                location,
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

            let frame = egui::Frame {
                inner_margin: Margin {
                    top: 30.0,
                    bottom: 30.0,
                    left: 30.0,
                    right: 30.0,
                },
                ..Default::default()
            };

            egui::CentralPanel::default().frame(frame).show(ctx,  |ui| {
                ui.heading("Carga de incidentes");
                ui.horizontal(|ui| {
                    let name_label = ui.label("Nueva latitud: ");
                    integer_edit_field(ui, &mut self.coordenates.latitude)
                    .labelled_by(name_label.id);
                });
                ui.horizontal(|ui| {
                    let name_label = ui.label("Nueva longitud: ");
                    integer_edit_field(ui,&mut self.coordenates.longitude)
                    .labelled_by(name_label.id);
                });
                if ui.button("Agregar incidente").clicked() {
                    self.add_incident(self.coordenates.clone());
                }
                ui.separator();
                
                ui.heading("Listado de cámaras");
                TableBuilder::new(ui)
                    .column(Column::exact(50.0))
                    .column(Column::exact(250.0))
                    .column(Column::exact(250.0))
                    .column(Column::exact(250.0))
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
        
            /* egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
                ui.heading("Apliación de monitoreo");
                ui.add(Map::new(
                    Some(&mut self.tiles),
                    &mut self.map_memory,
                    Position::from_lon_lat(17.03664, 51.09916)
                ));
            }); */
    }   

            
    }

    pub fn run_interface() -> Result<(), eframe::Error> {

        let mut options = eframe::NativeOptions::default();

        options.centered = true;

        eframe::run_native(
            "Apliación de monitoreo",
            options,
            Box::new(|_cc|
                Box::new(MyApp::new()
            )),
        )
    }
}