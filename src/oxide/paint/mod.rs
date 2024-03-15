use std::{ffi::CString, fs::File, io::Write};

use crate::{
    c,
    draw::fonts::NERD_FONT,
    error::OxideResult,
    hex_to_rgb, i, o,
    sdk::font::{Font, FontDrawType, FontFlags},
};

use super::interfaces::Interfaces;

pub mod esp;
pub mod hitbox;

#[derive(Debug)]
pub struct Paint {
    pub normal: Font,
}

impl Paint {
    pub fn init(interfaces: &Interfaces) -> Paint {
        let surface = interfaces.surface.interface_ref();

        let file_name = "HackNerdFont-Regular.ttf";

        let mut file = File::create(file_name).unwrap();
        file.write_all(NERD_FONT).unwrap();

        let name = CString::new("Hack").unwrap();
        let path = CString::new(file_name).unwrap();

        c!(surface, add_custom_font_file, name.as_ptr(), path.as_ptr());

        let id = c!(surface, create_font);

        let normal = Font {
            name: name.as_ptr(),
            tall: 15,
            weight: 700,
            flags: FontFlags::ANTIALIAS as i32,
            id,
        };

        c!(
            surface,
            set_font_glyph_set,
            normal.id,
            normal.name,
            normal.tall,
            normal.weight,
            0,
            0,
            normal.flags,
            0,
            0
        );

        Paint { normal }
    }
    pub fn paint(&mut self) -> OxideResult<()> {
        if let Some(cache) = &o!().last_entity_cache {
            self.draw_hitboxes(&cache)?;
            self.esp(&cache)?;
        }
        Ok(())
    }
    pub fn paint_text(
        &mut self,
        text: Vec<i32>,
        mut x: isize,
        mut y: isize,
        color: usize,
        center: bool,
    ) {
        let text = text.as_slice();

        //this gives inconsistant width for some reason;
        if center {
            let mut w = 0;
            let mut h = 0;
            c!(
                i!(surface),
                get_text_size,
                o!().paint.normal.id,
                text.as_ptr(),
                &mut w,
                &mut h
            );
            x -= w / 2;
            y -= h / 2;
        }

        c!(i!(surface), set_text_font, o!().paint.normal.id);
        c!(i!(surface), set_text_pos, x, y);
        let (r, g, b) = hex_to_rgb!(color);
        c!(i!(surface), set_text_color, r, g, b, 255);
        c!(
            i!(surface),
            print_text,
            text.as_ptr(),
            text.len(),
            FontDrawType::Default
        );
    }
}
