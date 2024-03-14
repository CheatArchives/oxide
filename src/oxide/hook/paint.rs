use std::ffi::CString;

use crate::{
    c, define_hook,
    draw::colors::FOREGROUND,
    hex_to_rgb, i,
    oxide::paint::{esp::esp, hitbox::draw_hitboxes},
    sdk::{engine_vgui::EngineVgui, font::FontDrawType},
};

fn subhooks(hook: &mut PaintHook) {
    hook.before = Some(|_, _| Ok(true));
    hook.after = Some(|_, _, _| {
        if let Some(cache) = &o!().last_tick_cache {
            draw_hitboxes(&cache)?;
            esp(&cache)?;
        }

        //c!(i!(surface), set_text_font, o!().paint.normal.id);
        //c!(i!(surface), set_text_pos, 100, 100);
        //let (r, g, b) = hex_to_rgb!(FOREGROUND);
        //c!(i!(surface), set_text_color, r, g, b, 255);
        //let text = CString::new("test").unwrap();
        //let bytes = text
        //    .into_bytes()
        //    .iter()
        //    .map(|b| *b as i32)
        //    .collect::<Vec<i32>>();
        //c!(
        //    i!(surface),
        //    print_text,
        //    bytes.as_slice().as_ptr(),
        //    bytes.as_slice().len(),
        //    FontDrawType::Default
        //);

        Ok(())
    });
}
define_hook!(
    PaintHook,
    "Paint",
    (),
    (),
    subhooks,
    engine_vgui,
    &EngineVgui,
    mode,
    isize
);
