use crate::*;
use freetype_sys::FT_Bitmap;
use sdl2_sys::*;
use sdl2_sys::*;
use std::{f32::consts::PI, intrinsics::offset, isize, mem::MaybeUninit, ptr::null, usize};

module_export!(menu);
module_export!(component);
module_export!(colors);
module_export!(fonts);
module_export!(sdl_wrappers);
module_export!(frame);

pub struct Draw {
    pub fonts: Fonts,
    pub renderer: *mut SDL_Renderer,
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub components: Vec<Box<dyn ComponentDebug>>
}

#[derive(Debug, Clone)]
pub enum FontSize {
    Small = 16,
    Medium = 24,
    Large = 36,
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

        let mut components = Vec::new();
        //components.push(Box::new(AimbotFov {}) as Box<dyn ComponentDebug>);
        components.push(Box::new(Overlay {}) as Box<dyn ComponentDebug>);

        println!("loaded menu");
        Ok(Draw {
            components,
            fonts: Fonts::init(),
            old_ctx,
            ctx,
            renderer,
        })
    }

    pub unsafe fn restore(&self) {
        SDL_GL_DeleteContext(self.ctx);
        self.fonts.restore();
    }

    pub unsafe fn run(&mut self, window: *mut SDL_Window) {
        SDL_GL_MakeCurrent(window, self.ctx);

        let mut frame = Frame::new(window, self.renderer);
        for component in &mut self.components {
            component.draw(&mut frame, 0, 0)
        }

        SDL_RenderPresent(self.renderer);
        SDL_GL_MakeCurrent(window, self.old_ctx);
    }

    pub fn draw_text(&mut self, text: &str, x: isize, y: isize, size: FontSize, color: usize) {
        //unsafe {
        //    let face = self.get_face(&size);

        //    FT_Load_Char(face, text.chars().next().unwrap() as u32, FT_LOAD_RENDER);
        //    let glyph = (*face).glyph.read_volatile();

        //    let mut x_offset = -(glyph.metrics.vertBearingX >> 6) as isize;
        //    let mut y_offset = self.get_text_size(text, size).1 as isize;

        //    for (i, letter) in text.chars().enumerate() {
        //        if letter == ' ' {
        //            x_offset += (face.read().size.read().metrics.max_advance >> 6) as isize;
        //            continue;
        //        }
        //        FT_Load_Char(face, letter as u32, FT_LOAD_RENDER);
        //        let glyph = (*face).glyph.read_volatile();

        //        let x = x + x_offset + (glyph.metrics.vertBearingX >> 6) as isize;
        //        let y = y + y_offset - (glyph.metrics.horiBearingY >> 6) as isize;

        //        x_offset += (glyph.metrics.horiAdvance >> 6) as isize;
        //        self.draw_bitmap(glyph.bitmap, x, y, color);
        //    }
        //}
    }
    pub unsafe fn draw_bitmap(&mut self, bitmap: FT_Bitmap, x: isize, y: isize, color: usize) {
        let r = self.renderer;

        let len = (bitmap.width * bitmap.rows * 4) as usize;
        let mut rgba = vec![0u8; len];

        let buffer = std::slice::from_raw_parts(bitmap.buffer, len);
        for i in (0..len).step_by(4) {
            let val = buffer[i / 4];
            (rgba[i], rgba[i + 1], rgba[i + 2]) = hex_to_rgb!(color);
            rgba[i + 3] = val;
        }

        let glyph = SDL_CreateRGBSurfaceFrom(
            rgba.as_ptr() as *mut c_void,
            bitmap.width,
            bitmap.rows,
            32,
            bitmap.width * 4,
            0x000000ff,
            0x0000ff00,
            0x00ff0000,
            0xff000000,
        );
        SDL_SetSurfaceBlendMode(glyph, SDL_BlendMode::SDL_BLENDMODE_BLEND);

        let texture = SDL_CreateTextureFromSurface(r, glyph);
        let mut dest = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: bitmap.width,
            h: bitmap.rows,
        };

        SDL_RenderCopy(r, texture, null(), &mut dest);

        SDL_DestroyTexture(texture);
        SDL_FreeSurface(glyph);
    }

    pub unsafe fn handle_event(&mut self, event: *mut SDL_Event) {
        match transmute::<u32, SDL_EventType>((*event).type_) {
            SDL_EventType::SDL_KEYUP => {
                let key = (*event).key.keysym.scancode;
                match key {
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
