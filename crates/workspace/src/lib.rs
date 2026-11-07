use gpui_kit::base::v_flex;
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;

pub struct Workspace {
    pub name: SharedString,
    pub project: Project,
    focus: FocusHandle,
}

impl Focusable for Workspace {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Workspace {
    pub fn new(project: Project, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            name: project.name().into(),
            project,
            focus: cx.focus_handle(),
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(actions::WORKSPACE_CONTEXT)
            .track_focus(&self.focus)
            .size_full()
            .child(self.name.clone())
            .child(self.project.path().to_string_lossy().to_string())
            .child("STATUS_BAR")
    }
}
