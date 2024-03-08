use crate::*;

pub type PaintFn = cfn!((), &EngineVgui, isize);

pub unsafe extern "C-unwind" fn paint_hook(engine_vgui: &EngineVgui, mode: isize) {
    (oxide!().hooks.paint.org)(engine_vgui, mode);
    let Some(p_local) = Entity::local() else {
        return;
    };
    if unsafe { call!(interface!(base_engine), is_in_game) } {
        let entity_count = unsafe { call!(interface!(entity_list), get_highest_entity_index) };
        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                    continue;
                };
            if ent as *const _ == p_local as *const _ {
                continue;
            }
            let team = call!(ent, get_team_number);
            let hitbox = ent.get_hitbox(HitboxId::Head).unwrap();
            let (pos, angle) = ent.get_hitbox_raw(hitbox.0.bone);
            let rotation = angle.to_vectors();

            let corners = [
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
                Vector3::zeroed(),
            ];
            let min = hitbox.0.min;
            let max = hitbox.0.max;
            let mut corners = Vec::new();
            for i in 0..8 {
                let x = if i & 0x1 != 0 { max.x } else { min.x };
                let y = if i & 0x2 != 0 { max.y } else { min.y };
                let z = if i & 0x4 != 0 { max.z } else { min.z };

                let mut corner = Vector3::new(x, y, z);

                let mut corner = corner.rotate(&rotation);
                corner += pos.clone();
                corners.push(corner)
            }

            for corner in corners {
                let Some(pos) = world_to_screen(&corner) else {
                        continue;
                    };
                let (r, g, b) = hex_to_rgb!(team.color());
                call!(interface!(mat_surface), set_color, r, g, b, 255);

                call!(
                    interface!(mat_surface),
                    draw_rect,
                    pos.x as isize,
                    pos.y as isize,
                    pos.x as isize + 2,
                    pos.y as isize + 2
                );
            }
        }
    }
}
