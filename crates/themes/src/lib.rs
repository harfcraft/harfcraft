use gpui_kit::{
    App, SharedString,
    component::{Theme, ThemeRegistry},
};
use storage::AppStorage;

pub fn init(cx: &mut App, theme_name: SharedString) -> anyhow::Result<()> {
    let themes_dir = AppStorage::themes_dir()?;
    ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
        let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name) else {
            return;
        };
        let theme = theme.clone();
        // Keep this so that dock area separator does not appear white
        Theme::change(theme.mode, None, cx);
        Theme::global_mut(cx).apply_config(&theme);
    })?;
    Ok(())
}
