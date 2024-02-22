use crate::*;

pub struct VMTMoveHelper(*mut c_void);

pub struct MoveHelper {
    vmt: *mut VMTMoveHelper,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTPrediction {
    _pad1: [u32; 13],
    pub get_local_view_angles: cfn!(c_void, *mut Prediction, *mut Angles),
    pub set_local_view_angles: cfn!(c_void, *const Prediction, *mut Angles),
    _pad2: [u32; 3],
    pub run_command: cfn!(
        c_void,
        *mut Prediction,
        *mut Entity,
        *mut UserCmd,
        *mut MoveHelper
    ),
    pub setup_move: cfn!(
        c_void,
        *mut Prediction,
        *mut Entity,
        *mut UserCmd,
        *mut MoveHelper,
        *mut CMoveData
    ),
    pub finish_move: cfn!(c_void, *mut Prediction, *mut Entity, *mut UserCmd),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Prediction {
    vmt: &'static VMTPrediction,
    last_ground: c_int,
    in_prediction: bool,
    first_time_predicted: bool,
    old_cl_predict_value: bool,
    engine_paused: bool,
    previous_start_frame: c_int,
    commands_predicted: c_int,
    server_commands_acknowledged: c_int,
    previous_ack_had_errors: c_int,
    incoming_packet_number: c_int,
    ideal_pitch: c_float,
}
unsafe impl Send for Prediction {}

impl_has_vmt!(Prediction, VMTPrediction);
