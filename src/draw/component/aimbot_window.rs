use std::sync::{Arc, Mutex};

use sdl2_sys::SDL_Scancode;

use crate::*;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct AimbotWindow {
    window: window::Window,
    aimbot_val: Arc<Mutex<bool>>,
    aimbot_draw_fov_val: Arc<Mutex<bool>>,
    aimbot_fov_val: Arc<Mutex<String>>,
    aimbot_key: Arc<Mutex<SDL_Scancode>>,
}

impl AimbotWindow {
    pub fn new(visible: Arc<Mutex<bool>>) -> AimbotWindow {
        let mut components = Components::new();

        let aimbot_val = Arc::new(Mutex::new(settings!().aimbot));
        let aimbot_draw_fov_val = Arc::new(Mutex::new(settings!().aimbot_draw_fov));
        let aimbot_fov_val = Arc::new(Mutex::new(settings!().aimbot_fov.to_string()));
        let aimbot_key = Arc::new(Mutex::new(settings!().aimbot_key));

        components.add(Checkbox::new("enable", aimbot_val.clone(), 10, 10));
        components.add(Checkbox::new(
            "draw_fov",
            aimbot_draw_fov_val.clone(),
            10,
            30,
        ));
        components.add(TextInput::new(10, 50, 100, aimbot_fov_val.clone()));
        components.add(KeyInput::new(10, 75, 100, aimbot_key.clone()));

        let window = window::Window::new("AIMBOT".to_owned(), visible, components);
        AimbotWindow {
            window,
            aimbot_val,
            aimbot_draw_fov_val,
            aimbot_fov_val,
            aimbot_key,
        }
    }
    fn update_settings(&self) {
        settings!().aimbot = *self.aimbot_val.lock().unwrap();
        settings!().aimbot_draw_fov = *self.aimbot_draw_fov_val.lock().unwrap();
        if let Ok(fov) = self.aimbot_fov_val.lock().unwrap().parse() {
            settings!().aimbot_fov = fov;
        }
        settings!().aimbot_key = *self.aimbot_key.lock().unwrap();
    }
}

impl RawComponent for AimbotWindow {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.update_settings();
        self.window.draw(frame, root_x, root_y);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.window.handle_event(event);
    }
}

impl Component for AimbotWindow {}
