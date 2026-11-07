use ::actions::CloseWorkspace;
use actions::PickWorkspace;
use gpui_kit::base::v_flex;
use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::Notification;
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;
use session::AppSession;
use std::path::PathBuf;
use titlebar::AppTitleBar;
use workspace::Workspace;

pub struct Workbench {
    session: AppSession,
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
        let workbench = Self {
            workspace: None,
            session: AppSession::load(),
            focus: cx.focus_handle(),
        };

        if let Some(path) = path.or(workbench.session.last_project_path.clone()) {
            cx.spawn_in(window, async move |this, cx| {
                this.update_in(cx, move |this, window, cx| {
                    this.open_workspace(path, window, cx);
                })
            })
            .detach();
        }

        workbench
    }
    fn open_workspace(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        match Project::from_path(&path) {
            Ok(project) => {
                self.session
                    .record_project_open(project.path().to_path_buf());
                self.workspace = Some(cx.new(|cx| Workspace::new(project, window, cx)));
                cx.notify();
            }
            Err(err) => {
                tracing::error!("Failed to open project ({path:?}): {err}");
                window.push_notification(
                    Notification::error(err.to_string())
                        .title(format!("Failed to open project {}", path.to_string_lossy())),
                    cx,
                );
            }
        }
    }
    fn pick_workspace(&mut self, _: &PickWorkspace, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        cx.spawn_in(window, async move |this, cx| {
            let path = prompt.await.ok()?.ok()??.first()?.clone();
            _ = this.update_in(cx, |this, window, cx| {
                this.open_workspace(path, window, cx);
            });
            Some(())
        })
        .detach();
    }
    fn close_workspace(
        &mut self,
        _: &CloseWorkspace,
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
            .on_action(cx.listener(Self::pick_workspace))
            .on_action(cx.listener(Self::close_workspace))
            .track_focus(&self.focus)
            .size_full()
            .child(AppTitleBar::new(self.workspace.clone()))
            .when_none(&self.workspace, |this| this.child("Welcome Screen"))
            .when_some(self.workspace.as_ref(), |this, workspace| {
                this.child(workspace.clone())
            })
    }
}
