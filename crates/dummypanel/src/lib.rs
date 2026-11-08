use gpui_kit::base::dock::{Panel as BasePanel, PanelEvent, PanelInfo, PanelState, register_panel};
use gpui_kit::component::dock::{Panel, panel_handle};
use gpui_kit::prelude::*;
use gpui_kit::*;
use serde::{Deserialize, Serialize};

pub const PANEL_NAME: &'static str = "dummy-panel";

pub fn init(cx: &mut App) {
    register_panel(cx, PANEL_NAME, move |content, _, cx| {
        let state = content.state();
        let entity = DummyPanel::restore(state, cx);
        panel_handle(entity)
    });
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DummyPanelState {
    pub name: SharedString,
    pub content: SharedString,
}

impl DummyPanelState {
    pub fn new(name: SharedString, content: SharedString) -> Self {
        Self { name, content }
    }
}

pub struct DummyPanel {
    pub state: DummyPanelState,
    focus: FocusHandle,
}

impl Focusable for DummyPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl EventEmitter<PanelEvent> for DummyPanel {}

impl BasePanel for DummyPanel {
    fn panel_name(&self) -> &'static str {
        PANEL_NAME
    }
    fn dump(&self, _cx: &App) -> PanelState {
        let mut state = PanelState::new(PANEL_NAME);
        if let Ok(value) = serde_json::to_value(self.state.clone()) {
            state.info = PanelInfo::panel(value);
        }
        state
    }
}

impl Panel for DummyPanel {
    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.state.name.clone()
    }
}

impl DummyPanel {
    pub fn new(
        name: impl Into<SharedString>,
        content: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            state: DummyPanelState::new(name.into(), content.into()),
            focus: cx.focus_handle(),
        }
    }
    pub fn restore(state: &PanelState, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let state = match &state.info {
                PanelInfo::Panel(value) => {
                    serde_json::from_value(value.to_owned()).unwrap_or_default()
                }
                _ => DummyPanelState::default(),
            };
            Self {
                state,
                focus: cx.focus_handle(),
            }
        })
    }
}

impl Render for DummyPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.state.content.clone())
    }
}
