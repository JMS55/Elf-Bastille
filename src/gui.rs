use crate::components::{Elf, Location, LocationInfo};
use glium::{Display, Frame};
use imgui::ImGui;
use imgui_glium_renderer::Renderer;
use specs::{Entity, Join, World};

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
        }
    }

    pub fn render(&self, draw_target: &Frame) {}
}

pub enum GUIState {
    NoEntitySelected,
    MainScreen(Entity),
}
