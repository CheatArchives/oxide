use crate::{
    c, draw::colors::GREEN, hex_to_rgb, i, math::{get_corners, vector::Vector2}, util::world_to_screen };

use super::Entity;

impl Entity {
    pub fn paint(&self) {
        let team = c!(self, get_team_number);
        let collidable = c!(self, get_collideable);
        let min = *c!(collidable, obb_mins);
        let max = *c!(collidable, obb_maxs);
        let origin = *c!(collidable, get_origin);
        let angles = *c!(collidable, get_angles);
        let corners = get_corners(&origin, &angles.to_vectors(), &min, &max);
        let corners = corners.iter().filter_map(|corner| world_to_screen(corner));

        let Some(Vector2 { x: minx, y: _ }) = corners.clone().min_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {return};
        let Some(Vector2 { x: _, y: miny }) = corners.clone().min_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {return;};

        let Some(Vector2 { x: maxx, y: _ }) = corners.clone().max_by(|c1,c2|c1.x.total_cmp(&c2.x)) else {return;};
        let Some(Vector2 { x: _, y: maxy }) = corners.clone().max_by(|c1,c2|c1.y.total_cmp(&c2.y)) else {return;};

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
        let health = c!(self, get_health);
        let max_health = c!(self, get_max_health);
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
    }
}
