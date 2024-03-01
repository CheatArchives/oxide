
#[derive(Debug, Clone)]
pub struct Condition {
    _1: u32,
    _2: u32,
    _3: u32,
    _4: u32,
    _5: u32,
}
impl Condition {
    pub fn get(&self, flag: ConditionFlags) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag - (flag / 32 * 32);
        self._1 & shifted == shifted
    }
}
#[derive(Debug, Clone)]
pub enum ConditionFlags {
    Aiming,
    Zoomed,
    Disguising,
    Disguised,
    Cloaked,
    Ubercharged,
    TeleportedGlow,
    Taunting,
    UberchargeFading,
    CloakFlicker,
    Teleporting,
    Kritzkrieged,
    TmpDamageBonus,
    DeadRingered,
    Bonked,
    Dazed,
    Buffed,
    Charging,
    DemoBuff,
    CritCola,
    InHealRadius,
    Healing,
    OnFire,
    Overhealed,
    Jarated,
    Bleeding,
    DefenseBuffed,
    Milked,
    MegaHeal,
    RegenBuffed,
    MarkedForDeath,
    NoHealingDamageBuff,
    SpeedBuffAlly, // 32
    HalloweenCritCandy,
    CritCanteen,
    CritDemoCharge,
    CritHype,
    CritOnFirstBlood,
    CritOnWin,
    CritOnFlagCapture,
    CritOnKill,
    RestrictToMelee,
    DefenseBuffNoCritBlock,
    Reprogrammed,
    CritMmmph,
    DefenseBuffMmmph,
    FocusBuff,
    DisguiseRemoved,
    MarkedForDeathSilent,
    DisguisedAsDispenser,
    Sapped,
    UberchargedHidden,
    UberchargedCanteen,
    HalloweenBombHead,
    HalloweenThriller,
    RadiusHealOnDamage,
    CritOnDamage,
    UberchargedOnTakeDamage,
    UberBulletResist,
    UberBlastResist,
    UberFireResist,
    SmallBulletResist,
    SmallBlastResist,
    SmallFireResist,
    Stealthed, // 64
    MedigunDebuff,
    StealthedUserBuffFade,
    BulletImmune,
    BlastImmune,
    FireImmune,
    PreventDeath,
    MvmbotRadiowave,
    HalloweenSpeedBoost,
    HalloweenQuickHeal,
    HalloweenGiant,
    HalloweenTiny,
    HalloweenInHell,
    HalloweenGhostMode,
    MiniCritOnKill,
    ObscuredSmoke,
    Parachute,
    BlastJumping,
    HalloweenKart,
    HalloweenKartDash,
    BalloonHead,
    MeleeOnly,
    SwimmingCurse,
    FreezeInput,
    HalloweenKartCage,
    HasRune,
    RuneStrength,
    RuneHaste,
    RuneRegen,
    RuneResist,
    RuneVampire,
    RuneWarlock,
    RunePrecision,
    RuneAgility,
    GrapplingHook,
    GrapplingHookSafeFall,
    GrapplingHookLatched,
    GrapplingHookBleeding,
    AfterburnImmune,
    RuneKnockout,
    RuneImbalance,
    CritRuneTemp,
    PasstimeInterception,
    SwimmingNoEffects,
    EyeaductUnderworld,
    KingRune,
    PlagueRune,
    SupernovaRune,
    Plague,
    KingAura,
    SpawnOutline,
    KnockedIntoAir,
    CompetitiveWinner,
    NoTaunting,
    AirblastHealingDebuff,
    PasstimePenaltyMarkedForDeath,
    GrappledToPlayer,
    GrappledByPlayer,
    BasejumperDeployed,
    GasCoated,
    PyroBurningByDragonsFury,
    ThermalThrusting,
    DecreasedFriction,
    AirBlasted,
    TeleportedToHellHeal,
    MannpowerDominant,
}
