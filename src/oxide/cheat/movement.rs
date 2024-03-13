use std::{any::Any, f32::consts::PI, ffi::CString};

use crate::{
    c, i,
    math::{angles::Angles, dtr},
    s,
    sdk::{
        condition::ConditionFlags,
        entity::{flags::Flag, Entity},
        player_class::PlayerClass,
        user_cmd::{ButtonFlags, UserCmd},
        weapon::WeaponType,
    },
};

use super::Cheat;

#[derive(Debug)]
pub struct Movement {}
impl Movement {
    pub fn init() -> Movement {
        Movement {}
    }
    pub fn name() -> &'static str {
        "Movement"
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd, org_cmd: &UserCmd) {
        let p_local = Entity::local().unwrap();

        self.bhop(cmd, p_local);
        self.auto_strafe(cmd, p_local);

        if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
            let (corrected_forward, correct_side) = Self::correct_movement(
                org_cmd.viewangles,
                &cmd.viewangles,
                cmd.forwardmove,
                cmd.sidemove,
            );
            cmd.forwardmove = corrected_forward;
            cmd.sidemove = correct_side;
        }
    }
    pub fn bhop(&mut self, cmd: &mut UserCmd, p_local: &Entity) {
        if cmd.buttons.get(ButtonFlags::InJump) && *s!().movement.bhop.lock().unwrap() {
            cmd.buttons
                .set(ButtonFlags::InJump, p_local.flags.get(Flag::ONGROUND));

            if *s!().movement.revhop.lock().unwrap()
                && !p_local.player_cond.get(ConditionFlags::Aiming)
                && matches!(
                    c!(c!(p_local, get_weapon), get_weapon_id),
                    WeaponType::Minigun
                )
            {
                cmd.buttons
                    .set(ButtonFlags::InAttack2, p_local.flags.get(Flag::ONGROUND));
            }
        }
    }
    pub fn auto_strafe(&self, cmd: &mut UserCmd, p_local: &Entity) {
        if p_local.flags.get(Flag::ONGROUND) || !*s!().movement.autostrafe.lock().unwrap() {
            return;
        }
        dbg!("test");
        let velocity = p_local.velocity;
        let speed = velocity.len2d();

        let speed_var = 6062.0; // Engine limit on 3 axis, see: reddit.com/r/tf2/comments/57hhl4/question_is_there_a_maximum_rocket_jumping/d8t9x82
        let air_var = c!(
            i!(cvar),
            find_var,
            &CString::new("sv_airaccelerate").unwrap()
        )
        .float_value;
        let wish_speed = 30.0; // This is hardcoded for tf2, unless you run sourcemod

        let term = wish_speed / air_var / speed_var * 100.0 / speed;

        let perfect_delta = if term < 1.0 && term > -1.0 {
            term.acos()
        } else {
            0.0
        };

        let yaw = dtr(cmd.viewangles.yaw);
        let angle = velocity.y.atan2(velocity.y) - yaw;
        let desired_angle = cmd.sidemove.atan2(cmd.forwardmove);
        let delta = angle - desired_angle;

        let direction = if delta < 0.0 {
            angle + perfect_delta
        } else {
            angle - perfect_delta
        };

        cmd.forwardmove = direction.cos() * 450.0;
        cmd.forwardmove = -direction.sin() * 450.0;
    }
    pub fn correct_movement(
        org_view_angles: Angles,
        new_view_angles: &Angles,
        old_forward: f32,
        old_side: f32,
    ) -> (f32, f32) {
        let alpha = (new_view_angles.yaw - org_view_angles.yaw) * PI / 180f32;

        let forward = old_forward * alpha.cos() - old_side * alpha.sin();
        let side = old_side * alpha.cos() + old_forward * alpha.sin();

        (forward, side)
    }
}
impl Cheat for Movement {
    fn handle_event(&mut self, _: &mut crate::draw::event::Event) {}
}
