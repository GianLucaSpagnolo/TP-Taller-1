use egui::Context;
use walkers::{sources::OpenStreetMap, MapMemory, Tiles};

pub struct MapInterface {
    pub tiles: Tiles,
    pub map_memory: MapMemory,
}

impl MapInterface {
    /// ### new
    ///
    /// Crea una nueva interfaz de mapa
    ///
    /// #### Parametros
    /// - `tiles`: Tiles
    /// - `map_memory`: Memoria del mapa
    ///
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            tiles: Tiles::new(OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(),
        }
    }
}
