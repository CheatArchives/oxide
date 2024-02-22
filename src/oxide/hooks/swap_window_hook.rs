use sdl2_sys::SDL_GL_MakeCurrent;

use crate::*;

pub type SwapWindowFn = cfn!(c_void, *mut sdl2_sys::SDL_Window);

pub unsafe extern "C-unwind" fn swap_window_hook(window: *mut sdl2_sys::SDL_Window) -> c_void { //TODO: move Menu init inside this if
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
