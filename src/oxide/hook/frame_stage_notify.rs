use crate::{
    define_hook,
    oxide::tick_cache::TickCache,
    sdk::base_client::{BaseClient, FrameStage},
};

fn subhooks(hook: &mut FrameStageNotifyHook) {
    hook.before = Some(|_, stage| {
        match stage {
            FrameStage::FrameNetUpdateEnd => {
                match TickCache::init() {
                    Ok(cache) => {
                        o!().last_tick_cache = Some(cache.clone());
                    }
                    Err(e) => {
                        o!().last_tick_cache = None;
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
