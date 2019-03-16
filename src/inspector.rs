use crate::components::*;
use imgui::{im_str, Ui};

pub trait Inspectable {
    fn create_ui(&self, ui: &Ui);
}

impl Inspectable for Position {
    fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Position"))
            .default_open(true)
            .build()
        {
            ui.text(format!("x: {}, y: {}", self.x, self.y))
        }
    }
}

impl Inspectable for Movement {
    fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Movement"))
            .default_open(true)
            .build()
        {
            ui.text(format!("Move Speed: {}", self.move_speed));
        }
    }
}

impl Inspectable for Displayable {
    // TODO: Display image
    fn create_ui(&self, ui: &Ui) {
        ui.text(format!("Name: {}", self.name));
    }
}

impl Inspectable for Elf {
    fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Elf"))
            .default_open(true)
            .build()
        {}
    }
}

impl Inspectable for Tree {
    fn create_ui(&self, ui: &Ui) {
        if ui
            .collapsing_header(im_str!("Tree"))
            .default_open(true)
            .build()
        {
            ui.text(format!("Durability: {}", self.durability))
        }
    }
}

impl Inspectable for ItemStorage {
    fn create_ui(&self, ui: &Ui) {
        unimplemented!()
    }
}
