use crate::shell::AppShell;
use gpui_kit::base::Root;
use gpui_kit::component::TitleBar;
use gpui_kit::*;
use harfcraft_core::constants::{APP_ID, APP_NAME};
use std::path::PathBuf;

mod shell;

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
