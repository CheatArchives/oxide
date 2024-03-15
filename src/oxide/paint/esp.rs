use std::mem::MaybeUninit;

use crate::{
    c,
    draw::colors::FOREGROUND,
    error::OxideResult,
    i,
    math::{get_corners, vector::Vector2},
    oxide::entity_cache::EntityCache,
    s,
    sdk::{entity::Entity, networkable::ClassId},
    util::world_to_screen,
};

use super::Paint;

impl Paint {
    pub fn esp(&mut self, cache: &EntityCache) -> OxideResult<()> {
        if !c!(i!(base_engine), is_in_game) || !*s!().visual.esp.lock().unwrap() {
            return Ok(());
        }
        for id in cache.get_ent(ClassId::CTFPlayer) {
            let player = Entity::get_ent(id)?;
            let p_local = Entity::get_local()?;
            if c!(player.as_networkable(), is_dormant) {
                continue;
            }
            if player as *const _ == p_local.as_ent() as *const _ || !c!(player, is_alive) {
                continue;
            }

            player.paint();

            let collidable = c!(player, get_collideable);
            let min = *c!(collidable, obb_mins);
            let max = *c!(collidable, obb_maxs);
            let origin = *c!(collidable, get_origin);
            let angles = *c!(collidable, get_angles);
            let corners = get_corners(&origin, &angles.to_vectors(), &min, &max);
            let corners = corners.iter().filter_map(|corner| world_to_screen(corner));

            let Some(Vector2 { x: minx, y: _ }) = corners.clone().min_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
            let Some(Vector2 { x: _, y: miny }) = corners.clone().min_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

            //name
            let mut info = unsafe { MaybeUninit::zeroed().assume_init() };
            c!(i!(base_engine), get_player_info, id, &mut info);
            let name = info
                .name
                .into_iter()
                .filter_map(|x| if x != 0 { Some(x as i32) } else { None })
                .collect::<Vec<i32>>();
            self.paint_text(
                name,
                minx as isize,
                (miny - 15f32) as isize,
                FOREGROUND,
                false,
            )
        }
        for id in cache.get_ent(ClassId::CObjectSentrygun) {
            let entity = Entity::get_ent(id)?;
            entity.paint();
        }
        Ok(())
    }
}
