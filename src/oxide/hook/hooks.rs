use std::{
    collections::HashMap,
    ptr::{addr_of, addr_of_mut},
};

//use super::{
//    create_move::{create_move_hook, CreateMoveFn},
//    frame_stage_notify::{frame_stage_notify_hook, FrameStageNotifyFn},
//    override_view::{override_view_hook, OverrideViewFn},
//    paint,
//    paint_traverse::{paint_traverse_hook, PaintRraverseFn},
//    poll_event::{poll_event_hook, PollEventFn},
//    swap_window::{swap_window_hook, SwapWindowFn}, Hook,
//};

use crate::oxide::interfaces::Interfaces;

use super::{create_move::CreateMoveHook, Hook};

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

#[derive(Debug)]

pub struct Hooks {
    pub hooks: HashMap<String, Box<dyn Hook + 'static>>, //pub create_move: Hook<CreateMoveFn>,
                                                         //pub swap_window: Hook<SwapWindowFn>,
                                                         //pub poll_event: Hook<PollEventFn>,
                                                         //pub paint_traverse: Hook<PaintRraverseFn>,
                                                         //pub override_view: Hook<OverrideViewFn>,
                                                         //pub frame_stage_notify: Hook<FrameStageNotifyFn>,
                                                         //pub paint: Hook<paint::PaintFn>,
}

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Hooks {
        //let create_move = Hook::<CreateMoveFn>::init(
        //    transmute(&(*interfaces.client_mode.get_vmt()).create_move),
        //    create_move_hook,
        //);

        //let override_view = Hook::<OverrideViewFn>::init(
        //    transmute(&(*interfaces.client_mode.get_vmt()).override_view),
        //    override_view_hook,
        //);

        //let paint_traverse = Hook::<PaintRraverseFn>::init(
        //    transmute(&(*interfaces.panel.get_vmt()).paint_traverse),
        //    paint_traverse_hook,
        //);

        //let frame_stage_notify = Hook::<FrameStageNotifyFn>::init(
        //    transmute(&(*interfaces.base_client.get_vmt()).frame_stage_notify),
        //    frame_stage_notify_hook,
        //);

        //let paint = Hook::<paint::PaintFn>::init(
        //    transmute(&(*interfaces.engine_vgui.get_vmt()).paint),
        //    paint::paint_hook,
        //);

        //let sdl_handle = get_handle("./bin/libSDL2-2.0.so.0").unwrap() as *const _
        //    as *const *const *const c_void;

        //let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut SwapWindowFn;
        //let init = Hook::init(swap_window_ptr, swap_window_hook);
        //let swap_window = init;

        //let poll_event_ptr = ((*sdl_handle) as usize + POLLEVENT_OFFSET) as *mut PollEventFn;
        //let poll_event = Hook::init(poll_event_ptr, poll_event_hook);

        let mut create_move_hook =
            CreateMoveHook::init(&((*interfaces.client_mode.get_vmt()).create_move));
        let mut hooks = HashMap::new();

        create_move_hook.before.push(|_, _, _| {
            dbg!("create_move");
        });
        hooks.insert(
            CreateMoveHook::name().to_owned(),
            Box::new(create_move_hook) as Box<dyn Hook>,
        );

        Hooks {
            hooks, //create_move,
                   //override_view,
                   //paint_traverse,
                   //swap_window,
                   //poll_event,
                   //frame_stage_notify,
                   //paint,
        }
    }
    pub fn restore(&mut self) {
        //self.create_move.restore();
        //self.override_view.restore();
        //self.paint_traverse.restore();
        //self.swap_window.restore();
        //self.poll_event.restore();
        //self.frame_stage_notify.restore();
        //self.paint.restore();
    }
}
