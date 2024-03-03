use sdl2_sys::{SDL_GL_MakeCurrent, SDL_ShowCursor, SDL_ENABLE};

use crate::*;

pub type SwapWindowFn = cfn!(c_void, *mut sdl2_sys::SDL_Window);

pub unsafe extern "C-unwind" fn swap_window_hook(window: *mut sdl2_sys::SDL_Window) -> c_void {
    if MENU.is_none() {
        let menu_ptr = alloc(Layout::new::<Menu>()) as *mut _ as *mut Menu;
        *menu_ptr = Menu::init(window).unwrap();
        MENU = Some(menu_ptr as *mut _ as *mut c_void);
    }

    menu!().run(window);

    (oxide!().hooks.swap_window.org)(window)
}
