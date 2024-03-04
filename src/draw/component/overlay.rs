use std::{
    isize,
    rc::Rc,
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const LEFT_OVERLAY_WIDTH: isize = 300;
const TOP_OVERLAY_WIDTH: isize = 50;
const PADDING: isize = 10;
const BUTTON_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct Overlay {
    pub visible: bool,
    pub components: Components,
    pub aimbot_visible: Arc<Mutex<bool>>,
}

impl Overlay {
    pub fn new() -> Overlay {
        let mut components = Components::new();
        let show_aimbot = Arc::new(Mutex::new(false));
        components.add(Button::new(
            PADDING,
            TOP_OVERLAY_WIDTH + PADDING,
            LEFT_OVERLAY_WIDTH - PADDING * 2,
            BUTTON_HEIGHT,
            show_aimbot.clone(),
        ));
        components.add(window::Window::new(
            700,
            700,
            show_aimbot.clone(),
        ));
        Overlay {
            visible: true,
            components,
            aimbot_visible: show_aimbot,
        }
    }
}

impl RawComponent for Overlay {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        if !self.visible {
            return;
        }
        let size = frame.window_size;
        frame.filled_rect(
            LEFT_OVERLAY_WIDTH,
            0,
            size.0,
            TOP_OVERLAY_WIDTH,
            BACKGROUND,
            220,
        );
        frame.filled_rect(0, 0, LEFT_OVERLAY_WIDTH, size.1, BACKGROUND, 255);

        frame.outlined_rect(-1, -1, LEFT_OVERLAY_WIDTH, TOP_OVERLAY_WIDTH, CURSOR, 255);

        frame.text(
            &NAME.to_uppercase(),
            (size.0 - LEFT_OVERLAY_WIDTH) / 2 + LEFT_OVERLAY_WIDTH,
            TOP_OVERLAY_WIDTH / 2,
            FontSize::Large,
            true,
            FOREGROUND,
            255,
        );

        let version = format!("V{}", VERSION);

        let text_size = frame.fonts.get_text_size(&version, FontSize::Small);
        frame.text(
            &version,
            size.0 - text_size.0 - PADDING,
            TOP_OVERLAY_WIDTH / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );

        self.components.draw(frame);

    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        self.components.handle_event(event);
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_KEYUP => {
                    let key = (*event).key.keysym.scancode;
                    match key {
                        SDL_Scancode::SDL_SCANCODE_INSERT => {
                            self.visible = !self.visible;
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
        }
    }
}

impl Component for Overlay {}
