use std::path::Path;

use crate::{
    config::settings::Config,
    font::loader::FontLoader,
    images::builder::ImageBuilder,
    models::{CodeSnippet, FontMetrics, ImageDimensions},
    syntax::highlighter::SyntaxHighlighter,
};
use anyhow::{Ok, Result};

pub struct CarbonProcessor {
    config: Config,
    syntax_highlighter: SyntaxHighlighter,
    font_loader: FontLoader,
    image_builder: ImageBuilder,
}

impl CarbonProcessor {
    pub fn new(config: Config) -> Result<Self> {
        let theme = config.theme.theme.clone();
        Ok(Self {
            syntax_highlighter: SyntaxHighlighter::new(theme)?,
            font_loader: FontLoader::new(),
            image_builder: ImageBuilder::new(config.clone()),
            config,
        })
    }

    pub fn process(&mut self, input_path: &Path, output_path: &str) -> Result<()> {
        let mut code_snippet = self.load_code(input_path)?;
        self.highlight_code(&mut code_snippet, input_path)?;
        let font_metrics = self.load_font_metrics()?;
        let dimensions = self.calculate_dimensions(&code_snippet, &font_metrics);
        self.render_image(code_snippet, font_metrics, dimensions, output_path)
    }

    fn load_code(&self, path: &Path) -> Result<CodeSnippet> {
        Ok(CodeSnippet {
            content: std::fs::read_to_string(path)?,
            highlighted_lines: Vec::new(),
            // line_numbers_start: self.config.decorations.line_numbers_start,
        })
    }

    fn highlight_code(&self, snippet: &mut CodeSnippet, input_path: &Path) -> Result<()> {
        *snippet = self.syntax_highlighter.highlight(snippet, input_path)?;
        Ok(())
    }

    fn load_font_metrics(&mut self) -> Result<FontMetrics> {
        self.font_loader.load_metrics(&self.config.typography)
    }

    fn calculate_dimensions(
        &self,
        snippet: &CodeSnippet,
        metrics: &FontMetrics,
    ) -> ImageDimensions {
        self.image_builder.calculate_dimensions(snippet, metrics)
    }

    fn render_image(
        &self,
        snippet: CodeSnippet,
        metrics: FontMetrics,
        dimensions: ImageDimensions,
        output_path: &str,
    ) -> Result<()> {
        self.image_builder
            .render(snippet, metrics, dimensions, output_path)
    }
}
