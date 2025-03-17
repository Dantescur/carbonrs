use std::path::Path;

use crate::models::CodeSnippet;
use anyhow::{Context, Result};
use syntect::{
    dumps,
    easy::HighlightLines,
    highlighting::{Style, Theme},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl SyntaxHighlighter {
    pub fn new(theme: Theme) -> Result<Self> {
        let syntaxes = dumps::from_uncompressed_data(yazi_prebuild::syntaxes());
        Ok(Self {
            syntax_set: syntaxes.unwrap(),
            theme,
        })
    }

    pub fn highlight(&self, snippet: &CodeSnippet, file_path: &Path) -> Result<CodeSnippet> {
        let syntax = self
            .syntax_set
            .find_syntax_for_file(file_path)
            .with_context(|| format!("Failed to find syntax for {:?}", file_path))?
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme;
        let mut highlighter = HighlightLines::new(syntax, theme);

        let highlighted_lines: Vec<Vec<(Style, String)>> = LinesWithEndings::from(&snippet.content)
            .map(|line| {
                let highlighted = highlighter.highlight_line(line, &self.syntax_set)?;
                Ok(highlighted
                    .into_iter()
                    .map(|(style, text)| (style, text.to_string()))
                    .collect())
            })
            .collect::<Result<_>>()?;

        Ok(CodeSnippet {
            content: snippet.content.clone(),
            highlighted_lines,
            // line_numbers_start: 0,
        })
    }
}
