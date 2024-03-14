use std::{ffi::CString, mem::MaybeUninit};

use crate::{
    c,
    draw::colors::{BLUE, FOREGROUND, GREEN},
    error::OxideResult,
    hex_to_rgb, i,
    math::{get_corners, vector::Vector2},
    oxide::tick_cache::TickCache,
    s,
    sdk::{entity::Entity, networkable::ClientClassId},
    util::world_to_screen,
};

use super::Paint;

impl Paint {
    pub fn esp(&mut self, cache: &TickCache) -> OxideResult<()> {
        if !c!(i!(base_engine), is_in_game) || !*s!().visual.esp.lock().unwrap() {
            return Ok(());
        }
        for id in cache.entities.get(&ClientClassId::CBasePlayer).unwrap().clone() {
            let player = Entity::get_player(id)?;
            let p_local = Entity::get_local()?;
            if c!(player.entity.as_networkable(), is_dormant) {
                continue;
            }
            if player as *const _ == p_local as *const _ || !c!(&player.entity, is_alive) {
                continue;
            }
            let team = c!(&player.entity, get_team_number);
            let collidable = c!(&player.entity, get_collideable);
            let min = *c!(collidable, obb_mins);
            let max = *c!(collidable, obb_maxs);
            let origin = *c!(collidable, get_origin);
            let angles = *c!(collidable, get_angles);
            let corners = get_corners(&origin, &angles.to_vectors(), &min, &max);
            let corners = corners.iter().filter_map(|corner| world_to_screen(corner));

            let Some(Vector2 { x: minx, y: _ }) = corners.clone().min_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
            let Some(Vector2 { x: _, y: miny }) = corners.clone().min_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

            let Some(Vector2 { x: maxx, y: _ }) = corners.clone().max_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
            let Some(Vector2 { x: _, y: maxy }) = corners.clone().max_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

            let (r, g, b) = hex_to_rgb!(team.color());
            c!(
                i!(surface),
                set_color,
                r as isize,
                g as isize,
                b as isize,
                50 as isize
            );
            c!(
                i!(surface),
                draw_rect,
                minx as isize,
                miny as isize,
                maxx as isize,
                maxy as isize
            );

            //hp bar
            let (r, g, b) = hex_to_rgb!(GREEN);
            c!(
                i!(surface),
                set_color,
                r as isize,
                g as isize,
                b as isize,
                50 as isize
            );
            let health = c!(&player.entity, get_health);
            let max_health = c!(&player.entity, get_max_health);

            c!(
                i!(surface),
                draw_filled_rect,
                minx as isize - 5,
                miny as isize
                    + ((1.0 - (health.min(max_health) as f32 / max_health as f32))
                        * (maxy as f32 - miny as f32)) as isize,
                minx as isize - 2,
                maxy as isize
            );
            if health > max_health {
                let (r, g, b) = hex_to_rgb!(BLUE);
                c!(
                    i!(surface),
                    set_color,
                    r as isize,
                    g as isize,
                    b as isize,
                    50 as isize
                );

                c!(
                    i!(surface),
                    draw_filled_rect,
                    minx as isize - 5,
                    miny as isize
                        + ((2.0 - (health as f32 / max_health as f32).max(1.0))
                            * (maxy as f32 - miny as f32)) as isize,
                    minx as isize - 2,
                    maxy as isize
                );
            }
            //name
            let mut info = unsafe { MaybeUninit::zeroed().assume_init() };
            c!(i!(base_engine), get_player_info, id, &mut info);
            let name = info.name.into_iter().filter(|&x| x != 0).collect::<Vec<u8>>();
            let name = CString::new(name).unwrap();
            self.paint_text(
                name.to_str()?,
                minx as isize,
                (miny - 15f32) as isize,
                FOREGROUND,
                false,
            )
        }
        Ok(())
    }
}
