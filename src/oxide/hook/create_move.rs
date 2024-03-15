use crate::{
    c, define_hook,
    oxide::cheat::{aimbot::Aimbot, movement::Movement},
    s,
    sdk::{client_mode::ClientMode, entity::{Entity, player::Player}, user_cmd::UserCmd},
};

fn subhooks(hook: &mut CreateMoveHook) {
    hook.before = Some(|_, _, cmd| {
        if cmd.command_number == 0 {
            return Ok(true);
        }
        let p_local = Entity::get_local()?;

        if !c!(&p_local.as_ent(), is_alive) {
            return Ok(true);
        }

        let org_cmd = cmd.clone();

        if *s!().visual.third_person.lock().unwrap() {
            p_local.force_taunt_cam = 1
        } else {
            p_local.force_taunt_cam = 0
        }

        remove_punch(p_local);

        let mut aimbot = o!().cheats.get::<Aimbot>(Aimbot::name());
        aimbot.create_move(cmd)?;

        let mut movement = o!().cheats.get::<Movement>(Movement::name());
        movement.create_move(cmd, &org_cmd)?;

        Ok(false)
    });
    hook.after = Some(|_, _, _, res| {
        *res = !*s!().aimbot.silent.lock()?;
        Ok(())
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

pub fn remove_punch(p_local: &Player) {
    let mut my_angles = c!(&p_local.as_ent(), get_abs_angles).clone();
    my_angles.pitch += p_local.vec_punch_angle.pitch;
    my_angles.yaw += p_local.vec_punch_angle.yaw;
    my_angles.roll += p_local.vec_punch_angle.roll;
}
