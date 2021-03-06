//! Text functionality for Piet svg backend

use piet::kurbo::Point;
use piet::{new_error, Error, ErrorKind, HitTestPoint, HitTestTextPosition, LineMetric};

type Result<T> = std::result::Result<T, Error>;

/// SVG text (unimplemented)
pub struct Text(());

impl Text {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Text(())
    }
}

impl piet::Text for Text {
    type Font = Font;
    type FontBuilder = FontBuilder;
    type TextLayout = TextLayout;
    type TextLayoutBuilder = TextLayoutBuilder;

    fn new_font_by_name(&mut self, _name: &str, _size: f64) -> FontBuilder {
        FontBuilder(())
    }

    fn new_text_layout(
        &mut self,
        _font: &Self::Font,
        _text: &str,
        _width: impl Into<Option<f64>>,
    ) -> TextLayoutBuilder {
        TextLayoutBuilder(())
    }
}

/// SVG font builder (unimplemented)
pub struct FontBuilder(());

impl piet::FontBuilder for FontBuilder {
    type Out = Font;

    fn build(self) -> Result<Font> {
        Err(new_error(ErrorKind::NotSupported))
    }
}

/// SVG font (unimplemented)
pub struct Font(());

impl piet::Font for Font {}

pub struct TextLayoutBuilder(());

impl piet::TextLayoutBuilder for TextLayoutBuilder {
    type Out = TextLayout;

    fn build(self) -> Result<TextLayout> {
        Err(new_error(ErrorKind::NotSupported))
    }
}

/// SVG text layout (unimplemented)
#[derive(Clone)]
pub struct TextLayout(());

impl piet::TextLayout for TextLayout {
    fn width(&self) -> f64 {
        unimplemented!()
    }

    #[allow(clippy::unimplemented)]
    fn update_width(&mut self, _new_width: impl Into<Option<f64>>) -> Result<()> {
        unimplemented!();
    }

    #[allow(clippy::unimplemented)]
    fn line_text(&self, _line_number: usize) -> Option<&str> {
        unimplemented!();
    }

    #[allow(clippy::unimplemented)]
    fn line_metric(&self, _line_number: usize) -> Option<LineMetric> {
        unimplemented!();
    }

    #[allow(clippy::unimplemented)]
    fn line_count(&self) -> usize {
        unimplemented!();
    }

    fn hit_test_point(&self, _point: Point) -> HitTestPoint {
        unimplemented!()
    }

    fn hit_test_text_position(&self, _text_position: usize) -> Option<HitTestTextPosition> {
        unimplemented!()
    }
}
