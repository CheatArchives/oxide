use std::ffi::CStr;

use crate::{
    c, define_hook, s,
    sdk::panel::{Panel, VPanel},
};

fn subhooks(hook: &mut PaintTraverseHook) {
    hook.before = Some(|panel, vpanel, _, _| {
        let panel_name = unsafe { CStr::from_ptr(c!(panel, get_name, vpanel)) };
        Ok(match panel_name.to_str() {
            Ok("HudScope") => !*s!().visual.remove_scope.lock().unwrap(),
            _ => true,
        })
    });
    hook.after = Some(|_, _, _, _, _| Ok(()));
}

define_hook!(
    PaintTraverseHook,
    "PaintTraverse",
    (),
    (),
    subhooks,
    panel,
    &Panel,
    vpanel,
    VPanel,
    force_paint,
    bool,
    allow_force,
    bool
);
