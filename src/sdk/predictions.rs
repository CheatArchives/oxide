use crate::*;

pub struct VMTMoveHelper(*mut c_void);

pub struct MoveHelper {
    vmt: *mut VMTMoveHelper,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTPrediction {
    _pad1: [u32; 13],
    pub GetLocalViewAngles: cfn!(c_void, *mut Prediction, *mut Angles),
    pub SetLocalViewAngles: cfn!(c_void, *const Prediction, *mut Angles),
    _pad2: [u32; 3],
    pub RunCommand: cfn!(
        c_void,
        *mut Prediction,
        *mut Entity,
        *mut UserCmd,
        *mut MoveHelper
    ),
    pub SetupMove: cfn!(
        c_void,
        *mut Prediction,
        *mut Entity,
        *mut UserCmd,
        *mut MoveHelper,
        *mut CMoveData
    ),
    pub FinishMove: cfn!(c_void, *mut Prediction, *mut Entity, *mut UserCmd),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Prediction {
    vmt: *mut VMTPrediction,
    m_hLastGround: c_int,
    m_bInPrediction: bool,
    m_bFirstTimePredicted: bool,
    m_bOldCLPredictValue: bool,
    m_bEnginePaused: bool,
    m_nPreviousStartFrame: c_int,
    m_nCommandsPredicted: c_int,
    m_nServerCommandsAcknowledged: c_int,
    m_bPreviousAckHadErrors: c_int,
    m_nIncomingPacketNumber: c_int,
    m_flIdealPitch: c_float,
}
unsafe impl Send for Prediction {}
impl HasVmt<VMTPrediction> for Prediction {
    fn get_vmt(&self) -> *mut VMTPrediction {
        self.vmt
    }

    fn set_vmt(&mut self, vmt: *mut VMTPrediction) {
        unsafe{
            self.vmt = vmt
        }
    }
    unsafe fn c(&mut self) -> VMTPrediction {
        *self.vmt
    }
}
