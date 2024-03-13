use crate::{
    define_hook,
    sdk::base_client::{BaseClient, FrameStage},
};

fn subhooks(hook: &mut FrameStageNotifyHook) {
    hook.before = Some(|_, _| {});
    hook.after = Some(|_, _, _| {});
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
