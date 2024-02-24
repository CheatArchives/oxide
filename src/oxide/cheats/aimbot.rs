use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Aimbot {}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {}
    }
    pub unsafe fn get_target(&self, p_local: &Entity) -> Result<Option<Angles>, OxideError> {
        let mut my_angles = *call!(*p_local, get_abs_angles);
        let entity_count = call!(interface!(entity_list), get_max_entities);

        my_angles.pitch += p_local.vec_punch_angle.pitch;
        my_angles.yaw += p_local.vec_punch_angle.yaw;
        my_angles.roll += p_local.vec_punch_angle.roll;

        let my_eyes = call!(*p_local, eye_position);
        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                continue;
            };
            if call!(*ent, get_team_number) == call!(*p_local, get_team_number) {
                continue;
            }

            let Some(hitbox) = ent.get_hitbox(HitboxId::HitboxHead) else {
                return Err(OxideError::new("could not get hitbox"));
            };

            let diff = my_eyes - hitbox;

            return Ok(Some(diff.angle()));
        }
        Ok(None)
    }

    pub unsafe fn create_move(&mut self, cmd: &mut UserCmd) -> Result<(), OxideError> {
        if !menu!().aimbot_checkbox.checked {
            return Ok(());
        }

        let Some(p_local) = Entity::local() else {
            return Ok(());
        };
        if !call!(*p_local, is_alive) {
            return Ok(());
        }
        let weapon = *call!(*p_local, get_weapon);

        if let Some(new_angle) = self.get_target(p_local)? {
            if !p_local.player_cond.get(ConditionFlags::Zoomed) {
                cmd.buttons.set(ButtonFlags::InAttack2, true);
                return Ok(());
            }
            if !p_local.can_attack() || !call!(weapon, can_fire_critical_shot, true) {
                return Ok(());
            }
            cmd.viewangles = new_angle;
            cmd.buttons.set(ButtonFlags::InAttack, true);
        }
        Ok(())
    }
}
