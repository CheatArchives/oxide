static WHITE: usize = 0xFBF5F3;
static ORANGE: usize = 0xF75C03;
static LGREEN: usize = 0x5B9279;
static DGREEN: usize = 0x1E2D2F;
static BLACK: usize = 0x0C0F0A;

use std::{isize, mem::MaybeUninit, ptr::null};

use crate::*;
use freetype_sys::*;
use libc::CS;
use sdl2_sys::*;

module_export!(draw);

#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub renderer: *mut SDL_Renderer,
    pub draw: Draw,
}
impl Menu {
    pub unsafe fn init(window: *mut SDL_Window) -> Result<Menu, std::boxed::Box<dyn Error>> {
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
        };

        Ok(menu)
    }
    pub unsafe fn unload(self) {
        SDL_GL_DeleteContext(self.ctx);
        self.draw.unload()
    }

    pub unsafe fn run(&mut self, window: *mut SDL_Window) {
        let r = self.renderer;

        self.draw_watermark();

        SDL_RenderPresent(r);
    }

    pub unsafe fn draw_watermark(&mut self) {
        let text_size = self.draw.get_text_size(NAME, FontSize::Small);
        self.draw
            .draw_rect(10, 10, text_size.0 + 8, text_size.1 + 8, LGREEN, 255);
        self.draw
            .draw_rect(11, 11, text_size.0 + 6, text_size.1 + 6, BLACK, 255);
        self.draw.draw_text(
            NAME.to_uppercase().as_str(),
            14,
            14,
            FontSize::Small,
            ORANGE,
        );
    }

    pub unsafe fn handle_event(&self, event: *mut SDL_Event) {
        match transmute::<u32, SDL_EventType>((*event).type_) {
            SDL_EventType::SDL_KEYUP => {
                let key = (*event).key.keysym.scancode;
            }
            SDL_EventType::SDL_MOUSEMOTION => {
                let motion = (*event).motion;
            }
            _ => (),
        };
    }
}
