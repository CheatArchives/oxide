static WHITE: usize = 0xFBF5F3;
static ORANGE: usize = 0xF75C03;
static LGREEN: usize = 0x5B9279;
static DGREEN: usize = 0x1E2D2F;
static BLACK: usize = 0x0C0F0A;

use std::{mem::MaybeUninit, ptr::null};

use crate::*;
use libc::CS;
use freetype_sys::*;
use sdl2_sys::*;

#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub renderer: *mut SDL_Renderer,
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

        let menu = Menu {
            old_ctx,
            ctx,
            renderer,
        };

        Ok(menu)
    }
    pub unsafe fn unload(self) {
        SDL_GL_DeleteContext(self.ctx);
    }

    pub unsafe fn run(&mut self, window: *mut SDL_Window) {
        let r = self.renderer;

        SDL_RenderPresent(r);
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
