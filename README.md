<div align="center" style="border: 2px solid #ffcc00; padding: 16px; border-radius: 8px; background-color: #fffbe6; margin-bottom: 24px;">

### 🚨 **Important Notice: Pull Requests Welcome, But Please Open an Issue First!** 🚨

We're currently planning a **major refactoring and modularization effort** for Carbonrs. While we greatly appreciate pull requests, please:

1. **Open an issue first** to discuss your proposed changes.
2. Wait for confirmation before starting work on larger features.

This will help us avoid conflicts with ongoing architectural changes and ensure your contributions align with our new structure. Thank you for your understanding and support! 🙏

</div>

---

<div align="center">

# Carbonrs 🖼️

**A CLI tool for generating beautiful, customizable code snippet images** - right from your terminal. Inspired by [carbon.now.sh](https://carbon.now.sh), Carbonrs lets you create stunning visuals of your code with syntax highlighting, customizable fonts, themes, sizes, and more.

</div>

---

## Overview

Carbonrs is written in Rust and leverages powerful libraries:

- **`syntect`**: For syntax highlighting
- **`image`/`imageproc`**: For drawing
- **`ab_glyph`**: For font rendering

The goal is to provide a **fast, lightweight**, and fully customizable CLI experience that replicates and extends the functionality of [carbon.now.sh](https://carbon.now.sh).

---

## Features

- **Command-Line Interface**: Generate images directly from your terminal.
- **Syntax Highlighting**: Uses `syntect` for accurate and beautiful code highlighting.
- **Customizable Appearance**:
  - Change fonts (with support for custom fonts).
  - Switch between themes.
  - Adjust font sizes and image dimensions.
- **Simulated Window Controls**: Optionally render macOS-style window buttons (red, yellow, green).
- **Fast & Lightweight**: Built in Rust for optimal performance.

---

## Installation

### Prerequisites

- Ensure you have [Rust](https://rustup.rs) installed.

### Install via Cargo

```bash
cargo install --git https://github.com/dantescur/carbonrs
```

### Build from Source

```bash
git clone https://github.com/dantescur/carbonrs.git
cd carbonrs
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

---

## Usage

Carbonrs is designed to be simple and straightforward. At its core, it converts a source code file into an image with highlighted syntax.

### Basic Command

```bash
carbonrs path/to/your/code_file --output path/to/output_image.png
```

If you do not specify an output file, it defaults to `output.png`.

### Example

To generate an image from `src/main.rs` and save it as `my_code.png`:

```bash
carbonrs src/main.rs --output my_code.png
```

---

## Customization

Carbonrs aims to offer extensive customization options, both now and in future releases:

### Fonts

- **Current**: Embedded Fira Code Nerd Font Mono.
- **Planned**: Support for custom fonts.

### Themes

- **Default**: `base16-ocean.dark`.
- **Planned**: Additional built-in themes and support for user-defined themes.

### Sizes & Layout

- Customize font size, line height, and image dimensions.

### Window Controls

- Optional macOS-style window controls (red, yellow, green buttons).
- Planned customization options for these controls.

> **Note:** Many of these options will be made available through CLI flags or a configuration file in upcoming releases.

---

## Roadmap

- [x] Basic CLI functionality for generating code snippet images.
- [ ] Support for dynamic font selection.
- [ ] Additional themes and support for user-defined themes.
- [ ] Enhanced customization for background colors, window controls, and overall layout.
- [ ] Better handling of diverse file types and languages.
- [ ] Integration with configuration files for persistent settings.

---

## Contributing

Contributions are very welcome! If you have suggestions, bug reports, or would like to add new features, please:

1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request.

For major changes, please **open an issue first** to discuss your ideas.

---

## License

Carbonrs is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- [carbon.now.sh](https://carbon.now.sh) for the inspiration.
- The Rust community and the developers of the libraries used in this project.
- Fira Code font authors for the embedded font.

---

<div align="center">

**Happy coding!** 🦀 Create stunning code visuals with Carbonrs!

</div>
