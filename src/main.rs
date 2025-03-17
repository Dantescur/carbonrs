use std::fs;
use std::path::PathBuf;

// Import necessary crates and modules used for font handling, image manipulation,
// command line argument parsing, and syntax highlighting.
use ab_glyph::{Font, ScaleFont}; // For font scaling and rendering text
use ab_glyph::{FontRef, PxScale}; // For referencing font data and pixel scaling
use anyhow::Result; // For simplified error handling
use clap::Parser;
use dirs::home_dir;
// For command-line argument parsing
use image::Rgba;
use image::RgbaImage;
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};
use serde::Deserialize;
use syntect::highlighting::Theme;
// For drawing shapes and text on images
use syntect::{
    easy::HighlightLines,            // For line-by-line syntax highlighting
    highlighting::{Style, ThemeSet}, // For text style and theme management
    parsing::SyntaxSet,              // For managing syntax definitions
    util::LinesWithEndings,          // For iterating over code lines with line endings preserved
};

/// Struct to define and parse command-line arguments using clap.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input code file
    input: String,

    /// Path to the output image file (default: output.png)
    output: Option<String>,

    /// An user selected theme
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Add line numbers
    #[arg(short, long, default_value_t = false)]
    line_numbers: bool,
}

#[derive(Deserialize, Debug, Default)]
struct Config {
    line_numbers: Option<bool>,
    font_size: Option<f32>,
    padding: Option<u32>,
    theme: Option<String>,
}

fn load_config() -> Config {
    let config_path = home_dir()
        .map(|mut path| {
            path.push(".config");
            path.push("carbonrs");
            path.push("config.toml");
            path
        })
        .unwrap_or_else(|| PathBuf::from("config.toml"));

    if config_path.exists() {
        let config_str = fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&config_str).unwrap_or_default()
    } else {
        Config::default()
    }
}

#[derive(Debug)]
struct MergedConfig {
    line_numbers: bool,
    font_size: f32,
    padding: u32,
    theme: String,
}

impl MergedConfig {
    fn merge(args: &Args, config: &Config) -> Self {
        MergedConfig {
            line_numbers: if args.line_numbers {
                true
            } else {
                config.line_numbers.unwrap_or(false)
            },
            font_size: config.font_size.unwrap_or(24.0),
            padding: config.padding.unwrap_or(40),
            theme: config.theme.unwrap() || "base16-mocha.dark".to_string(),
        }
    }
}
fn main() -> Result<()> {
    // Parse command-line arguments (input file and optional output file)
    let args = Args::parse();

    // Load clnfiguration from file
    let config = load_config();

    // Merge cli args with the config file
    let merged_config = MergedConfig::merge(&args, &config);
    // Use the provided output path or default to "output.png"
    let output_path = args.output.unwrap_or_else(|| "output.png".to_string());

    // Read the entire content of the input code file into a string.
    let code = std::fs::read_to_string(&args.input)?;

    // Load default syntax definitions (including newline handling) for highlighting.
    // These definitions are used to identify language-specific syntax.
    let ps = SyntaxSet::load_defaults_newlines();
    // Load the default set of themes.
    let ts = ThemeSet::load_defaults();

    // Determine the syntax to use based on the file extension of the input file.
    // If no matching syntax is found, fall back to plain text.
    let syntax = ps
        .find_syntax_for_file(&args.input)?
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    // Choose a fixed theme for highlighting.
    // TODO: Allow users to select or dynamically choose a theme.
    let theme = &ts.themes["base16-mocha.dark"];

    // Load embedded monospace font (Fira Code Nerd Font Mono) from the assets directory.
    // This ensures consistent rendering of code characters.
    // TODO: Allow dynamic font selection.
    let font_data = include_bytes!("../assets/FiraCodeNerdFontMono-Regular.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Error loading font");

    // Initialize the syntax highlighter with the selected syntax and theme.
    // The highlighter processes the input code line by line.
    let mut highlighter = HighlightLines::new(syntax, theme);
    // Iterate over each line in the code, apply highlighting, and collect the results.
    // Each line becomes a vector of tuples where each tuple contains a style and text segment.
    let highlighted_lines: Vec<Vec<(Style, &str)>> = LinesWithEndings::from(&code)
        .map(|line| highlighter.highlight_line(line, &ps))
        .collect::<Result<_, _>>()?;

    // Define the font size to use for rendering the text.
    // Calculate the scaling factor based on the desired font size.
    let scale = PxScale::from(merged_config.font_size);
    // Compute the line height using the font's unscaled height and converting it with the scale factor.
    let line_height = {
        let line_heigh_unscaled = font.height_unscaled();
        (line_heigh_unscaled / font.units_per_em().unwrap_or(1.0) * scale.y).ceil() as u32
    };
    // Determine the approximate width of a character by measuring the glyph for 'm'.
    let char_width = {
        let glyph_id = font.glyph_id('m');
        let scaled_glyph = font.as_scaled(scale);
        scaled_glyph.h_advance(glyph_id)
    };

    // Calculate dimensions for the output image:
    // - Define padding around the content.
    // - Reserve extra space at the top for window control simulation.
    let padding = 40;
    // Determine the maximum number of characters in any line (this affects image width).
    let max_line_chars = highlighted_lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|(_, text)| text.chars().count())
                .sum::<usize>()
        })
        .max()
        .unwrap_or(0);
    // Calculate the image width by accounting for horizontal padding and text width.
    let image_width =
        merged_config.padding * 2 + (max_line_chars as u32 * char_width.ceil() as u32);
    // Calculate the image height by adding vertical padding, the height for window controls,
    // and the total height needed for all lines of code.
    let image_height =
        merged_config.padding * 2 + 60 + (highlighted_lines.len() as u32 * line_height);

    let _ = build_image(
        image_width,
        image_height,
        padding,
        60,
        scale,
        font,
        line_height,
        highlighted_lines,
        char_width,
        output_path,
        theme,
        merged_config.line_numbers,
    );

    // Return Ok(()) to indicate successful execution.
    Ok(())
}

fn build_image(
    width: u32,
    height: u32,
    padding: u32,
    win_height: u32,
    scale: PxScale,
    font: FontRef,
    line_height: u32,
    highlighted_lines: Vec<Vec<(Style, &str)>>,
    char_width: f32,
    output_path: String,
    theme: &Theme,
    line_numbers: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new RGBA image buffer with the computed dimensions.
    let mut img = RgbaImage::new(width, height);

    // Fill the entire image with the theme's background color.
    let bg_color = Rgba([
        theme.settings.background.unwrap().r,
        theme.settings.background.unwrap().g,
        theme.settings.background.unwrap().b,
        theme.settings.background.unwrap().a,
    ]);
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }

    // Define colors for simulated window control buttons (red, yellow, green).
    let button_colors = [
        Rgba([255, 95, 86, 255]),  // Red
        Rgba([255, 189, 46, 255]), // Yellow
        Rgba([40, 201, 64, 255]),  // Green
    ];
    // Set the visual properties of the buttons: their radius and spacing.
    let button_radius = 8;
    let button_spacing = 30;
    // Calculate the vertical position for the buttons relative to the padding.
    let buttons_y = padding as i32 + 20;
    // Draw each button as a filled circle on the image.
    for (i, &color) in button_colors.iter().enumerate() {
        let x = padding as i32 + i as i32 * button_spacing;
        draw_filled_circle_mut(&mut img, (x, buttons_y), button_radius, color);
    }

    // Set starting positions for the code text on the image.
    let mut text_start_x = padding as i32;
    let text_start_y = padding as i32 + win_height as i32;

    // If line numbers are enabled, calculate the width required for the line numbers.
    let line_number_width = if line_numbers {
        let max_line_number = highlighted_lines.len();
        let max_digits = max_line_number.to_string().len();
        (max_digits as f32 * char_width).ceil() as i32 + 20 // Add some padding
    } else {
        0
    };

    // Adjust the text starting position to the right to make space for line numbers.
    text_start_x += line_number_width;

    // Render each highlighted line of code.
    for (line_num, line) in highlighted_lines.iter().enumerate() {
        // Reset x position to the left margin for each new line.
        let mut x = text_start_x;
        // Calculate y-coordinate for the current line.
        let assent_px = (font.ascent_unscaled() / font.units_per_em().unwrap_or(1.0)) * scale.y;
        let y = text_start_y + (line_num as i32 * line_height as i32) + assent_px as i32;

        // If line numbers are enabled, render the line number.
        if line_numbers {
            let line_number = (line_num + 1).to_string();
            let line_number_color = Rgba([128, 128, 128, 255]); // Gray color for line numbers
            draw_text_mut(
                &mut img,
                line_number_color,
                padding as i32,
                y,
                scale,
                &font,
                &line_number,
            );
        }

        // Process each token (styled segment) in the current line.
        for (style, text) in line {
            // Replace tab characters with four spaces for uniform spacing.
            let text = text.replace('\t', "    ");
            let text = text.replace("\n", " ");
            if text.is_empty() {
                continue;
            }

            // Convert the syntect color (used for syntax highlighting) to an RGBA color.
            let color = style.foreground;
            let rgba = Rgba([color.r, color.g, color.b, color.a]);

            // Render the text token on the image using the specified font, color, and scale.
            draw_text_mut(&mut img, rgba, x, y, scale, &font, &text);

            // Update the x-coordinate by advancing it by the width of the rendered token.
            x += (text.chars().count() as f32 * char_width).ceil() as i32;
        }
    }

    // Save the final image to the specified output path.
    img.save(output_path)?;
    Ok(())
}
