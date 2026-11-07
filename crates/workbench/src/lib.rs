use gpui_kit::base::v_flex;
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;
use session::AppSession;
use std::path::PathBuf;
use titlebar::AppTitleBar;

pub struct Workbench {
    project: Option<Entity<Project>>,
    focus: FocusHandle,
}

impl Focusable for Workbench {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Workbench {
    pub fn new(path: Option<PathBuf>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut session = AppSession::load();
        let project = path.or(session.last_project_path.clone()).and_then(|path| {
            match Project::from_path(path.clone()) {
                Ok(project) => {
                    session.record_project_open(project.path().to_path_buf());
                    Some(cx.new(|_| project))
                }
                Err(err) => {
                    tracing::error!("Failed to open project ({path:?}): {err}");
                    None
                }
            }
        });

        Self {
            project,
            focus: cx.focus_handle(),
        }
    }
}

impl Render for Workbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(AppTitleBar::new(self.project.clone()))
            .when_none(&self.project, |this| this.child("Welcome Screen"))
            .when_some(self.project.as_ref(), |this, project| {
                this.child(project.read(cx).path().to_string_lossy().to_string())
            })
    }
}
