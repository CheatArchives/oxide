use libc::isblank;

use crate::*;

pub type FrameStageNotifyFn = cfn!((), &BaseClient, FrameStage);

pub unsafe extern "C-unwind" fn frame_stage_notify_hook(client: &BaseClient, stage: FrameStage) {
    match stage {
        FrameStage::FrameRenderStart => {
            if let Some(p_local) = Entity::local() {
                if p_local.force_taunt_cam != menu!().third_person_checkbox.checked {
                    p_local.force_taunt_cam = menu!().third_person_checkbox.checked ;
                }
                let net = p_local.as_networkable();
            }
        }
        _ => {}
    }
    (oxide!().hooks.frame_stage_notify.org)(client, stage);
}
