use std::{
    isize,
    rc::Rc,
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const LEFT_OVERLAY_WIDTH: isize = 300;
const TOP_OVERLAY_HEIGHT: isize = 50;
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
        // ORDER OF ADDING IS IMPORTANT
        components.add(Button::new(
            "AIMBOT",
            PADDING,
            TOP_OVERLAY_HEIGHT + PADDING,
            LEFT_OVERLAY_WIDTH - PADDING * 2,
            BUTTON_HEIGHT,
            show_aimbot.clone(),
        ));
        components.add(AimbotWindow::new(show_aimbot.clone()));
        Overlay {
            visible: true,
            components,
            aimbot_visible: show_aimbot,
        }
    }
}

impl Overlay {
    fn draw_watermark(&mut self, frame: &mut Frame) {
        let text_size = frame
            .fonts
            .get_text_size(&NAME.to_uppercase(), FontSize::Small);

        let offset = (TOP_OVERLAY_HEIGHT / 2);
        let x = offset + text_size.0 / 2 + 2 + PADDING / 2;
        let y = offset - (text_size.1 + text_size.2) / 2 - 2 - PADDING / 2;

        frame.filled_rect(
            x,
            y,
            text_size.0 + PADDING,
            (text_size.1 + text_size.2) + PADDING,
            BACKGROUND,
            200,
        );
        frame.filled_rect(x, y, text_size.0 + PADDING, 1, FOREGROUND, 200);
        frame.text(
            &NAME.to_uppercase(),
            offset + text_size.0 + PADDING,
            offset,
            FontSize::Small,
            true,
            FOREGROUND,
            230,
        );
    }
}

impl RawComponent for Overlay {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let size = frame.window_size();

        if !self.visible {
            self.draw_watermark(frame);
            return;
        }

        frame.filled_rect(
            LEFT_OVERLAY_WIDTH,
            0,
            size.0,
            TOP_OVERLAY_HEIGHT,
            BACKGROUND,
            220,
        );
        frame.filled_rect(0, 0, LEFT_OVERLAY_WIDTH, size.1, BACKGROUND, 255);

        frame.outlined_rect(-1, -1, LEFT_OVERLAY_WIDTH, TOP_OVERLAY_HEIGHT, CURSOR, 255);

        let version = format!("V{}", VERSION);
        let text_size = frame.fonts.get_text_size(&version, FontSize::Small);
        frame.text(
            &version,
            size.0 - text_size.0 - PADDING,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );
        frame.text(
            &NAME.to_uppercase(),
            LEFT_OVERLAY_WIDTH / 2,
            TOP_OVERLAY_HEIGHT / 2,
            FontSize::Large,
            true,
            FOREGROUND,
            255,
        );

        self.components.draw(frame, 0, 0);
    }

    fn handle_event(&mut self, event: &mut Event) {
        if matches!(
            event.r#type,
            EventType::KeyDown(SDL_Scancode::SDL_SCANCODE_INSERT)
        ) {
            self.visible = !self.visible;
            event.handled = true;
        }
        if !self.visible {
            return;
        }
        self.components.handle_event(event);
    }
}

impl Component for Overlay {}
