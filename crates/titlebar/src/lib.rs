use actions::CloseActiveWorkspace;
use gpui_kit::IntoElement;
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, DropdownButton};
use gpui_kit::{App, Entity, Window, prelude::*};
use harfcraft_core::constants::APP_NAME;
use workspace::Workspace;

#[derive(IntoElement)]
pub struct AppTitleBar {
    workspace: Option<Entity<Workspace>>,
}

impl AppTitleBar {
    pub fn new(workspace: Option<Entity<Workspace>>) -> Self {
        Self { workspace }
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        TitleBar::new()
            .when_none(&self.workspace, |this| this.child(APP_NAME.to_string()))
            .when_some(self.workspace.as_ref(), |this, workspace| {
                let workspace = workspace.read(cx);
                this.child(
                    DropdownButton::new("active-workspace-dropdownbutton")
                        .button(
                            Button::new("active-workspace-button").label(workspace.name.clone()),
                        )
                        .dropdown_menu(|menu, _, _| {
                            menu.menu("Close", Box::new(CloseActiveWorkspace))
                        }),
                )
            })
    }
}
