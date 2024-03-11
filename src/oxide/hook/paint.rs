use crate::{define_hook, sdk::engine_vgui::EngineVgui};

//pub type PaintFn = cfn!((), &EngineVgui, isize);

//pub unsafe extern "C-unwind" fn paint_hook(engine_vgui: &EngineVgui, mode: isize) {
//    //(o!().hooks.paint.org)(engine_vgui, mode);
//    draw_hitboxes()
//}
define_hook!(
    PaintHook,
    "Paint",
    (),
    (),
    engine_vgui,
    &EngineVgui,
    mode,
    isize
);
