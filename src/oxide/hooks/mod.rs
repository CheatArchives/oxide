use std::{intrinsics::transmute_unchecked, mem::MaybeUninit};

use libc::CLONE_VFORK;

use crate::*;

module_export!(swap_window_hook);
module_export!(create_move);
module_export!(poll_event);
module_export!(paint_traverse);
module_export!(override_view);
module_export!(frame_stage_notify);

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

        let sdl_handle = get_handle("./bin/libSDL2-2.0.so.0").unwrap() as *const _
            as *const *const *const c_void;

        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;
        let swap_window = Hook::init(swap_window_ptr, swap_window_hook);

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
        }
    }
    pub fn restore(&mut self) {
        self.create_move.restore();
        self.override_view.restore();
        self.paint_traverse.restore();
        self.swap_window.restore();
        self.poll_event.restore();
        self.frame_stage_notify.restore();
    }
}

#[derive(Debug, Clone)]
pub struct Hook2<T>
where
    T: Clone + Copy,
{
    pub org: T,
    pub target: *mut T,
    pub hook: T,
}
impl<T: Clone + Copy> Hook2<T> {
    fn init(target: *mut T) {
        unsafe {
            unsafe extern "C" fn hook(first: *mut c_void, mut args: ...) {
                args.arg()
            }

            let org = *target;
            *target = transmute_unchecked(hook);
             
        }
    }
}
