use config::AppConfig;
use gpui_kit::base::Root;
use gpui_kit::component::TitleBar;
use gpui_kit::prelude::*;
use gpui_kit::*;
use harfcraft_core::constants::{APP_ID, APP_NAME};
use std::path::PathBuf;
use workbench::Workbench;

pub fn run(path: Option<PathBuf>) {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(move |cx| {
            gpui_kit::init(cx);

            let config = AppConfig::load();
            let theme_name = config.theme().into();
            if let Err(err) = themes::init(cx, theme_name) {
                tracing::error!(%err);
            };
            cx.set_app_identity(APP_ID, APP_NAME);
            cx.activate(true);

            let window_options = WindowOptions {
                app_id: Some(APP_ID.into()),
                window_decorations: Some(gpui_kit::WindowDecorations::Client),
                ..TitleBar::window_options()
            };
            cx.spawn(async move |cx| {
                cx.open_window(window_options, |window, cx| {
                    let workbench = cx.new(|cx| Workbench::new(path, window, cx));
                    cx.new(|cx| Root::new(workbench, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}
