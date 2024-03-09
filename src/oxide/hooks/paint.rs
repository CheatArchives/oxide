use crate::*;

pub type PaintFn = cfn!((), &EngineVgui, isize);

pub unsafe extern "C-unwind" fn paint_hook(engine_vgui: &EngineVgui, mode: isize) {
    (oxide!().hooks.paint.org)(engine_vgui, mode);
    draw_hitboxes()
}
