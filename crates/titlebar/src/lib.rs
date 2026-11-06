use gpui_kit::{component::TitleBar, *};
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
        let title = match self.project.as_ref() {
            Some(proj) => {
                let project_ref = proj.read(cx);
                format!("{APP_NAME} — {}", project_ref.name())
            }
            None => APP_NAME.to_string(),
        };
        TitleBar::new().child(title)
    }
}
