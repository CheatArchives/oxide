use std::{ alloc::{alloc, Layout}, intrinsics::{volatile_copy_nonoverlapping_memory, volatile_store}};


use libc::dlsym;

use crate::*;

#[derive(Debug)]
pub struct Interface<T: HasVmt<V>, V> {
    pub interface_ref: *mut T,
    pub old_vmt: *mut V,
}


impl<T: HasVmt<V>, V> Interface<T, V> {
    pub unsafe fn new(interface_ref: *mut T) -> Interface<T, V> {
        info!("creating interface {:?}", interface_ref);
        
        let old = interface_ref.read().get_vmt();
        let size = vmt_size(old as *mut c_void);

        let new = alloc(Layout::from_size_align(size, 0x8).unwrap()) as *mut V;
        volatile_copy_nonoverlapping_memory(new, old,size);
        interface_ref.read().set_vmt(new);
        info!("o: {:?}n: {:?}", old, new);
        Interface {
            interface_ref,
            old_vmt: old,
        }
    }
    unsafe fn create(handle: *mut c_void, name: &str) -> Result<Interface<T, V>, Box<dyn Error>> {
        let create_interface_fn: cfn!(*const c_void, *const c_char, *const c_int) =
            std::mem::transmute(dlsym(handle, CString::new("CreateInterface")?.as_ptr()));
        let interface_ref = create_interface_fn(CString::new(name)?.as_ptr(), std::ptr::null())
            as *const _ as *mut T;
        Ok(Interface::new(interface_ref))
    }

    pub fn get_vmt(&self) -> *mut V {
        unsafe{
            (*self.interface_ref).get_vmt()
        }
    }
    unsafe fn restore(&self) {
        self.interface_ref.read().set_vmt(self.old_vmt)
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Interfaces {
    pub base_client: Interface<BaseClient, VMTBaseClient>,
    pub base_engine: Interface<BaseEngine, VMTBaseEngine>,
    pub entity_list: Interface<EntityList, VMTEntityList>,
    pub engine_vgui: Interface<EngineVgui, VMTEngineVgui>,
    pub cvar: Interface<CVar, VMTCVar>,
    pub mat_surface: Interface<MatSurface, VMTMatSurface>,
    pub panel: Interface<Panel, VMTPanel>,
    pub model_info: Interface<ModelInfo, VMTModelInfo>,
    pub render_view: Interface<RenderView, VMTRenderView>,
    pub engine_trace: Interface<EngineTrace, VMTEngineTrace>,
    pub material_system: Interface<MaterialSystem, VMTMaterialSystem>,
    pub model_render: Interface<ModelRender, VMTModelRender>,
    pub game_movement: Interface<GameMovement, VMTGameMovement>,
    pub prediction: Interface<Prediction, VMTPrediction>,
    pub client_mode: Interface<ClientMode, VMTClientMode>,
}
impl Interfaces {
    pub unsafe fn create() -> Result<Interfaces, Box<dyn Error>> {
        log::info!("creating interfaces");
        let client_handle = get_handle("./tf/bin/client.so")?;
        let engine_handle = get_handle("./bin/engine.so")?;
        let matsurface_handle = get_handle("./bin/vguimatsurface.so")?;
        let vgui_handle = get_handle("./bin/vgui2.so")?;
        let materialsystem_handle = get_handle("./bin/materialsystem.so")?;
        let vstdlib_handle = get_handle("./bin/libvstdlib.so")?;
        let base_client: Interface<BaseClient, VMTBaseClient> =
            Interface::create(client_handle, "VClient017")?;

        let client_mode = (((base_client.interface_ref)
            .read()
            .vmt
            .read()
            .HudProcessInput as usize
            + 1) as *mut *mut ClientMode)
            .read();

        Ok(Interfaces {
            base_client,
            base_engine: Interface::create(engine_handle, "VEngineClient014")?,
            entity_list: Interface::create(client_handle, "VClientEntityList003")?,
            engine_vgui: Interface::create(engine_handle, "VEngineVGui002")?,
            cvar: Interface::create(vstdlib_handle, "VEngineCvar004")?,
            mat_surface: Interface::create(matsurface_handle, "VGUI_Surface030")?,
            panel: Interface::create(vgui_handle, "VGUI_Panel009")?,
            model_info: Interface::create(engine_handle, "VModelInfoClient006")?,
            render_view: Interface::create(engine_handle, "VEngineRenderView014")?,
            engine_trace: Interface::create(engine_handle, "EngineTraceClient003")?,
            material_system: Interface::create(materialsystem_handle, "VMaterialSystem081")?,
            model_render: Interface::create(engine_handle, "VEngineModel016")?,
            game_movement: Interface::create(client_handle, "GameMovement001")?,
            prediction: Interface::create(client_handle, "VClientPrediction001")?,
            client_mode: Interface::new(client_mode),
        })
    }

    pub unsafe fn restore(&self) {
        self.base_client.restore();
        self.base_engine.restore();
        self.entity_list.restore();
        self.engine_vgui.restore();
        self.cvar.restore();
        self.mat_surface.restore();
        self.panel.restore();
        self.model_info.restore();
        self.render_view.restore();
        self.engine_trace.restore();
        self.material_system.restore();
        self.model_render.restore();
        self.game_movement.restore();
        self.prediction.restore();
        self.client_mode.restore();
    }
}
