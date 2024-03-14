use crate::{
    define_hook, s,
    sdk::{
        client_mode::ClientMode, condition::ConditionFlags, entity::Entity, view_setup::ViewSetup,
    },
};

fn subhooks(hook: &mut OverrideViewHook) {
    hook.before = Some(|_, view_setup| {
        let Ok(p_local) = Entity::get_local() else { return Ok(true)};
        view_setup.fov = if p_local.player_cond.get(ConditionFlags::Zoomed) {
            *s!().visual.scoped_fov.lock().unwrap()
        } else {
            *s!().visual.fov.lock().unwrap()
        };
        o!().fov = Some(view_setup.fov);
        Ok(true)
    });
    hook.after = Some(|_, _, _| Ok(()));
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
