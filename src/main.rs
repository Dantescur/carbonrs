// src/main.rs
mod cli;
mod config;
mod font;
mod images;
mod models;
mod processor;
mod syntax;

use clap::Parser;
use std::path::Path;
use syntax::theme::list_themes;

use config::{load_config, settings::Config};
use font::FontQuery;

use crate::{cli::args::Cli, processor::CarbonProcessor};
use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut env_logger_build = env_logger::Builder::new();

    if args.verbose.is_present() {
        env_logger_build.filter_level(args.verbose.log_level_filter());
    } else {
        env_logger_build.parse_env("RUST_LOG");
    }

    env_logger_build.init();

    if args.list_fonts {
        let query = FontQuery::new();
        let fonts = query.list_monospaced();
        log::debug!("Listed available fonts: {:?}", fonts);
        println!("Available monospace fonts:\n{}", fonts.join("\n"));
        return Ok(());
    }

    if args.list_themes {
        list_themes()?;
        return Ok(());
    }

    let mut config = load_config().unwrap_or_else(|_| Config::default());

    let input_path = &args.input.expect("Input file required");
    let input = Path::new(input_path);
    let output_path = args.output.unwrap_or_else(|| "output.png".to_string());

    if let Some(font) = args.font {
        config.typography.font_family = Some(font);
    }

    if let Some(theme_name) = args.theme {
        config = config.with_theme(&theme_name)?;
    }

    if args.line_numbers {
        config.decorations.line_numbers = true;
    }

    if args.no_windows {
        config.layout.window_controls.visible = false;
    }

    let mut processor = CarbonProcessor::new(config)?;
    processor.process(input, &output_path)?;
    Ok(())
}
