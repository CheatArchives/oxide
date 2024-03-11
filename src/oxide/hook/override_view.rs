use crate::{
    define_hook, sdk::{client_mode::ClientMode, view_setup::ViewSetup}
};

//pub type OverrideViewFn = cfn!((), &mut ClientMode, &mut ViewSetup);
//
//pub unsafe extern "C-unwind" fn override_view_hook(
//    client_mode: &mut ClientMode,
//    view_setup: &mut ViewSetup,
//) {
//    view_setup.fov = *s!().visual.fov.lock().unwrap();
//    o!().fov = view_setup.fov;
//    //(o!().hooks.override_view.org)(client_mode, view_setup)
//}

define_hook!(
    OverrideViewHook,
    "OverrideView",
    (),
    (),
    client_move,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
