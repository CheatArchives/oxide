
use crate::{
    define_hook,
    oxide::entity_cache::EntityCache,
    sdk::base_client::{BaseClient, FrameStage},
};

fn subhooks(hook: &mut FrameStageNotifyHook) {
    hook.before = Some(|_, stage| {
        match stage {
            FrameStage::FrameNetUpdateEnd => {
                match EntityCache::init() {
                    Ok(cache) => {
                        o!().last_entity_cache = Some(cache);
                    }
                    Err(e) => {
                        o!().last_entity_cache = None;
                        return Err(e);
                    }
                };
            }
            _ => {}
        }
        Ok(true)
    });
    hook.after = Some(|_, _, _| Ok(()));
}

define_hook!(
    FrameStageNotifyHook,
    "FrameStageNotify",
    (),
    (),
    subhooks,
    client,
    &BaseClient,
    stage,
    FrameStage
);
