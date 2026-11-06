use gpui_kit::base::v_flex;
use gpui_kit::component::TitleBar;
use gpui_kit::*;
use gpui_kit::{base::Root, prelude::FluentBuilder};
use harfcraft_core::constants::{APP_ID, APP_NAME};
use project::Project;
use session::AppSession;
use std::path::PathBuf;
use titlebar::AppTitleBar;

pub struct AppShell {
    session: AppSession,
    project: Option<Entity<Project>>,
}

impl AppShell {
    fn new(path: Option<PathBuf>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
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

        Self { session, project }
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(AppTitleBar::new(self.project.clone()))
            .when_none(&self.project, |this| this.child("Welcome Screen"))
            .when_some(self.project.as_ref(), |this, project| {
                this.child(project.read(cx).path().to_string_lossy().to_string())
            })
    }
}

pub fn run(path: Option<PathBuf>) {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(move |cx| {
            gpui_kit::init(cx);
            cx.set_app_identity(APP_ID, APP_NAME);
            cx.activate(true);

            let window_options = WindowOptions {
                app_id: Some(APP_ID.into()),
                window_decorations: Some(gpui_kit::WindowDecorations::Client),
                ..TitleBar::window_options()
            };
            cx.spawn(async move |cx| {
                cx.open_window(window_options, |window, cx| {
                    let shell = cx.new(|cx| AppShell::new(path, window, cx));
                    cx.new(|cx| Root::new(shell, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}
