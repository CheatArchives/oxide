use libc::isblank;

use crate::*;

pub type PaintRraverseFn = cfn!((), &'static Panel, VPanel, bool, bool);

pub unsafe extern "C-unwind" fn paint_traverse_hook(
    panel: &'static Panel,
    vpanel: VPanel,
    force_paint: bool,
    allow_force: bool,
) {
    let panel_name = CStr::from_ptr(call!(*panel, get_name, vpanel));
    match panel_name.to_str() {
        Ok("HudScope") => return,
        _ => {}
    }

    (oxide!().hooks.paint_traverse.org)(panel, vpanel, force_paint, allow_force);
}
