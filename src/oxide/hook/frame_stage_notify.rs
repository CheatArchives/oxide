use crate::{
    define_hook,
    sdk::base_client::{BaseClient, FrameStage},
};

//pub type FrameStageNotifyFn = cfn!((), &BaseClient, FrameStage);
//
//pub unsafe extern "C-unwind" fn frame_stage_notify_hook(client: &BaseClient, stage: FrameStage) {
//    match stage {
//        _ => {}
//    }
//    //(o!().hooks.frame_stage_notify.org)(client, stage);
//}

define_hook!(
    FrameStageNotifyHook,
    "FrameStageNotify",
    (),
    (),
    client,
    &BaseClient,
    stage,
    FrameStage
);
