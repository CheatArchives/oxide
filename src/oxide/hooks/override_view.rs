use std::usize;

use crate::*;


pub type OverrideViewFn = cfn!((), &mut ClientMode,&mut ViewSetup);

pub unsafe extern "C-unwind" fn override_view_hook(
    client_mode: &mut ClientMode,
    view_setup: &mut ViewSetup,
)  {
    view_setup.fov = 100f32
}
