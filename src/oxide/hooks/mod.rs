use crate::*;

module_export!(swap_window_hook);
module_export!(create_move);
module_export!(poll_event);

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;



#[derive(Debug, Clone, Copy)]
pub struct Hook {
    pub org: *const c_void,
    pub target: *mut *const c_void,
}

impl Hook {
    unsafe fn init(target: *mut *const c_void, hook: *const c_void) -> Hook {
        let org = *target;
        *target = hook;
        Hook { org, target }
    }
    unsafe fn restore(&self) {
        *self.target = self.org
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hooks {
    pub create_move: Hook,
    pub swap_window: Hook,
    pub poll_event: Hook,
}

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Result<Hooks, std::boxed::Box<dyn Error>> {
        let create_move = Hook::init(
            addr_of!((*interfaces.client_mode.get_vmt()).create_move) as *mut *const c_void,
            create_move_hook as *const c_void,
        );

        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;

        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut *const c_void;
        let swap_window = Hook::init(
            swap_window_ptr,
            transmute::<SwapWindowFn, *const c_void>(swap_window_hook),
        );

        let poll_event_ptr = ((*sdl_handle) as usize + POLLEVENT_OFFSET) as *mut *const c_void;
        let poll_event = Hook::init(
            poll_event_ptr,
            transmute::<PollEventFn, *const c_void>(poll_event_hook),
        );
        Ok(Hooks {
            create_move,
            swap_window,
            poll_event
        })
    }
    pub unsafe fn restore(&self) {
        self.swap_window.restore();
        self.create_move.restore();
        self.poll_event.restore();
    }
}
