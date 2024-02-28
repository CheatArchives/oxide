pub static WHITE: usize = 0xFBF5F3;
pub static ORANGE: usize = 0xF75C03;
pub static LGREEN: usize = 0x5B9279;
pub static DGREEN: usize = 0x1E2D2F;
pub static BLACK: usize = 0x0C0F0A;
pub static LBLUE: usize = 0x295183;
pub static DBLUE: usize = 0x2e4254;

use std::{isize, mem::MaybeUninit, ptr::null};

use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(draw);
module_export!(component);

#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub renderer: *mut SDL_Renderer,
    pub draw: Draw,
    pub is_menu_visible: bool,
    pub aimbot_checkbox: Checkbox,
    pub third_person_checkbox: Checkbox,
    pub bhop_checkbox: Checkbox,
    pub x: isize,
    pub y: isize,
}
impl Menu {
    pub unsafe fn init(window: *mut SDL_Window) -> Result<Menu, std::boxed::Box<dyn Error>> {
        println!("loading menu");
        let old_ctx = SDL_GL_GetCurrentContext();
        let ctx = SDL_GL_CreateContext(window);
        let mut renderer = SDL_CreateRenderer(window, -1, 0);
        if renderer.is_null() {
            renderer = SDL_GetRenderer(window)
        }
        let title = CString::new(format!(
            "Team Fortress 2 - [{}] v{} by {}",
            NAME, VERSION, AUTHOR
        ))
        .unwrap();

        SDL_SetWindowTitle(window, title.as_ptr());

        SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode::SDL_BLENDMODE_BLEND);

        let draw = Draw::init(window, renderer);

        let menu = Menu {
            old_ctx,
            ctx,
            renderer,
            draw,
            is_menu_visible: true,
            aimbot_checkbox: Checkbox::new("aimbot", 10, 10),
            third_person_checkbox: Checkbox::new("third person", 10, 30),
            bhop_checkbox: Checkbox::new("bhop", 10, 50),
            x: 100,
            y: 100,
        };

        println!("loaded menu");
        Ok(menu)
    }
    pub unsafe fn unload(self) {
        SDL_GL_DeleteContext(self.ctx);
        self.draw.unload()
    }

    pub unsafe fn run(&mut self, window: *mut SDL_Window) {
        let r = self.renderer;

        self.draw_watermark();

        if self.is_menu_visible {
            self.draw_menu();
        }

        SDL_RenderPresent(r);
    }

    pub fn draw_menu(&mut self) {
        let rect = SDL_Rect {
            x: self.x as i32,
            y: self.y as i32,
            w: 500,
            h: 500,
        };
        self.draw.filled_rect(rect, DGREEN, 250);
        self.draw.outlined_rect(rect, LGREEN, 255);

        self.aimbot_checkbox
            .draw(&mut self.draw, self.x as isize, self.y as isize);
        self.third_person_checkbox
            .draw(&mut self.draw, self.x as isize, self.y as isize);
        self.bhop_checkbox
            .draw(&mut self.draw, self.x as isize, self.y as isize);
    }

    pub unsafe fn draw_watermark(&mut self) {
        let text_size = self.draw.get_text_size(NAME, FontSize::Small);

        let rect = SDL_Rect {
            x: 10,
            y: 10,
            w: text_size.0 as i32 + 10,
            h: 2,
        };
        self.draw.filled_rect(rect, LGREEN, 255);
        let rect = SDL_Rect {
            x: 10,
            y: 12,
            w: text_size.0 as i32 + 10,
            h: (text_size.1 + text_size.2) as i32 + 8,
        };
        self.draw.filled_rect(rect, DGREEN, 255);
        self.draw.draw_text(NAME, 15, 16, FontSize::Small, ORANGE);
    }

    pub unsafe fn handle_event(&mut self, event: *mut SDL_Event) {
        self.aimbot_checkbox.handle_event(event);
        self.third_person_checkbox.handle_event(event);
        self.bhop_checkbox.handle_event(event);
        match transmute::<u32, SDL_EventType>((*event).type_) {
            SDL_EventType::SDL_KEYUP => {
                let key = (*event).key.keysym.scancode;
                match key {
                    SDL_Scancode::SDL_SCANCODE_INSERT => {
                        self.is_menu_visible = !self.is_menu_visible;
                    }
                    _ => (),
                }
            }
            SDL_EventType::SDL_MOUSEMOTION => {
                let motion = (*event).motion;
            }
            _ => (),
        };
    }
}
