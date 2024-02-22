static WHITE: usize = 0xFBF5F3;
static ORANGE: usize = 0xF75C03;
static LGREEN: usize = 0x5B9279;
static DGREEN: usize = 0x1E2D2F;
static BLACK: usize = 0x0C0F0A;

use crate::*;
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
        let renderer = SDL_CreateRenderer(window, -1, 0);
        let title = CString::new("Team Fortress 2 - [OXIDE]").unwrap();
        SDL_SetWindowTitle(window, title.as_ptr());

        let menu = Menu{
                old_ctx,
                ctx,
                renderer,
            };


        Ok(menu)
    }
    pub unsafe fn unload(self) {
        SDL_GL_DeleteContext(self.ctx);
    }

    pub unsafe fn run(&mut self,window: *mut SDL_Window) {
        let r = self.renderer;

        let rect = SDL_Rect {
            x: 10,
            y: 10,
            w: 500,
            h: 500,
        };

        SDL_SetRenderDrawColor( r, (BLACK >> 16) as u8, (BLACK >> 8 ) as u8, BLACK as u8, 200 );
        SDL_RenderFillRect(r, &rect);
        SDL_RenderPresent(r);
    }
}
