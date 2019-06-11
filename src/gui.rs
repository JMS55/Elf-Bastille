use crate::components::{Elf, Location, LocationInfo};
use glium::{Display, Frame};
use imgui::{im_str, ImGui, ImGuiCond};
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
            GUIState::InventoryScreen => {}
        }
    }

    pub fn render(&mut self, draw_target: &mut Frame, display: &Display) {
        let hidpi_factor = display.gl_window().get_hidpi_factor().round();
        let frame_size =
            imgui_winit_support::get_frame_size(&display.gl_window(), hidpi_factor).unwrap();
        let gui = self.imgui.frame(frame_size, 1.0 / 60.0); // TODO: Actual delta time
        let mut new_gui_state = self.gui_state;

        match self.gui_state {
            GUIState::NoEntitySelected => {}
            GUIState::MainScreen(_) => {
                gui.window(im_str!(""))
                    .size((400.0, 300.0), ImGuiCond::FirstUseEver)
                    .build(|| {
                        if gui.button(im_str!("Inventory"), (0.0, 0.0)) {
                            new_gui_state = GUIState::InventoryScreen;
                        }
                    });
            }
            GUIState::InventoryScreen => {}
        }

        self.gui_state = new_gui_state;
        self.renderer
            .render(draw_target, gui)
            .expect("Failed to render GUI");
    }
}

#[derive(Copy, Clone)]
pub enum GUIState {
    NoEntitySelected,
    MainScreen(Entity),
    InventoryScreen,
}
