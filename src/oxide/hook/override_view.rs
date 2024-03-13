use crate::{
    define_hook, s,
    sdk::{client_mode::ClientMode, view_setup::ViewSetup},
};

fn subhooks(hook: &mut OverrideViewHook) {
    hook.before = Some(|_, view_setup| {
        view_setup.fov = *s!().visual.fov.lock().unwrap();
        o!().fov = Some(view_setup.fov);
    });
    hook.after = Some(|_, _, _| {});
}

define_hook!(
    OverrideViewHook,
    "OverrideView",
    (),
    (),
    subhooks,
    client_move,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
