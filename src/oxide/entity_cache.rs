use std::{collections::HashMap, mem::transmute};

use crate::{
    c,
    error::OxideResult,
    i,
    sdk::{entity::Entity, networkable::ClientClassId},
};

#[derive(Debug, Clone)]
pub struct EntityCache {
    pub entities: HashMap<ClientClassId, Vec<isize>>,
}

impl EntityCache {
    pub fn init() -> OxideResult<EntityCache> {
        let entity_count = c!(i!(entity_list), get_max_entities);

        let mut entities: HashMap<ClientClassId, Vec<isize>> = HashMap::new();
        for id in 0..entity_count {
            let Ok(ent) = Entity::get_ent(id) else {
                continue;
            };
            let net = ent.as_networkable();
            let class = c!(net, get_client_class);
            if let Some(vec) = entities.get_mut(&class.class_id) {
                vec.push(id);
            }else {
                entities.insert(class.class_id.clone(), vec![id]);
            };
        }

        Ok(EntityCache { entities })
    }
}
