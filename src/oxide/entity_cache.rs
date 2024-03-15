use std::{collections::HashMap, mem::MaybeUninit};

use crate::{
    c,
    error::OxideResult,
    i, o,
    sdk::{
        entity::{BoneMask, Bones, Entity, MAX_STUDIO_BONES},
        model_render::Matrix3x4,
        networkable::ClassId,
    },
};

#[derive(Debug, Clone)]
pub struct EntityCache {
    entities: HashMap<ClassId, Vec<isize>>,
    bones: HashMap<isize, [Matrix3x4; MAX_STUDIO_BONES]>,
}

impl EntityCache {
    pub fn init() -> OxideResult<EntityCache> {
        let entity_count = c!(i!(entity_list), get_max_entities);

        let mut entities: HashMap<ClassId, Vec<isize>> = HashMap::new();

        for id in 0..entity_count {
            let Ok(ent) = Entity::get_ent(id) else {
                continue;
            };
            let net = ent.as_networkable();
            let class = c!(net, get_client_class);
            if let Some(vec) = entities.get_mut(&class.class_id) {
                vec.push(id);
            } else {
                entities.insert(class.class_id.clone(), vec![id]);
            };
        }

        Ok(EntityCache { entities, bones: HashMap::new() })
    }
    pub fn get_bones(&mut self, id: isize) -> OxideResult<Bones> {
        if let Some(bones) = self.bones.get(&id) {
            return Ok(bones.clone())
        }
        
        let ent = Entity::get_ent(id)?;
        let renderable = ent.as_renderable();

        let bones = unsafe { MaybeUninit::zeroed().assume_init() };
        c!(
            renderable,
            setup_bones,
            &bones,
            MAX_STUDIO_BONES,
            BoneMask::Hitbox,
            o!().global_vars().curtime
        );
        self.bones.insert(id, bones.clone());
        Ok(bones.clone())
    }
    pub fn get_ent(&self, id: ClassId) -> Vec<isize> {
        self.entities.get(&id).cloned().unwrap_or(vec![])
    }
}
