use egui::Ui;
use walkers::{Map, Position};

use crate::views::map_views::windows;

use super::plugins::{self, images};

pub fn show_map(
    ui: &mut Ui,
    tiles: &mut walkers::Tiles,
    map_memory: &mut walkers::MapMemory,
    click_incident: &mut plugins::ClickIncidentEvent,
    cams: &mut plugins::ImagesData,
    incidents: &mut plugins::ImagesData,
) {
    let my_position = Position::from_lon_lat(-58.4426488, -34.6177712);

    let map = Map::new(Some(tiles), map_memory, my_position)
        .with_plugin(click_incident)
        .with_plugin(images(cams))
        .with_plugin(images(incidents));

    ui.add(map);

    {
        use windows::*;

        zoom(ui, map_memory);
    }
}
