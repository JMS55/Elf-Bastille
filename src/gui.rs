use crate::components::{
    ActionMove, ActionStore, Elf, Inventory, Location, LocationInfo, StorageInfo,
};
use glium::{Display, Frame};
use imgui::{im_str, ImGui, ImGuiCond, ImString};
use imgui_glium_renderer::Renderer;

use specs::{Entities, Entity, Join, ReadStorage, World, WriteStorage};
use std::time::Duration;
pub struct GUI {
    pub imgui: ImGui,
    pub renderer: Renderer,
    pub gui_state: GUIState,
}

impl GUI {
    pub fn new(display: &Display) -> Self {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        imgui_winit_support::configure_keys(&mut imgui);
        let renderer = Renderer::init(&mut imgui, display).expect("Failed to initialize renderer");
        Self {
            imgui,
            renderer,
            gui_state: GUIState::NoEntitySelected,
        }
    }

    pub fn handle_click(&mut self, click_location: Location, world: &World) {
        let new_selected_entity = (
            &world.read_storage::<Elf>(),
            &world.read_storage::<LocationInfo>(),
            &world.entities(),
        )
            .join()
            .find_map(|(_, location, entity)| {
                if location.location.x == click_location.x
                    && location.location.y == click_location.y
                {
                    Some(entity)
                } else {
                    None
                }
            });

        match self.gui_state {
            GUIState::NoEntitySelected => {
                if let Some(new_selected_entity) = new_selected_entity {
                    self.gui_state = GUIState::MainScreen(new_selected_entity);
                }
            }
            GUIState::MainScreen(old_selected_entity) => {
                if let Some(new_selected_entity) = new_selected_entity {
                    if new_selected_entity == old_selected_entity {
                        self.gui_state = GUIState::NoEntitySelected;
                    } else {
                        self.gui_state = GUIState::MainScreen(new_selected_entity);
                    }
                } else {
                    self.gui_state = GUIState::NoEntitySelected;
                }
            }
            GUIState::MoveScreen(selected_entity, assigning_movement_in_progress) => {
                let location_data = world.read_storage::<LocationInfo>();
                let selected_entity_z_position = location_data
                    .get(selected_entity)
                    .expect("Selected entity in MoveScreen did not have a LocationInfo component")
                    .location
                    .z;
                if assigning_movement_in_progress {
                    if !(&world.read_storage::<LocationInfo>())
                        .join()
                        .any(|location| {
                            location.location.x == click_location.x
                                && location.location.y == click_location.y
                                && location.location.z == selected_entity_z_position
                        })
                    {
                        let mut elf_data = world.write_storage::<Elf>();
                        let elf = elf_data
                            .get_mut(selected_entity)
                            .expect("Selected entity in MoveScreen did not have an Elf component");
                        elf.queue_action(ActionMove::new(Location::new(
                            click_location.x,
                            click_location.y,
                            selected_entity_z_position,
                        )));
                    }
                    self.gui_state = GUIState::MoveScreen(selected_entity, false);
                }
            }
            GUIState::InventorySelectionScreen(_) => {}
            GUIState::InventoryElf(_, _) => {}
            GUIState::InventoryOther(_, _) => {}
        }
    }

    pub fn render(
        &mut self,
        draw_target: &mut Frame,
        display: &Display,
        inventory_data: &ReadStorage<Inventory>,
        location_data: &ReadStorage<LocationInfo>,
        storage_info_data: &ReadStorage<StorageInfo>,
        elf_data: &mut WriteStorage<Elf>,
        entities: &Entities,
    ) {
        let hidpi_factor = display.gl_window().get_hidpi_factor().round();
        let frame_size =
            imgui_winit_support::get_frame_size(&display.gl_window(), hidpi_factor).unwrap();
        let ui = self.imgui.frame(frame_size, 1.0 / 60.0); // TODO: Actual delta time
        let mut new_gui_state = self.gui_state;

        match self.gui_state {
            GUIState::NoEntitySelected => {}
            GUIState::MainScreen(selected_entity) => {
                ui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        if ui.button(im_str!("Move"), (0.0, 0.0)) {
                            new_gui_state = GUIState::MoveScreen(selected_entity, false);
                        }
                        if ui.button(im_str!("Inventory"), (0.0, 0.0)) {
                            new_gui_state = GUIState::InventorySelectionScreen(selected_entity);
                        }
                    });
            }
            GUIState::MoveScreen(selected_entity, assigning_movement_in_progress) => {
                ui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        if assigning_movement_in_progress {
                            ui.text(im_str!("Click a location to move to"));
                        } else {
                            if ui.button(im_str!("Queue a movement"), (0.0, 0.0)) {
                                new_gui_state = GUIState::MoveScreen(selected_entity, true);
                            }
                            ui.separator();
                            if ui.button(im_str!("Back"), (0.0, 0.0)) {
                                new_gui_state = GUIState::MainScreen(selected_entity);
                            }
                        }
                    });
            }
            GUIState::InventorySelectionScreen(selected_entity) => {
                let selected_entity_location = location_data
                    .get(selected_entity)
                    .expect("Selected entity in InventorySelectionScreen did not have a LocationInfo component")
                    .location;
                ui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        if ui.button(im_str!("Elf"), (0.0, 0.0)) {
                            new_gui_state = GUIState::InventoryElf(selected_entity, None);
                        }
                        for other_entity in (inventory_data, location_data, entities)
                            .join()
                            .filter_map(|(_, other_location, other_entity)| {
                                if other_location
                                    .location
                                    .is_adjacent_to(&selected_entity_location)
                                {
                                    Some(other_entity)
                                } else {
                                    None
                                }
                            })
                        {
                            if ui.button(im_str!("Other"), (0.0, 0.0)) {
                                new_gui_state =
                                    GUIState::InventoryOther(selected_entity, other_entity);
                            }
                        }
                        ui.separator();
                        if ui.button(im_str!("Back"), (0.0, 0.0)) {
                            new_gui_state = GUIState::MainScreen(selected_entity);
                        }
                    });
            }
            GUIState::InventoryElf(selected_entity, other_entity) => {
                let selected_entity_inventory = inventory_data
                    .get(selected_entity)
                    .expect("Selected entity in InventoryElf did not have an Inventory component");
                let selected_entity_elf = elf_data
                    .get_mut(selected_entity)
                    .expect("Selected entity in InventoryElf did not have an Elf component");
                ui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        ui.text(format!(
                            "Volume used: {}/{} Weight used: {}/{}",
                            selected_entity_inventory.max_volume
                                - selected_entity_inventory.volume_free,
                            selected_entity_inventory.max_volume,
                            selected_entity_inventory.max_weight
                                - selected_entity_inventory.weight_free,
                            selected_entity_inventory.max_weight
                        ));
                        for stored_entity in &selected_entity_inventory.stored_entities {
                            let stored_entity_storage_info = storage_info_data.get(*stored_entity).expect("Stored entity in InventoryElf did not have a StorageInfo component");
                            if let Some(other_entity) = other_entity {
                                if ui.button(&ImString::from(format!("{} - Volume: {} Weight: {}", stored_entity_storage_info.name, stored_entity_storage_info.volume, stored_entity_storage_info.weight)), (0.0, 0.0)) {
                                    selected_entity_elf.queue_action(ActionStore::new(*stored_entity, other_entity, Duration::from_secs(3)));
                                }
                            } else {
                                ui.text(format!("{} - Volume: {} Weight: {}", stored_entity_storage_info.name, stored_entity_storage_info.volume, stored_entity_storage_info.weight));
                            }
                        }
                        ui.separator();
                        if ui.button(im_str!("Back"), (0.0, 0.0)) {
                            new_gui_state = GUIState::MainScreen(selected_entity);
                        }
                    });
            }
            GUIState::InventoryOther(selected_entity, other_entity) => {
                let other_entity_inventory = inventory_data
                    .get(other_entity)
                    .expect("Other entity in InventoryOther did not have an Inventory component");
                ui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        ui.text(format!(
                            "Volume used: {}/{} Weight used: {}/{}",
                            other_entity_inventory.max_volume
                                - other_entity_inventory.volume_free,
                            other_entity_inventory.max_volume,
                            other_entity_inventory.max_weight
                                - other_entity_inventory.weight_free,
                            other_entity_inventory.max_weight
                        ));
                        for stored_entity in &other_entity_inventory.stored_entities {
                            let stored_entity_storage_info = storage_info_data.get(*stored_entity).expect("Stored entity in InventoryOther did not have a StorageInfo component");
                            ui.text(format!("{} - Volume: {} Weight: {}", stored_entity_storage_info.name, stored_entity_storage_info.volume, stored_entity_storage_info.weight));
                        }
                        if ui.button(im_str!("Insert"), (0.0, 0.0)) {
                            new_gui_state = GUIState::InventoryElf(selected_entity, Some(other_entity));
                        }
                        ui.separator();
                        if ui.button(im_str!("Back"), (0.0, 0.0)) {
                            new_gui_state = GUIState::MainScreen(selected_entity);
                        }
                    });
            }
        }

        self.gui_state = new_gui_state;
        self.renderer
            .render(draw_target, ui)
            .expect("Failed to render GUI");
    }

    pub fn want_capture_mouse(&self) -> bool {
        let imgui_io = unsafe { &*imgui_sys::igGetIO() };
        imgui_io.want_capture_mouse
    }

    pub fn want_capture_keyboard(&self) -> bool {
        let imgui_io = unsafe { &*imgui_sys::igGetIO() };
        imgui_io.want_capture_keyboard
    }
}

#[derive(Copy, Clone)]
pub enum GUIState {
    // Entity is the selected entity
    NoEntitySelected,
    MainScreen(Entity),
    MoveScreen(Entity, bool), // bool is assigning_movement_in_progress
    InventorySelectionScreen(Entity),
    InventoryElf(Entity, Option<Entity>), // second Entity is the adjacent entity and allows for inserting into it
    InventoryOther(Entity, Entity), // second Entity is the adjacent entity and allows for inserting into it
}
