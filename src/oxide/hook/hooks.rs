use std::{collections::HashMap, mem::{transmute, ManuallyDrop}};

use libc::c_void;

use crate::{oxide::interfaces::Interfaces, util::get_handle};

use super::{
    create_move::CreateMoveHook, frame_stage_notify::FrameStageNotifyHook,
    override_view::OverrideViewHook, paint::PaintHook, paint_traverse::PaintTraverseHook,
    poll_event::PollEventHook, swap_window::SwapWindowHook, Hook,
};

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

#[derive(Debug)]

pub struct Hooks(HashMap<String, Box<dyn Hook + 'static>>);

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Hooks {
        let mut hooks = HashMap::new();

        let override_view_hook =
            OverrideViewHook::init(&(*interfaces.client_mode.get_vmt()).override_view);
        hooks.insert(
            OverrideViewHook::name(),
            Box::new(override_view_hook) as Box<dyn Hook>,
        );

        let frame_stage_notify_hook =
            FrameStageNotifyHook::init(&(*interfaces.base_client.get_vmt()).frame_stage_notify);
        hooks.insert(
            FrameStageNotifyHook::name(),
            Box::new(frame_stage_notify_hook) as Box<dyn Hook>,
        );

        let paint_hook = PaintHook::init(&(*interfaces.engine_vgui.get_vmt()).paint);
        hooks.insert(PaintHook::name(), Box::new(paint_hook) as Box<dyn Hook>);

        let paint_traverse_hook =
            PaintTraverseHook::init(&(*interfaces.panel.get_vmt()).paint_traverse);
        hooks.insert(
            PaintTraverseHook::name(),
            Box::new(paint_traverse_hook) as Box<dyn Hook>,
        );

        let create_move_hook =
            CreateMoveHook::init(&((*interfaces.client_mode.get_vmt()).create_move));
        hooks.insert(
            CreateMoveHook::name(),
            Box::new(create_move_hook) as Box<dyn Hook>,
        );

        let sdl_handle = get_handle("./bin/libSDL2-2.0.so.0").unwrap() as *const _
            as *const *const *const c_void;

        let swap_window_ptr = (*sdl_handle) as usize + SWAPWINDOW_OFFSET;
        let swap_window_hook = SwapWindowHook::init(transmute(swap_window_ptr));
        hooks.insert(
            SwapWindowHook::name(),
            Box::new(swap_window_hook) as Box<dyn Hook>,
        );

        let poll_event_ptr = (*sdl_handle) as usize + POLLEVENT_OFFSET;
        let poll_event_hook = PollEventHook::init(transmute(poll_event_ptr));
        hooks.insert(
            PollEventHook::name(),
            Box::new(poll_event_hook) as Box<dyn Hook>,
        );

        Hooks(hooks)
    }
    pub fn get<T: Hook>(&mut self) -> ManuallyDrop<&mut Box<T>>{
        unsafe { ManuallyDrop::new(transmute(self.0.get_mut(&SwapWindowHook::name()).unwrap())) }
    }
    pub fn restore(&mut self) {
        for (_, hook) in &mut self.0 {
            hook.restore()
        }
    }
}
