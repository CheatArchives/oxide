use crate::*;

pub unsafe extern "C-unwind" fn create_move_hook(
    client_mode: *mut ClientMode,
    input_sample_time: c_float,
    cmd: &'static mut UserCmd,
) -> bool {
    if cmd.command_number == 0 {
        return true;
    }
    if let Err(err) = o!().cheats.aimbot.create_move(cmd){
        eprintln!("{}",err);
    }
    true
}
