use crate::components::*;
use imgui::{im_str, Ui};
use specs::ReadStorage;

impl Position {
    pub fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Position"))
            .default_open(true)
            .build()
        {
            ui.text(format!("x: {}, y: {}", self.x, self.y))
        }
    }
}

impl Movement {
    pub fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Movement"))
            .default_open(true)
            .build()
        {
            ui.text(format!("Move Speed: {}", self.move_speed));
        }
    }
}

impl Displayable {
    // TODO: Display texture
    pub fn create_ui(&self, ui: &Ui) {
        ui.text(format!("Name: {}", self.name));
    }
}

impl Elf {
    pub fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Elf"))
            .default_open(true)
            .build()
        {}
    }
}

impl Tree {
    pub fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Tree"))
            .default_open(true)
            .build()
        {
            ui.text(format!("Durability: {}", self.durability))
        }
    }
}

impl ItemStorage {
    // TODO: Show children in (closed by default) tree
    // TODO: Display item texture by calling Displayable::create_ui()
    pub fn create_ui(
        &self,
        ui: &Ui,
        item_data: &ReadStorage<Item>,
        item_storage_data: &ReadStorage<ItemStorage>,
    ) {
        if ui
            .collapsing_header(im_str!("Items"))
            .default_open(true)
            .build()
        {
            if let Some(weight_limit) = self.weight_limit {
                ui.text(format!(
                    "Volume: {}/{}, Weight: {}/{}",
                    self.get_stored_volume(item_data, item_storage_data),
                    self.volume_limit,
                    self.get_stored_weight(item_data, item_storage_data),
                    weight_limit
                ))
            } else {
                ui.text(format!(
                    "Volume: {}/{}",
                    self.get_stored_volume(item_data, item_storage_data),
                    self.volume_limit,
                ))
            }
        }
    }
}
