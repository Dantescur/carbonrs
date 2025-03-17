use crate::{
    config::settings::Config,
    models::{CodeSnippet, FontMetrics, ImageDimensions},
};
use anyhow::Result;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};

pub struct ImageBuilder {
    config: Config,
}

impl ImageBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn calculate_dimensions(
        &self,
        snippet: &CodeSnippet,
        metrics: &FontMetrics,
    ) -> ImageDimensions {
        let max_line_chars = snippet
            .highlighted_lines
            .iter()
            .map(|line| line.iter().map(|(_, text)| text.chars().count()).sum())
            .max()
            .unwrap_or(0);

        let windows_control_height = if self.config.layout.window_controls.visible {
            self.config.layout.window_controls.size
        } else {
            0
        };

        let width = self.config.layout.padding_horizontal * 2
            + (max_line_chars as u32 * metrics.char_width.ceil() as u32);
        let height = self.config.layout.padding_vertical * 2
            + windows_control_height
            + (snippet.highlighted_lines.len() as u32 * metrics.line_height as u32);

        ImageDimensions {
            width,
            height,
            text_start: (
                self.config.layout.padding_horizontal as i32,
                (self.config.layout.padding_vertical + windows_control_height) as i32,
            ),
        }
    }

    pub fn render(
        &self,
        snippet: CodeSnippet,
        metrics: FontMetrics,
        dimensions: ImageDimensions,
        output_path: &str,
    ) -> Result<()> {
        let mut img = RgbaImage::new(dimensions.width, dimensions.height);
        self.draw_background(&mut img);
        if self.config.layout.shadow.enabled {
            todo!()
        }

        if self.config.layout.window_controls.visible {
            self.draw_window_controls(&mut img);
        }
        self.draw_code(&mut img, &snippet, &metrics, &dimensions);

        if self.config.layout.watermark {
            todo!()
        }

        img.save(output_path)?;
        Ok(())
    }

    fn draw_background(&self, img: &mut RgbaImage) {
        let bg_color = self.config.theme.theme.settings.background.unwrap();
        let rgba = Rgba([bg_color.r, bg_color.b, bg_color.g, bg_color.a]);
        for pixel in img.pixels_mut() {
            *pixel = rgba;
        }
    }

    fn draw_window_controls(&self, img: &mut RgbaImage) {
        let button_radius = 8;
        let button_spacing = 30;
        let buttons_y = self.config.layout.padding_vertical as i32 + 20;

        // Iterate over the button colors and draw them
        for (i, &color) in self
            .config
            .layout
            .window_controls
            .button_colors
            .iter()
            .enumerate()
        {
            let x = self.config.layout.padding_vertical as i32 + i as i32 * button_spacing;
            draw_filled_circle_mut(img, (x, buttons_y), button_radius, Rgba(color));
        }
    }

    fn draw_code(
        &self,
        img: &mut RgbaImage,
        snippet: &CodeSnippet,
        metrics: &FontMetrics,
        dimensions: &ImageDimensions,
    ) {
        let (start_x, start_y) = dimensions.text_start;
        let line_height = metrics.line_height as i32;

        for (line_num, line) in snippet.highlighted_lines.iter().enumerate() {
            let mut x = start_x;
            let y = start_y + (line_num as i32 * line_height);

            for (style, text) in line {
                let text = text.replace('\t', "    ");
                let text = text.replace("\n", "");
                if text.is_empty() {
                    continue;
                }

                let color = style.foreground;
                draw_text_mut(
                    img,
                    Rgba([color.r, color.g, color.b, 255]),
                    x,
                    y,
                    metrics.scale,
                    &metrics.font,
                    &text,
                );

                x += (text.chars().count() as f32 * metrics.char_width).ceil() as i32;
            }
        }
    }
}
