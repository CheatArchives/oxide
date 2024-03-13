use crate::{
    c, draw::colors::GREEN, hex_to_rgb, i, math::{get_corners, vector::Vector2}, s, sdk::entity::Entity, util::world_to_screen
};

pub fn esp() {
    let Some(p_local) = Entity::local() else {
        return;
    };
    if !c!(i!(base_engine), is_in_game) || !*s!().visual.esp.lock().unwrap(){
        return;
    }
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
        let corners = get_corners(&origin, &angles.to_vectors(), &min, &max);
        let corners = corners.iter().filter_map(|corner| world_to_screen(corner));

        let Some(Vector2 { x: minx, y: _ }) = corners.clone().min_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
        let Some(Vector2 { x: _, y: miny }) = corners.clone().min_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

        let Some(Vector2 { x: maxx, y: _ }) = corners.clone().max_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {continue;};
        let Some(Vector2 { x: _, y: maxy }) = corners.clone().max_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {continue;};

        let (r, g, b) = hex_to_rgb!(team.color());
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

        let (r, g, b) = hex_to_rgb!(GREEN);
        c!(
            i!(mat_surface),
            set_color,
            r as isize,
            g as isize,
            b as isize,
            50 as isize
        );
        let health = c!(ent, get_health);
        let max_health = c!(ent, get_max_health);

        c!(
            i!(mat_surface),
            draw_filled_rect,
            minx as isize - 5,
            miny as isize
                + ((1.0 - (health as f32 / max_health as f32)) * (maxy as f32 - miny as f32))
                    as isize,
            minx as isize - 2,
            maxy as isize
        );
    }
}
