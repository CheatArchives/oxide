use crate::{
    define_hook, sdk::{client_mode::ClientMode, view_setup::ViewSetup}
};


define_hook!(
    OverrideViewHook,
    "OverrideView",
    (),
    (),
    client_move,
    &mut ClientMode,
    view_setup,
    &mut ViewSetup
);
