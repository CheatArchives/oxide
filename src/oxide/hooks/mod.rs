use std::intrinsics::transmute_unchecked;

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
pub struct Hook<T> {
    pub org: T,
    pub target: *mut T,
}

impl<T: Clone> Hook<T> {
    unsafe fn init(target: *mut T, hook: T) -> Self {
        let org = target;
        *target = hook;
        Hook { org:org.read(), target }
    }
    unsafe fn restore(&mut self) {
        self.target = transmute_unchecked(self.org.clone())
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
    pub unsafe fn init(interfaces: &Interfaces) -> Result<Hooks, std::boxed::Box<dyn Error>> {
        //todo: move all those to thier files
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

        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;

        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;
        let swap_window = Hook::init(swap_window_ptr, swap_window_hook);

        let poll_event_ptr = ((*sdl_handle) as usize + POLLEVENT_OFFSET) as *mut PollEventFn;
        let poll_event = Hook::init(poll_event_ptr, poll_event_hook);
        Ok(Hooks {
            create_move,
            swap_window,
            poll_event,
            paint_traverse,
            override_view,
            frame_stage_notify
        })
    }
    pub unsafe fn restore(&mut self) {
        self.swap_window.restore();
        self.create_move.restore();
        self.poll_event.restore();
        self.paint_traverse.restore();
    }
}
