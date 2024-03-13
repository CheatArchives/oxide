use crate::{define_hook, oxide::paint::{esp::esp, hitbox::draw_hitboxes}, sdk::engine_vgui::EngineVgui};


fn subhooks(hook:&mut PaintHook) {
    
    hook.before = Some(|_,_|{

    });
    hook.after = Some(|_,_,_|{
        draw_hitboxes();
        esp();
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
