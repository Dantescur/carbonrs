use anyhow::{Context, Ok, Result};
use syntect::highlighting::{Theme, ThemeSet};

pub fn load_theme(name: &str) -> Result<Theme> {
    let theme_set = ThemeSet::load_defaults();
    theme_set
        .themes
        .get(name)
        .cloned()
        .context(format!("Theme '{}' not founf.", name))
}

// pub fn load_custom_theme(path: &Path) -> Result<Theme> {
//     ThemeSet::get_theme(path).context(format!("Failed to load theme from {:?}", path))
// }

pub fn list_themes() -> Result<()> {
    let theme_set = ThemeSet::load_defaults();
    println!("Themes availables: ");
    for (name, _) in theme_set.themes.iter() {
        println!("{}", name);
    }
    Ok(())
}
