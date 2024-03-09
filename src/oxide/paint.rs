use crate::*;

const COLOR_SCALE: f32 = 1.0 / 2.0;

pub fn draw_hitboxes() {
    let Some(p_local) = Entity::local() else {
        return;
    };
    if unsafe { c!(i!(base_engine), is_in_game) } {
        let entity_count = unsafe { c!(i!(entity_list), get_highest_entity_index) };
        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                    continue;
                };
            if ent as *const _ == p_local as *const _ {
                continue;
            }
            let team = unsafe { c!(ent, get_team_number) };

            let scale = oxide!().cheats.aimbot.hitbox_scale;

            let mut hitbox = ent.get_hitbox(HitboxId::Head).unwrap();
            hitbox.min *= scale;
            hitbox.max *= scale;
            draw_hitbox(ent, hitbox, team.color(), 5);
            for hitbox_id in HitboxId::body() {
                let (mut r, mut g, mut b) = hex_to_rgb!(team.color());
                let color = rgb_to_hex!(
                    r as f32 * COLOR_SCALE,
                    g as f32 * COLOR_SCALE,
                    b as f32 * COLOR_SCALE
                );
                let mut hitbox = ent.get_hitbox(hitbox_id).unwrap();
                hitbox.min *= scale;
                hitbox.max *= scale;
                draw_hitbox(ent, hitbox, color, 5);
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
        unsafe {
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
}
