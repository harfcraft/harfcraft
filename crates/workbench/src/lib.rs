use ::actions::CloseActiveWorkspace;
use gpui_kit::base::v_flex;
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;
use session::AppSession;
use std::path::PathBuf;
use titlebar::AppTitleBar;
use workspace::Workspace;

pub struct Workbench {
    workspace: Option<Entity<Workspace>>,
    focus: FocusHandle,
}

impl Focusable for Workbench {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Workbench {
    pub fn new(path: Option<PathBuf>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut session = AppSession::load();
        let project = path.or(session.last_project_path.clone()).and_then(|path| {
            match Project::from_path(path.clone()) {
                Ok(project) => {
                    session.record_project_open(project.path().to_path_buf());
                    Some(project)
                }
                Err(err) => {
                    tracing::error!("Failed to open project ({path:?}): {err}");
                    None
                }
            }
        });

        Self {
            workspace: project.map(|project| cx.new(|cx| Workspace::new(project, window, cx))),
            focus: cx.focus_handle(),
        }
    }
    fn close_active_workspace(
        &mut self,
        _: &CloseActiveWorkspace,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace = None;
        cx.notify();
    }
}

impl Render for Workbench {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(actions::WORKBENCH_CONTEXT)
            .on_action(cx.listener(Self::close_active_workspace))
            .track_focus(&self.focus)
            .size_full()
            .child(AppTitleBar::new(self.workspace.clone()))
            .when_none(&self.workspace, |this| this.child("Welcome Screen"))
            .when_some(self.workspace.as_ref(), |this, workspace| {
                this.child(workspace.clone())
            })
    }
}
