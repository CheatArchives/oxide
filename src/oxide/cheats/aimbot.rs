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

        if distance_to_center > *settings!().aimbot.fov.lock().unwrap() as f32 {
            return None;
        }

        Some(-distance_to_center as isize)
    }

    pub fn ent_priority(&self, p_local: &Entity, ent: &Entity) -> Option<isize> {
        unsafe {
            if call!(ent, get_team_number) == call!(p_local, get_team_number) {
                return None;
            }
        }
        Some(1 as isize)
    }

    pub fn point_scan(
        &self,
        p_local: &'static Entity,
        ent: &Entity,
        hitboxid: HitboxId,
        hitbox: &Hitbox,
        bone: &Matrix3x4,
    ) -> Option<Vector3> {
        let my_eyes = unsafe { call!(p_local, eye_position) };

        let points = vec![hitbox.center(bone)];

        let mut corners = hitbox.corners(bone).to_vec();

        corners.sort_by(|corner1, corner2| {
            let diff1 = corner1.clone() - my_eyes.clone();
            let diff2 = corner2.clone() - my_eyes.clone();
            diff1.len().total_cmp(&diff2.len())
        });

        corners.pop();
        corners.remove(0);

        let points = vec![points, corners].concat();

        for point in points {
            let trace = trace(
                my_eyes.clone(),
                point.clone(),
                MASK_SHOT | CONTENTS_GRATE,
                p_local,
            );
            if trace.entity != ent || trace.hitbox != hitboxid {
                continue;
            }
            return Some(point);
        }
        None
    }

    pub fn find_point(&self, p_local: &'static Entity, ent: &'static Entity) -> Option<Vector3> {
        let my_eyes = unsafe { call!(p_local, eye_position) };
        for hitboxid in self.hitbox_order(p_local) {
            let (hitbox, bone) = ent.get_hitbox(hitboxid).unwrap();

            let Some(point) = self.point_scan(p_local, ent, hitboxid, &hitbox, &bone) else {
                continue;
            };

            let Some(prio) = self.point_priority(p_local, point.clone()) else {
                continue;
            };

            return Some(point);
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

            let Some(prio) = self.ent_priority(p_local, ent) else {
                continue;
            };

            let Some(point) = self.find_point(p_local, ent) else {
                continue;
            };

            let Some((_, last_prio)) = &target else {
                target = Some((point, prio));
                continue;
            };

            if prio > *last_prio {
                target = Some((point, prio))
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
        let id = unsafe { call!(weapon, get_weapon_id) };
        match id {
            WeaponType::Sniperrifle => {
                vec![HitboxId::Head]
            }
            _ => [HitboxId::body(), vec![HitboxId::Head]].concat(),
        }
    }
    pub fn should_run(&mut self) -> bool {
        if !*settings!().aimbot.enabled.lock().unwrap() || !self.shoot_key_pressed {
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
        let weapon = unsafe { call!(p_local, get_weapon) };
        let id = unsafe { call!(weapon, get_weapon_id) };
        match id {
            WeaponType::Sniperrifle => {
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
            WeaponType::Knife => unsafe {
                if weapon.ready_to_backstab {
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    return true;
                }
                return false;
            },
            _ => {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                true
            }
        }
    }
}
