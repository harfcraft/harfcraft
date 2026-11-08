use dummypanel::DummyPanel;
use gpui_kit::base::dock::{DockArea, DockLayout, DockPlacement};
use gpui_kit::base::v_flex;
use gpui_kit::component::dock::{DockSkin, panel_handle};
use gpui_kit::prelude::*;
use gpui_kit::*;
use project::Project;
use workspace_session::WorkspaceSession;

pub struct Workspace {
    pub name: SharedString,
    pub project: Project,
    dock: Entity<DockArea>,
    focus: FocusHandle,
}

impl Focusable for Workspace {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Workspace {
    pub fn new(project: Project, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let session = WorkspaceSession::load(project.path());
        let (dock, _) = DockSkin::dock_area("workspace-dock", Some(1), window, cx);
        dock.update(cx, |dock, cx| {
            if let Some(dock_state) = session.dock_state {
                if let Err(err) = dock.load(dock_state, window, cx) {
                    tracing::error!("Failed to load dock state {err}");
                    Self::restore_default_dock(dock, window, cx);
                };
            } else {
                Self::restore_default_dock(dock, window, cx);
            }
            cx.notify();
        });
        cx.on_release(|this, cx| {
            this.save_session(cx);
        })
        .detach();
        Self {
            name: project.name().into(),
            project,
            dock,
            focus: cx.focus_handle(),
        }
    }
    fn restore_default_dock(dock: &mut DockArea, window: &mut Window, cx: &mut Context<DockArea>) {
        let explorer = cx.new(|cx| DummyPanel::new("Explorer", "Explorer files", cx));
        let glyphs = cx.new(|cx| DummyPanel::new("Glyphs", "Font Glyphs", cx));
        dock.set_dock(
            DockPlacement::Left,
            DockLayout::v_split()
                .child(
                    DockLayout::tabs().panel_view(panel_handle(explorer), cx),
                    None,
                )
                .child(
                    DockLayout::tabs().panel_view(panel_handle(glyphs), cx),
                    None,
                ),
            window,
            cx,
        );
    }
    pub fn save_session(&self, cx: &mut App) {
        let dock_state = self.dock.read(cx).dump(cx);
        let session = WorkspaceSession::new(self.project.path().to_path_buf(), dock_state);
        if let Err(err) = session.save() {
            tracing::error!("Failed to save workspace session {err}");
        } else {
            tracing::info!("Workspace session saved successfully");
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(actions::WORKSPACE_CONTEXT)
            .track_focus(&self.focus)
            .size_full()
            .child(self.dock.clone())
    }
}
