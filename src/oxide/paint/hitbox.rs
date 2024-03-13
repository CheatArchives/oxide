use crate::{
    c, hex_to_rgb, i, o, rgb_to_hex,
    sdk::{
        entity::Entity,
        model_info::{Hitbox, HitboxId},
    },
    util::world_to_screen,
};

use crate::oxide::cheat::aimbot::HITBOX_SCALE;

const COLOR_SCALE: f32 = 1.0 / 2.0;

pub fn draw_hitboxes() {
    let Some(p_local) = Entity::local() else {
        return;
    };
    if c!(i!(base_engine), is_in_game) {
        let entity_count = c!(i!(entity_list), get_highest_entity_index);
        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                    continue;
                };
            if ent as *const _ == p_local as *const _ || !c!(ent, is_alive) {
                continue;
            }
            let team = c!(ent, get_team_number);

            let hitbox = ent.get_hitbox(HitboxId::Head).unwrap().scaled(HITBOX_SCALE);
            draw_hitbox(ent, hitbox, team.color(), 10);
            for hitbox_id in HitboxId::body() {
                let (r, g, b) = hex_to_rgb!(team.color());
                let color = rgb_to_hex!(
                    r as f32 * COLOR_SCALE,
                    g as f32 * COLOR_SCALE,
                    b as f32 * COLOR_SCALE
                );
                let hitbox = ent.get_hitbox(hitbox_id).unwrap().scaled(HITBOX_SCALE);
                draw_hitbox(ent, hitbox, color, 10);
            }
        }
    }
}
pub fn draw_hitbox(ent: &Entity, hitbox: Hitbox, color: usize, alpha: u8) {
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
            i!(mat_surface),
            set_color,
            r as isize,
            g as isize,
            b as isize,
            alpha as isize
        );

        c!(
            i!(mat_surface),
            draw_line,
            pos1.x as isize,
            pos1.y as isize,
            pos2.x as isize,
            pos2.y as isize
        );
    }
}
