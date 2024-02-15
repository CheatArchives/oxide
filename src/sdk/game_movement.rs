use crate::*;

pub type GameMovement = WithVmt<VMTGameMovement>;

type EntityHandle = CBaseHandle;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CMoveData{
    m_bFirstRunOfFunctions: bool,
    m_bGameCodeMovedPlayer: bool,
    m_nPlayerHandle: EntityHandle,
    m_nImpulseCommand: c_int,
    m_vecViewAngles: Angles,    /* QAngle */
    m_vecAbsViewAngles: Angles, /* QAngle */
    m_nButtons: c_int,
    m_nOldButtons: c_int,
    m_flForwardMove: c_float,
    m_flOldForwardMove: c_float,
    m_flSideMove: c_float,
    m_flUpMove: c_float,
    m_flMaxSpeed: c_float,
    m_flClientMaxSpeed: c_float,
    m_vecVelocity: Vector3,
    m_vecAngles: Angles,    /* QAngle */
    m_vecOldAngles: Angles, /* QAngle*/
    m_outStepHeight: c_float,
    m_outWishVel: Vector3,
    m_outJumpVel: Vector3,
    m_vecConstraintCenter: Vector3,
    m_flConstraintRadius: c_float,
    m_flConstraintWidth: c_float,
    m_flConstraintSpeedFactor: c_float,
    /* SetAbsOrigin() */
    /* GetAbsOrigin() */
    m_vecAbsOrigin: Vector3,
}


#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTGameMovement {
    _pad1: [u8; 4 * 2],
    pub ProcessMovement: cfn!(c_void, *mut GameMovement, *mut Entity, *mut CMoveData),
    pub StartTrackPredictionErrors: cfn!(c_void, *mut GameMovement, *mut Entity),
    pub FinishTrackPredictionErrors: cfn!(c_void, *mut GameMovement, *mut Entity),
}
