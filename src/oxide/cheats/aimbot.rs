use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Aimbot {}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {}
    }
    pub unsafe fn run(&self, p_local: &mut Entity) -> Result<Option<Angles>, OxideError> {
        let my_angles = call!(p_local, GetAbsAngles);
        let entity_count = call!(interface_ref!(entity_list), GetMaxEntities);
        my_angles.pitch += p_local.vecPunchAngle.roll;
        my_angles.yaw += p_local.vecPunchAngle.yaw;
        my_angles.roll += p_local.vecPunchAngle.roll;
        let my_eyes = call!(p_local, EyePosition);
        for i in 0..entity_count {
            let Some(ent) = Entity::get(i) else {
                continue;
            };
            if call!(ent, GetTeamNumber) == call!(p_local, GetTeamNumber) {
                continue;
            }

            let Some(hitbox) = ent.get_hitbox(HitboxId::HitboxHead) else {
                return Err(OxideError::new("could not get hitbox"));
            };

            let diff = my_eyes - hitbox;

            return Ok(Some(diff.ang()));
        }
        Ok(None)
    }

    pub unsafe fn create_move(&mut self, cmd: &'static mut UserCmd) -> Result<(), OxideError> {
        let p_local = get_plocal().unwrap();
        if !call!(p_local, IsAlive) {
            return Ok(());
        }
        let weapon = call!(p_local, GetWeapon);

        if let Some(new_angle) = self.run(p_local)? {
            if !p_local.m_nPlayerCond.get(ConditionFlags::Zoomed) {
                cmd.buttons.set(ButtonFlags::IN_ATTACK2, true);
                dbg!("zoom");
                return Ok(());
            }
            if !p_local.can_attack() || !call!(weapon, CanFireCriticalShot, true) {
                return Ok(());
            }
            dbg!("shooting");
            dbg!(&cmd);
            cmd.viewangles = new_angle;
            cmd.buttons.set(ButtonFlags::IN_ATTACK, true);
        }
        Ok(())
    }
}
