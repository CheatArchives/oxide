use std::{f32::consts::PI, usize};

use crate::*;

pub type CreateMoveFn = cfn!(bool, &'static mut ClientMode, f32, &'static mut UserCmd);

pub unsafe extern "C-unwind" fn create_move_hook(
    client_mode: &'static mut ClientMode,
    input_sample_time: f32,
    cmd: &'static mut UserCmd,
) -> bool {
    if cmd.command_number == 0 || MENU.is_null() {
        return true;
    }
    // move to bhop
    let Some(p_local) = Entity::local() else {
        return true;
    };
    if !call!(p_local, is_alive) {
        return true;
    }

    let org_cmd = cmd.clone();

    if let Err(err) = { oxide!().cheats.aimbot.pre_create_move(cmd) } {
        eprintln!("{}", err);
    }

    if cmd.buttons.get(ButtonFlags::InJump) && menu!().bhop_checkbox.checked {
        cmd.buttons
            .set(ButtonFlags::InJump, (p_local.flags & 1) == 1);
    }

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

    false
}

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
