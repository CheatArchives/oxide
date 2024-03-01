use crate::*;

pub struct VMTMoveHelper(&'static mut c_void);

pub struct MoveHelper {
    vmt: *mut VMTMoveHelper,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTPrediction {
    _pad1: [u32; 13],
    pub get_local_view_angles: cfn!(c_void, &'static mut Prediction, &'static mut Angles),
    pub set_local_view_angles: cfn!(c_void, &'static Prediction, &'static mut Angles),
    _pad2: [u32; 3],
    pub run_command: cfn!(
        c_void,
        &'static mut Prediction,
        &'static mut Entity,
        &'static mut UserCmd,
        &'static mut MoveHelper
    ),
    pub setup_move: cfn!(
        c_void,
        &'static mut Prediction,
        &'static mut Entity,
        &'static mut UserCmd,
        &'static mut MoveHelper,
        &'static mut CMoveData
    ),
    pub finish_move: cfn!(c_void, &'static mut Prediction, &'static mut Entity, &'static mut UserCmd),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Prediction {
    vmt: *const VMTPrediction,
    last_ground: isize,
    in_prediction: bool,
    first_time_predicted: bool,
    old_cl_predict_value: bool,
    engine_paused: bool,
    previous_start_frame: isize,
    commands_predicted: isize,
    server_commands_acknowledged: isize,
    previous_ack_had_errors: isize,
    incoming_packet_number: isize,
    ideal_pitch: f32,
}
unsafe impl Send for Prediction {}

impl_has_vmt!(Prediction, VMTPrediction);
