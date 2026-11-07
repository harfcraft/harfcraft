use gpui_kit::{Action, actions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const WORKBENCH_CONTEXT: &'static str = "workbench";
pub const WORKSPACE_CONTEXT: &'static str = "workspace";
pub const BUFFER_CONTEXT: &'static str = "buffer";

actions!(WORKBENCH_CONTEXT, [PickWorkspace, CloseWorkspace]);

#[derive(Clone, PartialEq, Default, Debug, Action, Serialize, Deserialize, JsonSchema)]
#[action(namespace = WORKBENCH_CONTEXT)]
pub struct LoadWorkspace(pub PathBuf);

pub fn init() {}
