use std::ffi::CStr;

use crate::{
    c, define_hook, sdk::panel::{Panel, VPanel}
};

fn subhooks(hook: &mut PaintTraverseHook) {
    hook.before = Some(|panel, vpanel, _, _| {
        let panel_name = unsafe { CStr::from_ptr(c!(panel, get_name, vpanel)) };
        match panel_name.to_str() {
            Ok("HudScope") => return,
            _ => {}
        }
    });
    hook.after = Some(|_, _, _, _, _| {});
}

//pub type PaintRraverseFn = cfn!((), &'static Panel, VPanel, bool, bool);

//pub unsafe extern "C-unwind" fn paint_traverse_hook(
//    panel: &'static Panel,
//    vpanel: VPanel,
//    force_paint: bool,
//    allow_force: bool,
//) {
//}

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
