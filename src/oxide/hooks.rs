
use crate::*;
use sdl2_sys::*;

static SWAPWINDOW_OFFSET: usize = 0xFD648;
static POLLEVENT_OFFSET: usize = 0xFCF64;

type SwapWindowFn = cfn!(c_void, *mut sdl2_sys::SDL_Window);

unsafe extern "C-unwind" fn swap_window(window: *mut sdl2_sys::SDL_Window) -> c_void {

    //TODO: move Menu init inside this if 
    if MENU.is_null() {
        let menu_ptr = alloc(Layout::new::<Menu>()) as *mut _ as *mut Menu;
        *menu_ptr = Menu::init(window).unwrap();
        MENU = menu_ptr as *mut _ as *mut c_void;

    }

    SDL_GL_MakeCurrent(window, m!().ctx);

    m!().run(window);

    SDL_GL_MakeCurrent(window, m!().old_ctx);
    (transmute::<*const c_void, SwapWindowFn>(o!().hooks.swap_window.org))(window)
}
pub unsafe extern "C-unwind" fn create_move_hook(
    client_mode: *mut ClientMode,
    input_sample_time: c_float,
    cmd: *mut UserCmd,
) -> bool {
    let entity_count = ((*o!().interfaces.entity_list.get_vmt()).GetMaxEntities)(o!().interfaces.entity_list.interface_ref);

    for i in 0..entity_count {
        let ent = ((*o!().interfaces.entity_list.get_vmt()).GetClientEntity)(o!().interfaces.entity_list.interface_ref,i);
        if ent.is_null() {
            continue;
        }
        dbg!(ent);
    }
    
    true
}

#[derive(Debug, Clone, Copy)]
struct Hook {
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
    pub swap_window: Hook
}

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Result<Hooks, Box<dyn Error>> {
        let create_move = 
            Hook::init(
                addr_of_mut!((*interfaces.client_mode.get_vmt()).CreateMove) as *mut *const c_void,
                create_move_hook as *const c_void,
            );
        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;
            let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut *const c_void;
        let swap_window = Hook::init(swap_window_ptr,transmute::<SwapWindowFn,*const c_void>(swap_window));
        Ok(Hooks { create_move, swap_window})
    }
    pub unsafe fn restore(&self) {

        self.swap_window.restore();
        self.create_move.restore();
    }
}
