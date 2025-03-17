use super::manager::FontQuery;
use crate::{config::settings::TypographyConfig, models::FontMetrics};
use ab_glyph::{Font as _, FontArc, PxScale, ScaleFont};
use anyhow::Result;

pub struct FontLoader {
    query: FontQuery,
    embedded_font: Option<Vec<u8>>,
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            query: FontQuery::new(),
            embedded_font: Some(
                include_bytes!("../../assets/FiraCodeNerdFontMono-Regular.ttf").to_vec(),
            ),
        }
    }

    pub fn load_metrics(&mut self, config: &TypographyConfig) -> Result<FontMetrics> {
        let font_data = self
            .query
            .load_font(config.font_family.as_ref())
            .or_else(|_| self.load_system_default())
            .or_else(|_| self.load_embedded())?;
        let font = FontArc::try_from_vec(font_data)?;
        let scale = PxScale::from(config.font_size);
        Ok(FontMetrics {
            line_height: config
                .line_height
                .unwrap_or_else(|| self.calculate_line_height(&font, scale) as f32),
            char_width: self.calculate_char_width(&font, scale),
            scale,
            font,
        })
    }

    fn calculate_line_height(&self, font: &FontArc, scale: PxScale) -> u32 {
        let units_per_em = font.units_per_em().unwrap_or(1000.0);
        let ascent = font.ascent_unscaled() / units_per_em * scale.y;
        let descent = font.descent_unscaled() / units_per_em * scale.y;
        ((ascent - descent) * 1.2).ceil() as u32
    }

    fn calculate_char_width(&self, font: &FontArc, scale: PxScale) -> f32 {
        let glyph_id = font.glyph_id('m');
        let scaled_glyph = font.as_scaled(scale);
        scaled_glyph.h_advance(glyph_id)
    }

    fn load_system_default(&self) -> Result<Vec<u8>> {
        self.query
            .load_font(Some(&"Fira Code".to_string()))
            .or_else(|_| self.query.load_font(Some(&"Cascadia Code".to_string())))
            .or_else(|_| self.query.load_font(Some(&"Consolas".to_string())))
            .or_else(|_| self.query.load_font(Some(&"Menlo".to_string())))
            .or_else(|_| self.query.load_font(Some(&"DejaVu Sans Mono".to_string())))
    }

    fn load_embedded(&mut self) -> Result<Vec<u8>> {
        self.embedded_font
            .take()
            .ok_or_else(|| anyhow::anyhow!("Embedded font already used"))
    }
}
