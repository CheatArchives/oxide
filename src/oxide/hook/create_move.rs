use crate::{
    c, define_hook,
    oxide::cheat::{aimbot::Aimbot, movement::Movement},
    sdk::{client_mode::ClientMode, entity::Entity, user_cmd::UserCmd},
};

fn subhooks(hook: &mut CreateMoveHook) {
    hook.before = Some(|_, _, cmd| {
        let Some(p_local) = Entity::local() else {
            return;
        };
        if !c!(p_local, is_alive) {
            return;
        }

        let org_cmd = cmd.clone();

        remove_punch(p_local);

        let mut aimbot = o!().cheats.get::<Aimbot>(Aimbot::name());
        aimbot.create_move(cmd).unwrap();
        let mut movement = o!().cheats.get::<Movement>(Movement::name());
        movement.create_move(cmd, &org_cmd);
    });
    hook.after = Some(|_, _, _, res| {
        *res = false;
    });
}

define_hook!(
    CreateMoveHook,
    "CreateMove",
    bool,
    true,
    subhooks,
    client_mode,
    &mut ClientMode,
    input_sample_time,
    f32,
    cmd,
    &mut UserCmd
);

pub fn remove_punch(p_local: &Entity) {
    let mut my_angles = c!(p_local, get_abs_angles).clone();
    my_angles.pitch += p_local.vec_punch_angle.pitch;
    my_angles.yaw += p_local.vec_punch_angle.yaw;
    my_angles.roll += p_local.vec_punch_angle.roll;
}
