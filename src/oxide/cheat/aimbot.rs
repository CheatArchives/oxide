use crate::{
    c,
    draw::event::EventType,
    error::OxideResult,
    math::{angles::Angles, vector::Vector3},
    o, s,
    sdk::{
        condition::ConditionFlags,
        engine_trace::{trace, MASK_SHOT},
        entity::{Entity, Player},
        model_info::{Hitbox, HitboxId},
        networkable::ClientClassId,
        user_cmd::{ButtonFlags, UserCmd},
        weapon::WeaponType,
    },
};

use super::Cheat;

pub const HITBOX_SCALE: f32 = 9.0 / 10.0;

#[derive(Debug, Clone)]
pub struct Aimbot {
    pub shoot_key_pressed: bool,
}

impl Aimbot {
    pub fn name() -> &'static str {
        "Aimbot"
    }
    pub fn init() -> Aimbot {
        Aimbot {
            shoot_key_pressed: false,
        }
    }

    pub fn point_priority(&self, target_point: Vector3) -> Option<isize> {
        let p_local = &*Entity::get_local().unwrap();
        let my_eyes = c!(p_local, eye_position);

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

        if distance_to_center > *s!().aimbot.fov.lock().unwrap() as f32 {
            return None;
        }

        Some(-distance_to_center as isize)
    }

    pub fn ent_priority(&self, player: &Player) -> Option<isize> {
        let p_local = &*Entity::get_local().unwrap();
        if c!(&player.entity, get_team_number) == c!(&p_local.entity, get_team_number) {
            return None;
        }
        Some(1 as isize)
    }

    pub fn point_scan(
        &self,
        ent: &Entity,
        hitboxid: HitboxId,
        hitbox: &Hitbox,
    ) -> Option<(Vector3, isize)> {
        let p_local = &*Entity::get_local().unwrap();
        let my_eyes = c!(&p_local.entity, eye_position);

        let scaled_hitbox = hitbox.scaled(HITBOX_SCALE);

        let mut points = vec![scaled_hitbox.center(ent)];
        if *s!().aimbot.multipoint.lock().unwrap() {
            let mut corners = scaled_hitbox.corners(ent).to_vec();

            corners.sort_by(|corner1, corner2| {
                let diff1 = corner1.clone() - my_eyes.clone();
                let diff2 = corner2.clone() - my_eyes.clone();
                diff1.len().total_cmp(&diff2.len())
            });

            corners.pop();
            corners.remove(0);

            points = vec![points, corners].concat();
        }

        for point in points {
            let trace = trace(my_eyes.clone(), point.clone(), MASK_SHOT);
            if trace.entity != ent || trace.hitbox != hitboxid {
                continue;
            }
            let Some(prio) = self.point_priority(point.clone()) else {
                continue;
            };
            return Some((point, prio));
        }
        None
    }

    pub fn find_point(&self, player: &Player) -> Option<(Vector3, isize)> {
        for hitboxid in self.hitbox_order() {
            let hitbox = player.entity.get_hitbox(hitboxid).unwrap();

            let Some((point,prio)) = self.point_scan(player, hitboxid, &hitbox) else {
                continue;
            };

            return Some((point, prio));
        }
        None
    }

    pub fn find_target(&self) -> OxideResult<Option<Angles>> {
        let p_local = &*Entity::get_local().unwrap();
        let mut target: Option<(Vector3, (isize, isize))> = None;
        let my_eyes = c!(&p_local.entity, eye_position);

        for id in o!()
            .last_tick_cache
            .clone()
            .unwrap()
            .entities
            .get(&ClientClassId::CBasePlayer)
            .unwrap()
            .clone()
        {
            let player = Entity::get_player(id)?;
            if c!(player.entity.as_networkable(), is_dormant) {
                continue;
            }
            let Some(prio) = self.ent_priority(&player) else {
                continue;
            };

            let Some((point,point_prio)) = self.find_point(&player) else {
                continue;
            };

            let Some((_, (last_prio, last_point_prio))) = target.clone() else {
                target = Some((point, (prio,point_prio)));
                continue;
            };

            if prio > last_prio {
                target = Some((point, (prio, point_prio)))
            } else if prio == last_prio && last_point_prio < point_prio {
                target = Some((point, (prio, point_prio)))
            }
        }

        let Some((target_point, _)) = target else {
            return Ok(None);
        };
        let diff = my_eyes - target_point;

        Ok(Some(diff.angle()))
    }
    pub fn hitbox_order(&self) -> Vec<HitboxId> {
        let p_local = &*Entity::get_local().unwrap();
        let weapon = c!(&p_local.entity, get_weapon);
        let id = c!(weapon, get_weapon_id);
        match id {
            WeaponType::Sniperrifle => {
                vec![HitboxId::Head]
            }
            _ => [HitboxId::body(), vec![HitboxId::Head]].concat(),
        }
    }
    pub fn should_run(&self) -> bool {
        let p_local = &*Entity::get_local().unwrap();
        if !*s!().aimbot.enabled.lock().unwrap() || !self.shoot_key_pressed {
            return false;
        }

        if !c!(&p_local.entity, is_alive) {
            return false;
        }

        true
    }

    pub fn create_move(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        if !self.should_run() {
            return Ok(());
        }

        if let Some(new_angle) = self.find_target()? {
            if *s!().aimbot.autoshoot.lock().unwrap() {
                if self.shoot(cmd) {
                    cmd.viewangles = new_angle;
                }
            } else {
                cmd.viewangles = new_angle;
            }
        }
        Ok(())
    }
    pub fn shoot(&mut self, cmd: &mut UserCmd) -> bool {
        let p_local = &*Entity::get_local().unwrap();
        let weapon = c!(&p_local.entity, get_weapon);
        let id = c!(weapon, get_weapon_id);
        match id {
            WeaponType::Sniperrifle => {
                if !p_local.player_cond.get(ConditionFlags::Zoomed) {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                unsafe {
                    if !p_local.can_attack() || !c!(weapon, can_fire_critical_shot, true) {
                        return false;
                    }
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    true
                }
            }
            WeaponType::Knife => {
                if weapon.ready_to_backstab {
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    return true;
                }
                false
            }
            _ => {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                true
            }
        }
    }
}

impl Cheat for Aimbot {
    fn handle_event(&mut self, event: &mut crate::draw::event::Event) {
        let aimbot_key = *s!().aimbot.key.lock().unwrap();
        match event.r#type {
            EventType::KeyDown(key) => {
                if key == aimbot_key {
                    self.shoot_key_pressed = true
                }
                event.handled = true
            }
            EventType::KeyUp(key) => {
                if key == aimbot_key {
                    self.shoot_key_pressed = false
                }
                event.handled = true
            }
            _ => (),
        }
    }
}
