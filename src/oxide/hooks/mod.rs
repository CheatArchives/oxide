use std::mem::transmute;

use libc::c_void;

use crate::util::get_handle;

use self::{
    create_move::{create_move_hook, CreateMoveFn},
    frame_stage_notify::{frame_stage_notify_hook, FrameStageNotifyFn},
    override_view::{override_view_hook, OverrideViewFn},
    paint_traverse::{paint_traverse_hook, PaintRraverseFn},
    poll_event::{poll_event_hook, PollEventFn},
    swap_window::{swap_window_hook, SwapWindowFn},
};

use super::interfaces::Interfaces;

pub mod create_move;
pub mod frame_stage_notify;
pub mod override_view;
pub mod paint;
pub mod paint_traverse;
pub mod poll_event;
pub mod swap_window;

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

#[derive(Debug, Clone)]
pub struct Hook<T>
where
    T: Clone + Copy,
{
    pub org: T,
    pub target: *mut T,
}

impl<T: Clone + Copy + std::fmt::Debug> Hook<T> {
    unsafe fn init(target: *mut T, hook: T) -> Self {
        let org = target.read();
        *target = hook;
        Hook { org, target }
    }
    fn restore(&mut self) {
        unsafe { *self.target = self.org }
    }
}

#[derive(Debug, Clone)]
pub struct Hooks {
    pub create_move: Hook<CreateMoveFn>,
    pub swap_window: Hook<SwapWindowFn>,
    pub poll_event: Hook<PollEventFn>,
    pub paint_traverse: Hook<PaintRraverseFn>,
    pub override_view: Hook<OverrideViewFn>,
    pub frame_stage_notify: Hook<FrameStageNotifyFn>,
    pub paint: Hook<paint::PaintFn>,
}

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Hooks {
        let create_move = Hook::<CreateMoveFn>::init(
            transmute(&(*interfaces.client_mode.get_vmt()).create_move),
            create_move_hook,
        );

        let override_view = Hook::<OverrideViewFn>::init(
            transmute(&(*interfaces.client_mode.get_vmt()).override_view),
            override_view_hook,
        );

        let paint_traverse = Hook::<PaintRraverseFn>::init(
            transmute(&(*interfaces.panel.get_vmt()).paint_traverse),
            paint_traverse_hook,
        );

        let frame_stage_notify = Hook::<FrameStageNotifyFn>::init(
            transmute(&(*interfaces.base_client.get_vmt()).frame_stage_notify),
            frame_stage_notify_hook,
        );

        let paint = Hook::<paint::PaintFn>::init(
            transmute(&(*interfaces.engine_vgui.get_vmt()).paint),
            paint::paint_hook,
        );

        let sdl_handle = get_handle("./bin/libSDL2-2.0.so.0").unwrap() as *const _
            as *const *const *const c_void;

        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;
        let init = Hook::init(swap_window_ptr, swap_window_hook);
        let swap_window = init;

        let poll_event_ptr = ((*sdl_handle) as usize + POLLEVENT_OFFSET) as *mut PollEventFn;
        let poll_event = Hook::init(poll_event_ptr, poll_event_hook);

        #[allow(invalid_value)]
        Hooks {
            create_move,
            override_view,
            paint_traverse,
            swap_window,
            poll_event,
            frame_stage_notify,
            paint,
        }
    }
    pub fn restore(&mut self) {
        self.create_move.restore();
        self.override_view.restore();
        self.paint_traverse.restore();
        self.swap_window.restore();
        self.poll_event.restore();
        self.frame_stage_notify.restore();
        self.paint.restore();
    }
}
