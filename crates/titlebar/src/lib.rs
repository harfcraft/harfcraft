use actions::{CloseWorkspace, PickWorkspace};
use gpui_kit::IntoElement;
use gpui_kit::assets::IconName;
use gpui_kit::base::h_flex;
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, DropdownButton};
use gpui_kit::component::label::Label;
use gpui_kit::{App, Entity, SharedString, Window, prelude::*};
use harfcraft_core::constants::APP_NAME;
use project::ProjectSource;
use workspace::Workspace;

#[derive(IntoElement)]
pub struct AppTitleBar {
    app_name: SharedString,
    workspace: Option<Entity<Workspace>>,
}

impl AppTitleBar {
    pub fn new(workspace: Option<Entity<Workspace>>) -> Self {
        Self {
            app_name: APP_NAME.into(),
            workspace,
        }
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        TitleBar::new().child(
            h_flex()
                .gap_4()
                .child(Label::new(self.app_name.clone()))
                .when_some(
                    self.workspace.as_ref(),
                    |this, workspace| -> gpui_kit::Div {
                        let workspace = workspace.read(cx);
                        this.child(
                            DropdownButton::new("active-workspace-dropdownbutton")
                                .button(
                                    Button::new("active-workspace-button")
                                        .icon(match workspace.project.source() {
                                            &ProjectSource::UnifiedFontObject => IconName::Package,
                                            &ProjectSource::DesignSpace => IconName::Layers2,
                                        })
                                        .label(workspace.name.clone()),
                                )
                                .dropdown_menu(|menu, _, _| {
                                    menu.menu("Open", Box::new(PickWorkspace))
                                        .menu("Close", Box::new(CloseWorkspace))
                                }),
                        )
                    },
                ),
        )
    }
}
