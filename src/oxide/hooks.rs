use std::f32::consts::PI;

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
    cmd: &'static mut UserCmd,
) -> bool {
    let entity_count = call!(i!(entity_list), GetMaxEntities);

    let p_local = get_plocal().unwrap();
    let p_angles = call!(p_local, GetAbsAngles);
    for i in 0..entity_count {
        let Some(ent) = get_ent(i) else {
            continue;
        };
        if call!(ent, GetTeamNumber) == call!(p_local, GetTeamNumber) {
            continue;
        }
        let diff = p_local.m_vecOrigin - ent.m_vecOrigin;
        dbg!(p_local.m_vecOrigin);
        let mut ang = p_angles.clone();
        dbg!(ang);
        ang.yaw = diff.y.atan2(diff.x) / PI * 180f32 + 180f32;
        let dist2d = (diff.x.powi(2) + diff.y.powi(2)).sqrt();
        ang.pitch = diff.z.atan2(dist2d) / PI * 180f32;
        cmd.viewangles = ang;

        let net = get_networkabe(ent);
        struct ConditionData {
            cond_0: usize,
            cond_1: usize,
            cond_2: usize,
            cond_3: usize,
        }
        ent.condition_bitsz

        //bool zoomed    = HasCondition<TFCond_Zoomed>(entity);
        //
        //template <condition cond> inline bool HasCondition(CachedEntity *ent)
        //{
        //    if (cond < condition(96) && CondBitCheck<cond>(CE_VAR(ent, netvar._condition_bits, condition_data_s)))
        //        return true;
        //
        //    return CondBitCheck<cond>(CE_VAR(ent, netvar.iCond, condition_data_s));
        //}
        //#define CE_VAR(entity, offset, type) NET_VAR(RAW_ENT(entity), offset, type)
        //
        //template <condition cond> inline bool CondBitCheck(condition_data_s &data)
        //{
        //    if (cond >= 32 * 3)
        //        return data.cond_3 & (1u << (cond % 32));
        //
        //    if (cond >= 32 * 2)
        //        return data.cond_2 & (1u << (cond % 32));
        //
        //    if (cond >= 32)
        //        return data.cond_1 & (1u << (cond % 32));
        //
        //    return data.cond_0 & (1u << (cond));
        //}
        cmd.buttons.IN_ATTACK = true;

        break;
    }

    true
}

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
}

impl Hooks {
    pub unsafe fn init(interfaces: &Interfaces) -> Result<Hooks, Box<dyn Error>> {
        let create_move = Hook::init(
            addr_of_mut!((*interfaces.client_mode.get_vmt()).CreateMove) as *mut *const c_void,
            create_move_hook as *const c_void,
        );
        let sdl_handle =
            get_handle("./bin/libSDL2-2.0.so.0")? as *const _ as *const *const *const c_void;
        let swap_window_ptr = ((*sdl_handle) as usize + SWAPWINDOW_OFFSET) as *mut *const c_void;
        let swap_window = Hook::init(
            swap_window_ptr,
            transmute::<SwapWindowFn, *const c_void>(swap_window),
        );
        Ok(Hooks {
            create_move,
            swap_window,
        })
    }
    pub unsafe fn restore(&self) {
        self.swap_window.restore();
        self.create_move.restore();
    }
}
