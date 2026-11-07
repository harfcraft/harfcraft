use ::actions::CloseWorkspace;
use actions::{LoadWorkspace, PickWorkspace};
use gpui_kit::base::v_flex;
use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::Notification;
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;
use session::AppSession;
use std::path::{Path, PathBuf};
use titlebar::AppTitleBar;
use welcome::Welcome;
use workspace::Workspace;

pub struct Workbench {
    session: Entity<AppSession>,
    workspace: Option<Entity<Workspace>>,
    welcome: Entity<Welcome>,
    focus: FocusHandle,
}

impl Focusable for Workbench {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Workbench {
    pub fn new(path: Option<PathBuf>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let session = AppSession::load();
        if let Some(path) = path.or(session.last_project_path.clone()) {
            cx.spawn_in(window, async move |this, cx| {
                this.update_in(cx, move |this, window, cx| {
                    this.load_workspace(&path, window, cx);
                })
            })
            .detach();
        }
        let session = cx.new(|_| session);
        let workbench = Self {
            workspace: None,
            welcome: cx.new(|cx| Welcome::new(session.clone(), window, cx)),
            session,
            focus: cx.focus_handle(),
        };
        workbench.focus.focus(window, cx);

        workbench
    }
    fn load_workspace(&mut self, path: &Path, window: &mut Window, cx: &mut Context<Self>) {
        match Project::from_path(path) {
            Ok(project) => {
                self.session.update(cx, |session, cx| {
                    session.record_project_open(project.path().to_path_buf());
                    cx.notify();
                });
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
    fn on_pick_workspace(
        &mut self,
        _: &PickWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        cx.spawn_in(window, async move |this, cx| {
            let paths = prompt.await.ok()?.ok()??;
            let path = paths.first()?;
            _ = this.update_in(cx, |this, window, cx| {
                this.load_workspace(path, window, cx);
            });
            Some(())
        })
        .detach();
    }
    fn on_load_workspace(
        &mut self,
        LoadWorkspace(path): &LoadWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.load_workspace(path, window, cx);
    }
    fn on_close_workspace(
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
            .on_action(cx.listener(Self::on_pick_workspace))
            .on_action(cx.listener(Self::on_load_workspace))
            .on_action(cx.listener(Self::on_close_workspace))
            .track_focus(&self.focus)
            .size_full()
            .child(AppTitleBar::new(self.workspace.clone()))
            .when_none(&self.workspace, |this| this.child(self.welcome.clone()))
            .when_some(self.workspace.as_ref(), |this, workspace| {
                this.child(workspace.clone())
            })
    }
}
