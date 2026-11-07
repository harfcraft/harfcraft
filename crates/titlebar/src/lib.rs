use gpui_kit::component::{ActiveTheme, TitleBar};
use gpui_kit::{App, Entity, Window, prelude::*};
use gpui_kit::{IntoElement, div};
use harfcraft_core::constants::APP_NAME;
use project::Project;

#[derive(IntoElement)]
pub struct AppTitleBar {
    project: Option<Entity<Project>>,
}

impl AppTitleBar {
    pub fn new(project: Option<Entity<Project>>) -> Self {
        Self { project }
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        TitleBar::new()
            .child(div().child(APP_NAME).text_color(cx.theme().primary))
            .when_some(self.project.as_ref(), |this, project| {
                let name = project.read(cx).name().to_string();
                this.child(name)
            })
    }
}
