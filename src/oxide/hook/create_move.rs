use crate::{define_hook, sdk::{client_mode::ClientMode, user_cmd::UserCmd}};


define_hook!(
    CreateMoveHook,
    "CreateMove",
    bool,
    true,
    client_mode,
    &mut ClientMode,
    input_sample_time,
    f32,
    cmd,
    &mut UserCmd
);

//pub unsafe extern "C-unwind" fn create_move_hook(
//    client_mode: &'static mut ClientMode,
//    input_sample_time: f32,
//    cmd: &'static mut UserCmd,
//) -> bool {
//    //(o!().hooks.create_move.org)(client_mode, input_sample_time, cmd);
//    if cmd.command_number == 0 || DRAW.is_none() {
//        return true;
//    }
//    let Some(p_local) = Entity::local() else {
//        return true;
//    };
//    if !c!(p_local, is_alive) {
//        return true;
//    }
//    remove_punch(p_local);
//
//    let org_cmd = cmd.clone();
//
//    //if let Err(err) = { o!().cheats.aimbot.pre_create_move(cmd) } {
//    //    eprintln!("{}", err);
//    //}
//
//    if cmd.buttons.get(ButtonFlags::InJump) && true
//    {
//        cmd.buttons
//            .set(ButtonFlags::InJump, (p_local.flags & 1) == 1);
//    }
//
//    if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
//        let (corrected_forward, correct_side) = correct_movement(
//            org_cmd.viewangles,
//            &cmd.viewangles,
//            cmd.forwardmove,
//            cmd.sidemove,
//        );
//        cmd.forwardmove = corrected_forward;
//        cmd.sidemove = correct_side;
//    }
//    return true
//
//}
//
//pub fn correct_movement(
//    org_view_angles: Angles,
//    new_view_angles: &Angles,
//    old_forward: f32,
//    old_side: f32,
//) -> (f32, f32) {
//    let alpha = (new_view_angles.yaw - org_view_angles.yaw) * PI / 180f32;
//
//    let forward = old_forward * alpha.cos() - old_side * alpha.sin();
//    let side = old_side * alpha.cos() + old_forward * alpha.sin();
//
//    (forward, side)
//}
//
//pub fn remove_punch(p_local: &Entity) {
//    let mut my_angles = c!(p_local, get_abs_angles).clone();
//    my_angles.pitch += p_local.vec_punch_angle.pitch;
//    my_angles.yaw += p_local.vec_punch_angle.yaw;
//    my_angles.roll += p_local.vec_punch_angle.roll;
//}
