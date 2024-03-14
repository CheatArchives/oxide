use std::f32::consts::PI;

use crate::{
    c,
    error::OxideResult,
    math::{angles::Angles, dtr},
    s,
    sdk::{
        condition::ConditionFlags,
        entity::{flags::Flag, Entity},
        user_cmd::{ButtonFlags, UserCmd},
        weapon::WeaponType,
    },
};

use super::Cheat;

const SPEED_VAR: f32 = 6062.0;
const WISH_SPEED: f32 = 30.0;

#[derive(Debug)]
pub struct Movement {}
impl Movement {
    pub fn init() -> Movement {
        Movement {}
    }
    pub fn name() -> &'static str {
        "Movement"
    }
    pub fn create_move(&mut self, cmd: &mut UserCmd, org_cmd: &UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        if p_local.entity.flags.get(Flag::INWATER) {
            return Ok(());
        }
        self.bhop(cmd)?;
        self.auto_strafe(cmd)?;

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

        Ok(())
    }
    pub fn bhop(&mut self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        if !*s!().movement.bhop.lock().unwrap() {
            return Ok(());
        }
        if (p_local.entity.velocity.len2d() < 200.0 && *s!().movement.revhop.lock().unwrap())
            || !cmd.buttons.get(ButtonFlags::InJump)
        {
            cmd.buttons.set(ButtonFlags::InJump, false);
            return Ok(());
        }

        cmd.buttons
            .set(ButtonFlags::InJump, p_local.entity.flags.get(Flag::ONGROUND));

        if *s!().movement.revhop.lock().unwrap()
            && !p_local.player_cond.get(ConditionFlags::Aiming)
            && matches!(
                c!(c!(&p_local.entity, get_weapon), get_weapon_id),
                WeaponType::Minigun
            )
        {
            cmd.buttons
                .set(ButtonFlags::InAttack2, p_local.entity.flags.get(Flag::ONGROUND));
        }
        Ok(())
    }
    pub fn auto_strafe(&self, cmd: &mut UserCmd) -> OxideResult<()> {
        let p_local = Entity::get_local()?;
        if p_local.entity.flags.get(Flag::ONGROUND) || !*s!().movement.autostrafe.lock().unwrap() {
            return Ok(());
        }
        let velocity = p_local.entity.velocity;
        let speed = velocity.len2d();

        //let var_name = CString::new("sv_airaccelerate").unwrap();
        //let air_var = c!(
        //    i!(cvar),
        //    find_var,
        //    var_name.as_ptr()
        //).float_value;

        let air_var = 10.0;

        let term = WISH_SPEED / air_var / SPEED_VAR * 100.0 / speed;

        let perfect_delta = if -1.0 < term && term < 1.0 {
            term.acos()
        } else {
            0.0
        };

        let yaw = dtr(cmd.viewangles.yaw);
        let angle = velocity.y.atan2(velocity.x) - yaw;
        let desired_angle = (-cmd.sidemove).atan2(cmd.forwardmove);
        let mut delta = angle - desired_angle;
        while delta > PI {
            delta -= 2.0 * PI
        }
        while delta < -PI {
            delta += 2.0 * PI
        }

        let direction = if delta < 0.0 {
            angle + perfect_delta
        } else {
            angle - perfect_delta
        };

        cmd.forwardmove = direction.cos() * 450.0;
        cmd.sidemove = -direction.sin() * 450.0;
        Ok(())
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
