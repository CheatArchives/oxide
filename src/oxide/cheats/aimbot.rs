use std::isize;

use sdl2_sys::SDL_Event;

use crate::*;

#[derive(Debug, Clone)]
pub struct Aimbot {
    pub shoot_key_pressed: bool,
}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {
            shoot_key_pressed: false,
        }
    }
    pub fn point_priority(&self, p_local: &Entity, target_point: Vector3) -> Option<isize> {
        let my_eyes = unsafe { call!(p_local, eye_position) };

        let diff = my_eyes - target_point;
        let angle = diff.angle();
        let my_angle = p_local.angle.clone();

        let distance_to_center = (((angle.yaw - my_angle.yaw)
            .min(360f32 - angle.yaw + my_angle.yaw)
            .abs()
            % 360f32)
            .powi(2)
            + (angle.pitch - my_angle.pitch).abs().powi(2))
        .sqrt();

        if distance_to_center > settings!().aimbot_fov as f32 {
            return None;
        }

        return Some(-distance_to_center as isize);
    }
    pub fn ent_priority(&self, p_local: &Entity, ent: &Entity) -> Option<isize> {
        unsafe {
            if call!(ent, get_team_number) == call!(p_local, get_team_number) {
                return None;
            }
        }
        return Some(1 as isize);
    }
    pub fn find_point(&self, p_local: &'static Entity, ent: &'static Entity) -> Option<Vector3> {
        let my_eyes = unsafe { call!(p_local, eye_position) };
        for hitboxid in self.hitbox_order(p_local) {
            let (hitbox, bone) = ent.get_hitbox(hitboxid).unwrap();
            let target_point_candidate = hitbox.center(&bone);

            let Some(prio) = self.point_priority(p_local, target_point_candidate.clone()) else { continue };

            let trace = trace(
                my_eyes.clone(),
                target_point_candidate.clone(),
                MASK_SHOT | CONTENTS_GRATE,
                p_local,
            );
            if trace.entity != ent && trace.hitbox == hitboxid {
                continue;
            }
            return Some(target_point_candidate);
        }
        None
    }

    pub fn find_target(&self, p_local: &'static Entity) -> Result<Option<Angles>, OxideError> {
        let entity_count = unsafe { call!(interface!(entity_list), get_max_entities) };

        let mut target: Option<(Vector3, isize)> = None;
        let my_eyes = unsafe { call!(p_local, eye_position) };

        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                continue;
            };

            let Some(prio) = self.ent_priority(p_local, ent) else { continue };

            let Some(target_point) = self.find_point(p_local, ent) else { continue };

            let Some((_, target_prio)) = &target else {
                    target = Some((target_point, prio));
                    continue;
                };

            if prio > *target_prio {
                target = Some((target_point, prio))
            }
        }

        let Some((target_point, prio)) = target else {
            return Ok(None);
        };
        let diff = my_eyes - target_point;

        return Ok(Some(diff.angle()));
        Ok(None)
    }
    pub fn hitbox_order(&self, p_local: &Entity) -> Vec<HitboxId> {
        let weapon = unsafe { call!(p_local, get_weapon) };
        match unsafe { call!(weapon, get_weapon_id) } {
            WeaponType::Sniperrifle => {
                vec![HitboxId::Head]
            }
            _ => [
                HitboxId::body(),
                vec![HitboxId::Head],
            ]
            .concat(),
        }
    }
    pub fn should_run(&mut self) -> bool {
        if !settings!().aimbot || !self.shoot_key_pressed {
            return false;
        }

        let Some(p_local) = Entity::local() else {
            return false;
        };

        if !unsafe { call!(p_local, is_alive) } {
            return false;
        }

        true
    }

    pub unsafe fn pre_create_move(&mut self, cmd: &mut UserCmd) -> Result<(), OxideError> {
        if !self.should_run() {
            return Ok(());
        }

        let p_local = Entity::local().unwrap();

        if let Some(new_angle) = self.find_target(p_local)? {
            if self.shoot(p_local, cmd) {
                cmd.viewangles = new_angle;
            }
        }

        Ok(())
    }
    pub fn shoot(&mut self, p_local: &Entity, cmd: &mut UserCmd) -> bool {
        match p_local.player_class {
            PlayerClass::Sniper => {
                let weapon = unsafe { call!(p_local, get_weapon) };
                if !p_local.player_cond.get(ConditionFlags::Zoomed) {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                unsafe {
                    if !p_local.can_attack() || !call!(weapon, can_fire_critical_shot, true) {
                        return false;
                    }
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    true
                }
            }
            PlayerClass::Hwguy => {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                true
            }
            _ => unsafe {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                return true;
            },
        }
    }
}
