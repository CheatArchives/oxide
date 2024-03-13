use crate::{
    c, define_hook, sdk::{base_client::{BaseClient, FrameStage}, cvar::get_cvar}
};

fn subhooks(hook: &mut FrameStageNotifyHook) {
    hook.before = Some(|_, _| {
        //let interpolation = get_cvar("cl_interpolate");
        //c!(interpolation, internal_set_float_value, 0.0, true);

    });
    hook.after = Some(|_, _, _| {
        //let interpolation = get_cvar("cl_interpolate");
        //c!(interpolation, internal_set_float_value, 0.0, true);

    });
}

define_hook!(
    FrameStageNotifyHook,
    "FrameStageNotify",
    (),
    (),
    subhooks,
    client,
    &BaseClient,
    stage,
    FrameStage
);
