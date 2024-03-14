use std::ffi::CStr;

use crate::o;

use super::*;

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTWeapon {
    #[derivative(Debug="ignore")]
    _pad1: [u8; 4 * 79],
    pub get_index: cfn!(isize, &'static Weapon),
    #[derivative(Debug="ignore")]
    _pad2: [u8; 4 * 318],
    pub get_slot: cfn!(isize, &'static Weapon),
    #[derivative(Debug="ignore")]
    _pad3: [u8; 4 * 1],
    pub get_name: cfn!(CStr, &'static Weapon),
    #[derivative(Debug="ignore")]
    _pad4: [u8; 4 * 48],
    pub get_weapon_id: cfn!(WeaponType, &Weapon),
    pub get_damage_type: cfn!(isize, &Weapon),
    #[derivative(Debug="ignore")]
    _pad5: [u8; 4 * 14],
    pub calc_is_attack_critical_helper: cfn!(bool, &'static Weapon),
    #[derivative(Debug="ignore")]
    _pad6: [u8; 4 * 28],
    pub can_fire_critical_shot: cfn!(bool, &Weapon, bool),
    #[derivative(Debug="ignore")]
    _pad7: [u8; 4 * 30],
    pub get_swing_range: cfn!(isize, &'static Weapon),
}

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Weapon {
    pub vmt: *mut VMTWeapon,
    #[derivative(Debug="ignore")]
    _pad1: [u8; 0x924],
    pub item_definition_index: isize,
    #[derivative(Debug="ignore")]
    _pad2: [u8; 0x10C],
    pub owner: CBaseHandle,
    #[derivative(Debug="ignore")]
    _pad3: [u8; 0x10],
    pub next_primary_attack: f32,
    #[derivative(Debug="ignore")]
    _pad4: [u8; 0x1DC],
    pub smack_time: f32,
    #[derivative(Debug="ignore")]
    _pad6: [u8; 0x10],
    pub ready_to_backstab: bool,
}

impl Weapon {
    pub fn can_attack_primary(&mut self) -> bool {
        let now = o!().global_vars.now();
        self.next_primary_attack <= now
    }
}

impl_has_vmt!(Weapon, VMTWeapon);

pub enum WeaponType{
	None = 0,
	Bat,
	BatWood,
	Bottle, 
	Fireaxe,
	Club,
	Crowbar,
	Knife,
	Fists,
	Shovel,
	Wrench,
	Bonesaw,
	ShotgunPrimary,
	ShotgunSoldier,
	ShotgunHwg,
	ShotgunPyro,
	Scattergun,
	Sniperrifle,
	Minigun,
	Smg,
	SyringegunMedic,
	Tranq,
	Rocketlauncher,
	Grenadelauncher,
	Pipebomblauncher,
	Flamethrower,
	GrenadeNormal,
	GrenadeConcussion,
	GrenadeNail,
	GrenadeMirv,
	GrenadeMirvDemoman,
	GrenadeNapalm,
	GrenadeGas,
	GrenadeEmp,
	GrenadeCaltrop,
	GrenadePipebomb,
	GrenadeSmokeBomb,
	GrenadeHeal,
	GrenadeStunball,
	GrenadeJar,
	GrenadeJarMilk,
	Pistol,
	PistolScout,
	Revolver,
	Nailgun,
	Pda,
	PdaEngineerBuild,
	PdaEngineerDestroy,
	PdaSpy,
	Builder,
	Medigun,
	GrenadeMirvbomb,
	FlamethrowerRocket,
	GrenadeDemoman,
	SentryBullet,
	SentryRocket,
	Dispenser,
	Invis,
	Flaregun,
	Lunchbox,
	Jar,
	CompoundBow,
	BuffItem,
	PumpkinBomb,
	Sword, 
	RocketlauncherDirecthit,
	Lifeline,
	LaserPointer,
	DispenserGun,
	SentryRevenge,
	JarMilk,
	HandgunScoutPrimary,
	BatFish,
	Crossbow,
	Stickbomb,
	HandgunScoutSecondary,
	SodaPopper,
	SniperrifleDecap,
	Raygun,
	ParticleCannon,
	MechanicalArm,
	DrgPomson,
	BatGiftwrap,
	GrenadeOrnamentBall,
	FlaregunRevenge,
	PepBrawlerBlaster,
	Cleaver,
	GrenadeCleaver,
	StickyBallLauncher,
	GrenadeStickyBall,
	ShotgunBuildingRescue,
	Cannon,
	Throwable,
	GrenadeThrowable,
	PdaSpyBuild,
	GrenadeWaterballoon,
	HarvesterSaw,
	Spellbook,
	SpellbookProjectile,
	SniperrifleClassic,
	Parachute,
	Grapplinghook,
	PasstimeGun,
	ChargedSmg
}
