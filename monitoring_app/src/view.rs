pub mod view{
    use app::shared::{cam_list::CamList, coordenates::Coordenates, incident::{Incident, IncidentState}};
    use eframe::egui::{self, Context, Margin};
    use walkers::{Tiles, Map, MapMemory, Position, sources::OpenStreetMap};

    pub struct MyApp {
        system: CamList,
        incident: Vec<Incident>,
        tiles: Tiles,
        map_memory: MapMemory,
    }
    
    impl MyApp {
        fn new(egui_ctx: Context) -> Self {
            let system = CamList::generate_ramdoms_cams(10);

            Self {
                system,
                incident: Vec::new(),
                tiles: Tiles::new(OpenStreetMap, egui_ctx),
                map_memory: MapMemory::default(),
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

        let central_frame = egui::Frame {
            inner_margin: Margin { left: 400.0, ..Default::default()},
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        let side_frame = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::SidePanel::left("sidebar").frame(side_frame).show(ctx, |ui| {
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

            ui.heading("Cámaras del sistema");
            for cam in &self.system.cams {
                ui.horizontal(|ui| {
                    ui.label(format!("Camara id: {}", cam.id));
                    ui.label(format!("Latitud: {}", cam.location.latitude));
                    ui.label(format!("Longitud: {}", cam.location.longitude));
                    ui.label(format!("Estado: {}", cam.state));
                });
            }
        });

        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            ui.heading("Apliación de monitoreo");
            ui.add(Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lon_lat(17.03664, 51.09916)
            ));
        });
    });
    
    });
    }
    }

    pub fn run_interface() -> Result<(), eframe::Error> {

        let options = eframe::NativeOptions{
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };

        eframe::run_native(
            "Apliación de monitoreo",
            options,
            Box::new(|cc| Box::new(MyApp::new(cc.egui_ctx.clone()))),
        )
    }
}