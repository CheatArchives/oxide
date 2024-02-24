use std::usize;

use crate::*;

pub unsafe extern "C-unwind" fn create_move_hook(
    client_mode: *mut ClientMode,
    input_sample_time: f32,
    cmd: &'static mut UserCmd,
) -> bool {
    if cmd.command_number == 0 || MENU.is_null() {
        return true;
    }
    if let Err(err) = {
        oxide!().cheats.aimbot.create_move(cmd)
    } {
        eprintln!("{}", err);
    }
    if let Some(p_local) = Entity::local() {
        if call!(*p_local, is_alive) {
            p_local.force_taunt_cam = menu!().third_person_checkbox.checked as isize;
            if cmd.buttons.get(ButtonFlags::InJump) && menu!().bhop_checkbox.checked{
                cmd.buttons
                    .set(ButtonFlags::InJump, (p_local.flags & 1) == 1);
            }
        }
    }

    true
}
