use crate::prelude::*;
use gpui_kit::{
    FillOptions, PathBuilder, PathStyle, Pixels, Point, Task,
    base::{Disableable, input::InputState},
    canvas,
    component::{alert::Alert, input::Input},
    point, px,
};
use harfrust::{
    Script,
    font::glyph::outline::{DrawMemory, DrawOptions, PathSink},
};
use std::{path::PathBuf, time::Duration};

pub struct Preview {
    pub root: PathBuf,
    status: PreviewStatus,
    text_state: Entity<InputState>,
    focus: FocusHandle,
    task: Option<Task<()>>,
}

enum PreviewStatus {
    Compiling,
    Ready(Vec<PreviewGlyph>),
    Error(SharedString),
}

impl PreviewStatus {
    fn glyphs(&self) -> Option<Vec<PreviewGlyph>> {
        match self {
            Self::Ready(glyphs) => Some(glyphs.clone()),
            _ => None,
        }
    }
    pub fn is_compiling(&self) -> bool {
        matches!(&self, &Self::Compiling)
    }
    pub fn error(&self) -> Option<SharedString> {
        match self {
            Self::Error(err) => Some(err.clone()),
            _ => None,
        }
    }
}

impl Preview {
    pub fn new(root: PathBuf, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut preview = Self {
            root,
            text_state: cx.new(|cx| InputState::new(window, cx)),
            focus: cx.focus_handle(),
            task: None,
            status: PreviewStatus::Compiling,
        };
        preview.refresh(cx);
        preview
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let root = self.root.clone();
        let text = self.text_state.read(cx).value().to_string();
        self.task = Some(cx.spawn(async move |this, cx| {
            _ = this.update(cx, |this, cx| {
                this.status = PreviewStatus::Compiling;
                cx.notify();
            });
            cx.background_executor()
                .timer(Duration::from_millis(500))
                .await;
            let result = cx
                .background_executor()
                .spawn(async move {
                    let input = fontc::Input::new(&root)?;
                    let font_source = input.create_source()?;
                    let options = fontc::Options::default();
                    fontc::generate_font(font_source, options)
                })
                .await;
            _ = this.update(cx, |this, cx| match result {
                Ok(bytes) => {
                    let font = harfrust::Font::new(bytes, 0).unwrap();
                    let shaper_font = harfrust::ShaperFont::new(&font);
                    let mut buffer = harfrust::Buffer::new();
                    buffer.set_script(Some(Script::ARABIC));
                    buffer.set_language(harfrust::Language::new("ar"));
                    buffer.set_direction(harfrust::Direction::RightToLeft);
                    let upem = font.units_per_em() as f32;
                    buffer.push_str(&text);
                    match harfrust::shape(
                        &shaper_font,
                        &mut buffer,
                        harfrust::ShapeOptions::default(),
                    ) {
                        Ok(_) => {
                            let mut glyphs = Vec::new();
                            let outlines = font.outline_glyphs();
                            let mut x = 0.0;
                            let mut y = 0.0;
                            for (info, position) in buffer
                                .glyph_infos()
                                .into_iter()
                                .zip(buffer.glyph_positions())
                            {
                                let id = harfrust::GlyphId::new(info.glyph_id);
                                let Some(outline) = outlines.get(id) else {
                                    continue;
                                };
                                let mut sink = PreviewGlyphSink::new(upem);
                                _ = outline.draw(
                                    &DrawOptions::default(),
                                    DrawMemory::Scratch,
                                    &mut sink,
                                );
                                tracing::info!(?position);
                                glyphs.push(PreviewGlyph {
                                    path: sink,
                                    x: x + (position.x_offset as f32 / upem),
                                    y: y - position.y_offset as f32 / upem,
                                });
                                x += position.x_advance as f32 / upem;
                                y -= position.y_advance as f32 / upem;
                            }
                            this.status = PreviewStatus::Ready(glyphs);
                            cx.notify();
                        }
                        Err(err) => {
                            this.status = PreviewStatus::Error(err.to_string().into());
                            cx.notify();
                        }
                    }
                }
                Err(err) => {
                    tracing::error!(%err);
                    this.status = PreviewStatus::Error(err.to_string().into());
                    cx.notify();
                }
            });
        }));
    }
}

#[derive(Clone)]
struct PreviewGlyph {
    path: PreviewGlyphSink,
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct PreviewGlyphSink(pub(crate) Vec<PreviewPathStep>, f32);

#[derive(Clone)]
enum PreviewPathStep {
    MoveTo(Point<Pixels>),
    LineTo(Point<Pixels>),
    QuadTo(Point<Pixels>, Point<Pixels>),
    CurveTo(Point<Pixels>, Point<Pixels>, Point<Pixels>),
    Close,
}

impl PathSink for PreviewGlyphSink {
    fn move_to(&mut self, x: f32, y: f32) {
        let x = x / self.1;
        let y = 1.0 - y / self.1;
        self.0.push(PreviewPathStep::MoveTo(point(px(x), px(y))));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let x = x / self.1;
        let y = 1.0 - y / self.1;
        self.0.push(PreviewPathStep::LineTo(point(px(x), px(y))));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let x = x / self.1;
        let y = 1.0 - y / self.1;
        let to = point(px(x), px(y));
        let cx0 = cx0 / self.1;
        let cy0 = 1.0 - cy0 / self.1;
        let c0 = point(px(cx0), px(cy0));
        self.0.push(PreviewPathStep::QuadTo(to, c0));
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let x = x / self.1;
        let y = 1.0 - y / self.1;
        let to = point(px(x), px(y));
        let cx0 = cx0 / self.1;
        let cy0 = 1.0 - cy0 / self.1;
        let c0 = point(px(cx0), px(cy0));
        let cx1 = cx1 / self.1;
        let cy1 = 1.0 - cy1 / self.1;
        let c1 = point(px(cx1), px(cy1));
        self.0.push(PreviewPathStep::CurveTo(to, c0, c1));
    }

    fn close(&mut self) {
        self.0.push(PreviewPathStep::Close);
    }
}

impl PreviewGlyphSink {
    pub fn new(upem: f32) -> Self {
        Self(Vec::new(), upem)
    }
    fn into_path_builder(self) -> PathBuilder {
        let mut builder =
            PathBuilder::default().with_style(PathStyle::Fill(FillOptions::non_zero()));
        for step in self.0 {
            match step {
                PreviewPathStep::MoveTo(to) => builder.move_to(to),
                PreviewPathStep::LineTo(to) => builder.line_to(to),
                PreviewPathStep::QuadTo(to, c0) => builder.curve_to(to, c0),
                PreviewPathStep::CurveTo(to, c0, c1) => builder.cubic_bezier_to(to, c0, c1),
                PreviewPathStep::Close => builder.close(),
            }
        }
        builder
    }
}

impl EventEmitter<PanelEvent> for Preview {}

impl Focusable for Preview {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl BasePanel for Preview {
    fn panel_name(&self) -> &'static str {
        "preview"
    }
    fn closable(&self, _cx: &App) -> bool {
        false
    }
    fn zoomable(&self, _cx: &App) -> bool {
        true
    }
}

impl Panel for Preview {
    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        "Preview"
    }
    fn zoom_control(&self, _cx: &App) -> Option<gpui_kit::component::dock::PanelControl> {
        Some(gpui_kit::component::dock::PanelControl::Toolbar)
    }
    fn title_suffix(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        Some(
            Button::new("reload-preview")
                .icon(IconName::RefreshCw)
                .xsmall()
                .ghost()
                .disabled(self.status.is_compiling())
                .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
        )
    }
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_4()
            .p_4()
            .when_some(self.status.error(), |this, error| {
                this.child(Alert::error("preview-error", error).rounded_none())
            })
            .when_some(self.status.glyphs(), |this, glyphs| {
                this.child(
                    canvas(
                        |bounds, _, _| bounds,
                        |_, bounds, window, cx| {
                            for g in glyphs {
                                let mut path = g.path.into_path_builder();
                                path.translate(point(px(g.x), px(g.y)));
                                path.scale(48.0);
                                path.translate(point(bounds.left(), bounds.top()));
                                if let Ok(path) = path.build() {
                                    window.paint_path(path, cx.theme().foreground);
                                }
                            }
                        },
                    )
                    .size_full(),
                )
            })
            .child(Input::new(&self.text_state).h_24())
    }
}
