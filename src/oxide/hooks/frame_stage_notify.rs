use libc::isblank;

use crate::*;

pub type FrameStageNotifyFn = cfn!((), &BaseClient, FrameStage);

pub unsafe extern "C-unwind" fn frame_stage_notify_hook(client: &BaseClient, stage: FrameStage) {
    match stage {
        _ => {}
    }
    (oxide!().hooks.frame_stage_notify.org)(client, stage);
}
