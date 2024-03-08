use std::usize;

use crate::*;

pub type OverrideViewFn = cfn!((), &mut ClientMode, &mut ViewSetup);

pub unsafe extern "C-unwind" fn override_view_hook(
    client_mode: &mut ClientMode,
    view_setup: &mut ViewSetup,
) {
    oxide!().fov = view_setup.fov;
    (oxide!().hooks.override_view.org)(client_mode, view_setup)
}
