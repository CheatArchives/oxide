use crate::{define_hook, sdk::panel::{Panel, VPanel}};

//pub type PaintRraverseFn = cfn!((), &'static Panel, VPanel, bool, bool);

//pub unsafe extern "C-unwind" fn paint_traverse_hook(
//    panel: &'static Panel,
//    vpanel: VPanel,
//    force_paint: bool,
//    allow_force: bool,
//) {
//    let panel_name = CStr::from_ptr(c!(panel, get_name, vpanel));
//    match panel_name.to_str() {
//        Ok("HudScope") => return,
//        _ => {}
//    }
//    if OXIDE.is_some() {
//        //(o!().hooks.paint_traverse.org)(panel, vpanel, force_paint, allow_force);
//    }
//}

define_hook!(
    PaintTraverseHook,
    "PaintTraverse",
    (),
    (),
    panel,
    &Panel,
    vpanel,
    VPanel,
    force_paint,
    bool,
    allow_force,
    bool
);
