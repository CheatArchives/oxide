use crate::{
    c, hex_to_rgb, i,
    math::{get_corners, vector::Vector2},
    sdk::entity::Entity,
    util::world_to_screen,
};

pub fn esp() {
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
            let collidable = c!(ent, get_collideable);
            let min = *c!(collidable, obb_mins);
            let max = *c!(collidable, obb_maxs);
            let origin = *c!(collidable, get_origin);
            let angles = *c!(collidable, get_angles);
            let (r, g, b) = hex_to_rgb!(team.color());
            let corners = get_corners(&origin, &angles, &min, &max);
            let corners = corners.iter().filter_map(|corner| world_to_screen(corner));

            let Some(Vector2 { x: minx, y: _ }) = corners.clone().min_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
            let Some(Vector2 { x: _, y: miny }) = corners.clone().min_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

            let Some(Vector2 { x: maxx, y: _ }) = corners.clone().max_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
            let Some(Vector2 { x: _, y: maxy }) = corners.clone().max_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

            c!(
                i!(mat_surface),
                set_color,
                r as isize,
                g as isize,
                b as isize,
                50 as isize
            );

            c!(
                i!(mat_surface),
                draw_rect,
                minx as isize,
                miny as isize,
                maxx as isize,
                maxy as isize
            );
        }
    }
}
