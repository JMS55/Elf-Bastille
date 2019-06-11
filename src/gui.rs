use crate::components::{ActionMove, Elf, Location, LocationInfo};
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
            GUIState::InventoryScreen => {
                unimplemented!("TODO: Handle clicks for inventory screen");
            }
        }
    }

    pub fn render(&mut self, draw_target: &mut Frame, display: &Display) {
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
                            new_gui_state = GUIState::InventoryScreen;
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
            GUIState::InventoryScreen => {
                unimplemented!("TODO: Render inventory screen");
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
    InventoryScreen,
}
