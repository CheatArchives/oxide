use std::{error::Error, ffi::CString};

use libc::c_void;
use sdl2_sys::*;

use crate::{draw::component::{aimbot_fov::AimbotFov, overlay::Overlay}, AUTHOR, NAME, VERSION};

use self::{component::Components, event::{Event, EventType}, fonts::Fonts, frame::Frame};

pub mod colors;
pub mod component;
pub mod event;
pub mod fonts;
pub mod frame;
pub mod sdl_wrappers;

pub struct Draw {
    pub fonts: Fonts,
    pub renderer: *mut SDL_Renderer,
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub components: Components,
    pub cursor: (isize, isize),
}

impl Draw {
    pub unsafe fn init(window: *mut SDL_Window) -> Result<Draw, std::boxed::Box<dyn Error>> {
        println!("loading menu");
        let old_ctx = SDL_GL_GetCurrentContext();
        let ctx = SDL_GL_CreateContext(window);
        let mut renderer = SDL_CreateRenderer(window, -1, 0);

        //STUPID WORKOAROUND
        if renderer.is_null() {
            renderer = SDL_GetRenderer(window);
        }

        let title = CString::new(format!(
            "Team Fortress 2 - [{}] v{} by {}",
            NAME, VERSION, AUTHOR
        ))
        .unwrap();

        SDL_SetWindowTitle(window, title.as_ptr());

        SDL_GL_MakeCurrent(window, old_ctx);

        let mut components = Components::new();

        components.add(AimbotFov {});
        components.add(Overlay::new());

        println!("loaded menu");
        Ok(Draw {
            components,
            fonts: Fonts::init(),
            old_ctx,
            ctx,
            renderer,
            cursor: (0, 0),
        })
    }

    pub unsafe fn restore(&self) {
        SDL_GL_DeleteContext(self.ctx);
        self.fonts.restore();
    }

    pub fn run(&'static mut self, window: *mut SDL_Window) {
        unsafe {
            SDL_GL_MakeCurrent(window, self.ctx);
        }

        let mut frame = Frame::new(window, self.renderer, &mut self.fonts);
        self.components.draw(&mut frame, 0, 0);

        unsafe {
            SDL_RenderPresent(self.renderer);
            SDL_GL_MakeCurrent(window, self.old_ctx);
        }
    }

    pub fn handle_event(&mut self, event: &mut Event) -> bool {
        if let EventType::CursorMove(pos) = event.r#type {
            self.cursor = pos
        }
        self.components.handle_event(event);
        event.handled
    }
}
