use std::path::PathBuf;

use actions::{LoadWorkspace, PickWorkspace};
use gpui_kit::assets::IconName;
use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::label::Label;
use gpui_kit::component::separator::Separator;
use gpui_kit::prelude::*;
use gpui_kit::*;
use harfcraft_core::constants::{APP_DESCRIPTION, APP_NAME};
use session::AppSession;

pub struct Welcome {
    recent_projects: Vec<SharedString>,
    app_name: SharedString,
    app_description: SharedString,
    focus: FocusHandle,
    _subscription: Subscription,
}

impl Focusable for Welcome {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Welcome {
    pub fn new(session: Entity<AppSession>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut welcome = Self {
            recent_projects: Vec::new(),
            app_name: APP_NAME.into(),
            app_description: APP_DESCRIPTION.into(),
            focus: cx.focus_handle(),
            _subscription: cx.observe(&session, Self::update_recent_projects),
        };
        welcome.update_recent_projects(session, cx);

        welcome
    }

    fn update_recent_projects(&mut self, session: Entity<AppSession>, cx: &mut Context<Self>) {
        self.recent_projects = session
            .read(cx)
            .recent_projects
            .iter()
            .map(|path| path.to_string_lossy().into())
            .collect();
        cx.notify();
    }

    pub fn open_workspace(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.dispatch_action(Box::new(PickWorkspace), cx);
    }
}

impl Render for Welcome {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus)
            .items_center()
            .items_stretch()
            .text_center()
            .w_128()
            .m_auto()
            .gap_2()
            .child(Label::new(self.app_name.clone()).text_3xl().font_bold())
            .child(
                Label::new(self.app_description.clone())
                    .text_sm()
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                Button::new("open-workspace")
                    .ghost()
                    .label("Open")
                    .icon(IconName::FolderOpen)
                    .on_click(cx.listener(|_, _, window, cx| {
                        window.dispatch_action(Box::new(PickWorkspace), cx);
                    })),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(Separator::horizontal().flex_1())
                    .child(
                        Label::new("RECENT PROJECTS")
                            .text_color(cx.theme().muted_foreground)
                            .text_xs(),
                    )
                    .child(Separator::horizontal().flex_1()),
            )
            .children(
                self.recent_projects
                    .iter()
                    .enumerate()
                    .map(|(index, path)| {
                        let path_copy = path.clone();
                        Button::new(format!("load-project-{index}"))
                            .ghost()
                            .label(path.clone())
                            .on_click(cx.listener(move |_, _, window, cx| {
                                window.dispatch_action(
                                    Box::new(LoadWorkspace(PathBuf::from(path_copy.as_str()))),
                                    cx,
                                );
                            }))
                    }),
            )
    }
}
