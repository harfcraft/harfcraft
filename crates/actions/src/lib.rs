use gpui_kit::actions;

pub const WORKBENCH_CONTEXT: &'static str = "workbench";
pub const WORKSPACE_CONTEXT: &'static str = "workspace";
pub const BUFFER_CONTEXT: &'static str = "buffer";

actions!(WORKBENCH_CONTEXT, [PickWorkspace, CloseWorkspace]);

pub fn init() {}
