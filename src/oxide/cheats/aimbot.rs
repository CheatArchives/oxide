use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Aimbot {}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {}
    }
    pub unsafe fn run(&self, p_local: &mut Entity) -> Result<Option<Angles>, OxideError> {
        let my_angles = call!(p_local, get_abs_angles);
        let entity_count = call!(interface_ref!(entity_list), GetMaxEntities);
        my_angles.pitch += p_local.vec_punch_angle.roll;
        my_angles.yaw += p_local.vec_punch_angle.yaw;
        my_angles.roll += p_local.vec_punch_angle.roll;
        let my_eyes = call!(p_local, eye_position);
        for i in 0..entity_count {
            let Some(ent) = Entity::get(i) else {
                continue;
            };
            if call!(ent, get_team_number) == call!(p_local, get_team_number) {
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
        if !call!(p_local, is_alive) {
            return Ok(());
        }
        let weapon = call!(p_local, get_weapon);

        if let Some(new_angle) = self.run(p_local)? {
            if !p_local.player_cond.get(ConditionFlags::Zoomed) {
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
