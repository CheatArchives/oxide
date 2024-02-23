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

#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub renderer: *mut SDL_Renderer,
    pub free_type: FT_Library,
    pub face: FT_Face,
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
        let mut free_type = MaybeUninit::zeroed().assume_init();
        FT_Init_FreeType(&mut free_type);
        let mut face = MaybeUninit::zeroed().assume_init();
        let name = CString::new("/usr/share/fonts/TTF/HackNerdFontMono-Regular.ttf").unwrap();
        FT_New_Face(free_type, name.as_ptr(), 0, &mut face);
        FT_Set_Char_Size(face, 12 << 6, 12 << 6, 72, 72);

        let menu = Menu {
            old_ctx,
            ctx,
            renderer,
            free_type,
            face,
        };

        Ok(menu)
    }
    pub unsafe fn unload(self) {
        SDL_GL_DeleteContext(self.ctx);
        FT_Done_Face(self.face);
        FT_Done_FreeType(self.free_type);
    }

    pub unsafe fn run(&mut self, window: *mut SDL_Window) {
        let r = self.renderer;
        self.draw_rect();
        FT_Load_Char(self.face, 'O' as u32, FT_LOAD_RENDER);

        let x = 10;
        let y = 10;

        let glyph = (*self.face).glyph.read_volatile();
        self.draw_bitmap(window, glyph.bitmap, x as isize, y as isize);

        SDL_RenderPresent(r);
    }
    pub unsafe fn draw_bitmap(
        &mut self,
        window: *mut SDL_Window,
        bitmap: FT_Bitmap,
        x: isize,
        y: isize,
    ) {
        let r = self.renderer;
        let glyph = SDL_CreateRGBSurfaceFrom(
            bitmap.buffer as *mut c_void,
            bitmap.width,
            bitmap.rows,
            8,
            bitmap.pitch,
            0,
            0,
            0,
            0xff,
        );
        let mut colors: [SDL_Color; 256] = MaybeUninit::zeroed().assume_init();
        for i in 0..256 {
            colors[i].r = i as u8;
            colors[i].g = i as u8;
            colors[i].b = i as u8;
        }
        SDL_SetPaletteColors((*(*glyph).format).palette, colors.as_ref() as *const _ as *const SDL_Color, 0, 256);
        SDL_SetSurfaceBlendMode(glyph, SDL_BlendMode::SDL_BLENDMODE_NONE);

        let texture = SDL_CreateTextureFromSurface(r, glyph);
        let mut dest = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: 100,
            h: 100,
        };

        let res = SDL_RenderCopy(r, texture, null(), &mut dest);
        if res < 0 {
            let err = SDL_GetError();
            let err = CStr::from_ptr(err);
            dbg!(err);
        }

        SDL_FreeSurface(glyph);
    }

    pub unsafe fn draw_rect(&self) {
        let r = self.renderer;
        let rect = SDL_Rect {
            x: 100,
            y: 100,
            w: 100,
            h: 100,
        };
        SDL_SetRenderDrawColor(r, 100, 100, 100, 100);
        SDL_RenderFillRect(r, &rect);
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
