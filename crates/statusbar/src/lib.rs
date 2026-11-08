use gpui_kit::component::status_bar::StatusBar;
use gpui_kit::*;
use gpui_kit::{IntoElement, prelude::*};
use workspace::Workspace;

#[derive(IntoElement)]
pub struct AppStatusbar {
    workspace: Option<Entity<Workspace>>,
}

impl AppStatusbar {
    pub fn new(workspace: Option<Entity<Workspace>>) -> Self {
        Self { workspace }
    }
}

impl RenderOnce for AppStatusbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        StatusBar::new()
            .when_none(&self.workspace, |this| this.child("Welcome"))
            .when_some(self.workspace.as_ref(), |this, workspace| {
                let workspace = workspace.read(cx);
                this.child(workspace.name.clone())
            })
    }
}
