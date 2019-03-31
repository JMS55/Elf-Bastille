use crate::components::*;
use imgui::{ImStr, Ui};
use specs::{Entity, ReadStorage};

impl Position {
    pub fn create_ui(&self, ui: &Ui) {
        ui.text(format!("x: {}, y: {}", self.x, self.y));
    }
}

impl Movement {
    pub fn create_ui(&self, ui: &Ui) {
        ui.text(format!("Move Speed: {}", self.move_speed));
    }
}

impl Displayable {
    pub fn create_ui(&self, ui: &Ui) {
        // TODO: Bold text
        ui.text(self.text);
    }
}

impl Durability {
    pub fn create_ui(&self, ui: &Ui) {
        ui.text(format!("Durability: {}", self.0));
    }
}

impl ItemStorage {
    pub fn create_ui(
        &self,
        ui: &Ui,
        item_data: &ReadStorage<Item>,
        item_storage_data: &ReadStorage<ItemStorage>,
        displayable_data: &ReadStorage<Displayable>,
    ) {
        fn create_child_node(
            ui: &Ui,
            entity: &Entity,
            item_data: &ReadStorage<Item>,
            item_storage_data: &ReadStorage<ItemStorage>,
            displayable_data: &ReadStorage<Displayable>,
        ) {
            let displayable = displayable_data.get(*entity).unwrap();
            if let Some(item_storage) = item_storage_data.get(*entity) {
                let mut text = format!(
                    "{} - Stored Volume: {}/{}, Stored Weight: {}",
                    displayable.text,
                    item_storage.stored_volume,
                    item_storage.volume_limit,
                    item_storage.stored_weight
                );
                if let Some(weight_limit) = item_storage.weight_limit {
                    text = format!("{}/{}", text, weight_limit);
                }
                text.push('\0');
                ui.tree_node(unsafe { ImStr::from_utf8_with_nul_unchecked(text.as_bytes()) })
                    .default_open(true)
                    .build(|| {
                        for item in &item_storage.items {
                            create_child_node(
                                ui,
                                item,
                                item_data,
                                item_storage_data,
                                displayable_data,
                            );
                        }
                    });
            } else {
                let item = item_data.get(*entity).unwrap();
                ui.text(format!(
                    "{} - Volume: {}, Weight: {}",
                    displayable.text, item.volume, item.weight
                ));
            }
        }

        // Create root node
        let mut text = format!(
            "Items - Stored Volume: {}/{}, Stored Weight: {}",
            self.stored_volume, self.volume_limit, self.stored_weight
        );
        if let Some(weight_limit) = self.weight_limit {
            text = format!("{}/{}", text, weight_limit);
        }
        text.push('\0');
        ui.tree_node(unsafe { ImStr::from_utf8_with_nul_unchecked(text.as_bytes()) })
            .build(|| {
                for item in &self.items {
                    create_child_node(ui, item, item_data, item_storage_data, displayable_data);
                }
            });
    }
}

impl Item {
    pub fn create_ui(&self, ui: &Ui) {
        ui.text(format!("Volume: {}, Weight: {}", self.volume, self.weight));
    }
}
