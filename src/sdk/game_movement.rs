use crate::*;

pub type GameMovement = WithVmt<VMTGameMovement>;

type EntityHandle = CBaseHandle;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CMoveData{
    first_run_of_functions: bool,
    game_code_moved_player: bool,
    player_handle: EntityHandle,
    impulse_command: c_int,
    view_angles: Angles,    
    abs_view_angles: Angles, 
    buttons: c_int,
    old_buttons: c_int,
    forward_bove: c_float,
    old_forward_bove: c_float,
    sidemove: c_float,
    up_move: c_float,
    max_speed: c_float,
    client_max_speed: c_float,
    velocity: Vector3,
    angles: Angles,    
    old_angles: Angles, 
    step_height: c_float,
    wish_vel: Vector3,
    jump_vel: Vector3,
    constraint_center: Vector3,
    constraint_radius: c_float,
    constraint_width: c_float,
    constraint_speed_factor: c_float,
    abs_origin: Vector3,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTGameMovement {
    _pad1: [u32; 2],
    pub process_movement: cfn!(c_void, *mut GameMovement, *mut Entity, *mut CMoveData),
    pub start_track_prediction_errors: cfn!(c_void, *mut GameMovement, *mut Entity),
    pub finish_track_prediction_errors: cfn!(c_void, *mut GameMovement, *mut Entity),
}
