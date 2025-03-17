use ab_glyph::{FontArc, PxScale};
use syntect::highlighting::Style;

#[derive(Debug)]
pub struct CodeSnippet {
    pub content: String,
    pub highlighted_lines: Vec<Vec<(Style, String)>>,
    // pub line_numbers_start: i32,
}

#[derive(Clone)]
// pub struct RenderSettings {
//     pub theme: ThemeConfig,
//     pub layout: LayoutConfig,
//     pub typography: TypographyConfig,
//     pub decorations: DecorationsConfig,
// }

pub struct FontMetrics {
    pub scale: PxScale,
    pub line_height: f32,
    pub char_width: f32,
    pub font: FontArc,
}

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
    pub text_start: (i32, i32),
}
