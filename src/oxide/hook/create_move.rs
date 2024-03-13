use std::f32::consts::PI;

use crate::{
    c, define_hook, math::angles::Angles, oxide::cheat::aimbot::Aimbot, s, sdk::{client_mode::ClientMode, condition::ConditionFlags, entity::Entity, user_cmd::{ButtonFlags, UserCmd}}
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
        if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
            let (corrected_forward, correct_side) = correct_movement(
                org_cmd.viewangles,
                &cmd.viewangles,
                cmd.forwardmove,
                cmd.sidemove,
            );
            cmd.forwardmove = corrected_forward;
            cmd.sidemove = correct_side;
        }
        if cmd.buttons.get(ButtonFlags::InJump) && *s!().movement.bhop.lock().unwrap(){

            cmd.buttons
                .set(ButtonFlags::InJump, (p_local.flags & 1) == 1);

            if *s!().movement.revhop.lock().unwrap() && !p_local.player_cond.get(ConditionFlags::Aiming){ 
                cmd.buttons
                    .set(ButtonFlags::InAttack2, (p_local.flags & 1) == 1); }
            }
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

pub fn correct_movement(
    org_view_angles: Angles,
    new_view_angles: &Angles,
    old_forward: f32,
    old_side: f32,
) -> (f32, f32) {
    let alpha = (new_view_angles.yaw - org_view_angles.yaw) * PI / 180f32;

    let forward = old_forward * alpha.cos() - old_side * alpha.sin();
    let side = old_side * alpha.cos() + old_forward * alpha.sin();

    (forward, side)
}

pub fn remove_punch(p_local: &Entity) {
    let mut my_angles = c!(p_local, get_abs_angles).clone();
    my_angles.pitch += p_local.vec_punch_angle.pitch;
    my_angles.yaw += p_local.vec_punch_angle.yaw;
    my_angles.roll += p_local.vec_punch_angle.roll;
}
