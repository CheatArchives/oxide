use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Aimbot {}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {}
    }
    pub fn ent_priority(&self, p_local: &Entity, ent: &Entity) -> Option<u8> {
        unsafe {
            if call!(*ent, get_team_number) == call!(*p_local, get_team_number) {
                return None;
            }
            return Some(1);
        }
    }

    pub fn remove_punch(p_local: &Entity) {
        let mut my_angles = unsafe { *call!(*p_local, get_abs_angles) };
        my_angles.pitch += p_local.vec_punch_angle.pitch;
        my_angles.yaw += p_local.vec_punch_angle.yaw;
        my_angles.roll += p_local.vec_punch_angle.roll;
    }

    pub unsafe fn find_target(&self, p_local: &Entity) -> Result<Option<Angles>, OxideError> {
        let entity_count = call!(interface!(entity_list), get_max_entities);

        let mut target: Option<(&mut Entity, u8)> = None;

        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                continue;
            };

            let Some(prio) = self.ent_priority(p_local, ent) else {
                continue;
            };

            let Some((_, target_prio)) = &target else {
                target = Some((ent, prio));
                continue;
            };
            if prio > *target_prio {
                target = Some((ent, prio))
            }
        }

        let Some((ent, prio)) = target else {
            return Ok(None);
        };

        let Some((hitbox, bone)) = ent.get_hitbox(HitboxId::HitboxHead) else {
            return Err(OxideError::new("could not get hitbox").into());
        };

        let my_eyes = call!(*p_local, eye_position);

        let diff = my_eyes - hitbox.center(bone);

        return Ok(Some(diff.angle()));
        Ok(None)
    }
    pub fn should_run(&mut self) -> bool {
        if !menu!().aimbot_checkbox.checked {
            return false;
        }

        let Some(p_local) = Entity::local() else {
            return false;
        };

        if !unsafe { call!(*p_local, is_alive) } {
            return false;
        }
        return true;
    }

    pub unsafe fn pre_create_move(&mut self, cmd: &mut UserCmd) -> Result<(), OxideError> {
        if !self.should_run() {
            return Ok(());
        }

        let p_local = Entity::local().unwrap();

        let start = std::time::SystemTime::now();
        if let Some(new_angle) = self.find_target(p_local)? {
            let shooting = match p_local.player_class {
                PlayerClass::Sniper => self.sniper_shoot(p_local, cmd),
                PlayerClass::Hwguy => self.heavy_shoot(p_local, cmd),
                _ => true,
            };

            if shooting {
                cmd.viewangles = new_angle;
            }
        }
        let end = std::time::SystemTime::now();

        Ok(())
    }
    pub fn sniper_shoot(&mut self, p_local: &Entity, cmd: &mut UserCmd) -> bool {
        let weapon = unsafe { *call!(*p_local, get_weapon) };
        if !p_local.player_cond.get(ConditionFlags::Zoomed) {
            cmd.buttons.set(ButtonFlags::InAttack2, true);
            return false;
        }
        unsafe {
            if !p_local.can_attack() || !call!(weapon, can_fire_critical_shot, true) {
                return false;
            }
            cmd.buttons.set(ButtonFlags::InAttack, true);
            return true;
        }
    }

    pub fn heavy_shoot(&mut self, p_local: &Entity, cmd: &mut UserCmd) -> bool {
        cmd.buttons.set(ButtonFlags::InAttack, true);
        return true;
    }
}
