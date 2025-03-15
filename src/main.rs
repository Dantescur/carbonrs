use ab_glyph::{Font, ScaleFont};
use ab_glyph::{FontRef, PxScale};
use anyhow::Result;
use clap::Parser;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input code file path
    input: String,

    /// Output image path (default: output.png)
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output_path = args.output.unwrap_or_else(|| "output.png".to_string());

    // Read code file
    let code = std::fs::read_to_string(&args.input)?;

    // Load syntax set and theme
    // TODO: add more syntax (bat repo)
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_for_file(&args.input)?
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    // TODO: Dynamic theme
    let theme = &ts.themes["base16-ocean.dark"];

    // Load embedded monospace font (Fira Mono)
    // TODO: Dynamic font
    let font_data = include_bytes!("../assets/FiraCodeNerdFontMono-Regular.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Error loading font");

    // Highlight code
    let mut highlighter = HighlightLines::new(syntax, theme);
    let highlighted_lines: Vec<Vec<(Style, &str)>> = LinesWithEndings::from(&code)
        .map(|line| highlighter.highlight_line(line, &ps))
        .collect::<Result<_, _>>()?;

    // Font metrics
    // TODO: Dynamic sizes
    let font_size = 24.0;
    let scale = PxScale::from(font_size);
    // let scaled_font = font.as_scaled(scale);
    let line_height = {
        let line_heigh_unscaled = font.height_unscaled();
        (line_heigh_unscaled / font.units_per_em().unwrap_or(1.0) * scale.y).ceil() as u32
    };
    let char_width = {
        let glyph_id = font.glyph_id('m');
        let scaled_glyph = font.as_scaled(scale);
        scaled_glyph.h_advance(glyph_id)
    };

    // Calculate image dimensions
    let padding = 40;
    let window_controls_height = 60;
    let max_line_chars = highlighted_lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|(_, text)| text.chars().count())
                .sum::<usize>()
        })
        .max()
        .unwrap_or(0);
    let image_width = padding * 2 + (max_line_chars as u32 * char_width.ceil() as u32);
    let image_height =
        padding * 2 + window_controls_height + (highlighted_lines.len() as u32 * line_height);

    // Create image buffer
    let mut img = RgbaImage::new(image_width, image_height);

    // Draw background (Carbon-like dark gray)
    // TODO: Dynamic colors
    let bg_color = Rgba([40, 44, 53, 255]);
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }

    // Draw window controls (red, yellow, green)
    // TODO: Support more windows?
    let button_colors = [
        Rgba([255, 95, 86, 255]),
        Rgba([255, 189, 46, 255]),
        Rgba([40, 201, 64, 255]),
    ];
    let button_radius = 8;
    let button_spacing = 30;
    let buttons_y = padding as i32 + 20;
    for (i, &color) in button_colors.iter().enumerate() {
        let x = padding as i32 + i as i32 * button_spacing;
        draw_filled_circle_mut(&mut img, (x, buttons_y), button_radius, color);
    }

    // Render highlighted code
    // FIXME: remove space char after line endings
    let text_start_x = padding as i32;
    let text_start_y = padding as i32 + window_controls_height as i32;
    for (line_num, line) in highlighted_lines.iter().enumerate() {
        let mut x = text_start_x;
        let assent_px = (font.ascent_unscaled() / font.units_per_em().unwrap_or(1.0)) * scale.y;
        let y = text_start_y + (line_num as i32 * line_height as i32) + assent_px as i32;

        for (style, text) in line {
            let text = text.replace('\t', "    ");
            if text.is_empty() {
                continue;
            }

            // Convert syntect color to Rgba
            let color = style.foreground;
            let rgba = Rgba([color.r, color.g, color.b, 255]);

            // Draw text using ab_glyph compatible font
            draw_text_mut(&mut img, rgba, x, y, scale, &font, &text);

            // Update x position for next token
            x += (text.chars().count() as f32 * char_width).ceil() as i32;
        }
    }

    // Save image
    img.save(output_path)?;

    Ok(())
}
