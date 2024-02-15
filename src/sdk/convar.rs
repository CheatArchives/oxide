use crate::*;


#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTConVar {
    _pad: [u8; 4*14],
    pub InternalSetValue: cfn!(c_void, *const ConVar , *const c_char),
    pub InternalSetFloatValue: cfn!(c_void, *const ConVar,c_float , bool),
    pub InternalSetIntValue: cfn!(c_void, *const ConVar, c_int),
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ConVar {
    vmt: VMTConVar,
    _pad: [u8; 0x18],
    m_pParent: *const ConVar,
    m_pszDefaultValue: *const c_char,
    m_pszString: *const c_char,
    m_StringLength: c_int,
    m_fValue: c_float,
    m_nValue: c_int,
    m_bHasMin: bool,
    m_fMinVal: c_float,
    m_bHasMax: bool,
    m_fMaxVal: c_float,
    m_bHasCompMin: bool,
    m_fCompMinVal: c_float,
    m_bHasCompMax: bool,
    m_fCompMaxVal: c_float,
    m_bCompetitiveRestrictions: bool,
    m_fnChangeCallback: *const c_void,
}
