use crate::{
    c,
    error::OxideResult,
    hex_to_rgb, i, o,
    oxide::entity_cache::EntityCache,
    rgb_to_hex, s,
    sdk::{
        entity::Entity,
        model_info::{Hitbox, HitboxId},
        networkable::ClassId,
    },
    util::world_to_screen,
};

use crate::oxide::cheat::aimbot::HITBOX_SCALE;

use super::Paint;

const COLOR_SCALE: f32 = 1.0 / 2.0;

impl Paint {
    pub fn draw_hitboxes(&mut self, cache: &EntityCache) -> OxideResult<()> {
        if !c!(i!(base_engine), is_in_game) || !*s!().visual.hitboxes.lock().unwrap() {
            return Ok(());
        }
        for id in cache.get(ClassId::CTFPlayer) {
            let p_local = Entity::get_local()?;
            let player = Entity::get_ent(id)?;
            if c!(player.as_networkable(), is_dormant) {
                continue;
            }
            if player as *const _ == &p_local.as_ent() as *const _ || !c!(player, is_alive) {
                continue;
            }
            let team = c!(player, get_team_number);

            let hitbox = player
                .get_hitbox(HitboxId::Head)
                .unwrap()
                .scaled(HITBOX_SCALE);
            self.draw_hitbox(&player, hitbox, team.color(), 10);
            for hitbox_id in HitboxId::body() {
                let (r, g, b) = hex_to_rgb!(team.color());
                let color = rgb_to_hex!(
                    r as f32 * COLOR_SCALE,
                    g as f32 * COLOR_SCALE,
                    b as f32 * COLOR_SCALE
                );
                let hitbox = player
                    .get_hitbox(hitbox_id)
                    .unwrap()
                    .scaled(HITBOX_SCALE);
                self.draw_hitbox(&player, hitbox, color, 10);
            }
        }
        Ok(())
    }
    pub fn draw_hitbox(&mut self, ent: &Entity, hitbox: Hitbox, color: usize, alpha: u8) {
        let corners = hitbox.corners(ent);

        let pairs = [
            (corners[0].clone(), corners[1].clone()),
            (corners[0].clone(), corners[2].clone()),
            (corners[0].clone(), corners[4].clone()),
            (corners[7].clone(), corners[3].clone()),
            (corners[7].clone(), corners[5].clone()),
            (corners[7].clone(), corners[6].clone()),
            (corners[2].clone(), corners[3].clone()),
            (corners[2].clone(), corners[6].clone()),
            (corners[1].clone(), corners[5].clone()),
            (corners[1].clone(), corners[3].clone()),
            (corners[4].clone(), corners[6].clone()),
            (corners[4].clone(), corners[5].clone()),
        ];

        for pair in pairs {
            let Some(pos1) = world_to_screen(&pair.0) else {
                        continue;
                    };
            let Some(pos2) = world_to_screen(&pair.1) else {
                        continue;
                    };
            let (r, g, b) = hex_to_rgb!(color);
            c!(
                i!(surface),
                set_color,
                r as isize,
                g as isize,
                b as isize,
                alpha as isize
            );

            c!(
                i!(surface),
                draw_line,
                pos1.x as isize,
                pos1.y as isize,
                pos2.x as isize,
                pos2.y as isize
            );
        }
    }
}
