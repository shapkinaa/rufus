use std::collections::HashMap;
use toml::Value;
use tui::style::Color;

fn map_color(value: &Value) -> Color {
    match value {
        Value::String(s) => match s.as_str() {
            "Reset" => Color::Reset,
            "Black" => Color::Black,
            "Red" => Color::Red,
            "Green" => Color::Green,
            "Yellow" => Color::Yellow,
            "Blue" => Color::Blue,
            "Magenta" => Color::Magenta,
            "Cyan" => Color::Cyan,
            "Gray" => Color::Gray,
            "DarkGray" => Color::DarkGray,
            "LightRed" => Color::LightRed,
            "LightGreen" => Color::LightGreen,
            "LightYellow" => Color::LightYellow,
            "LightBlue" => Color::LightBlue,
            "LightMagenta" => Color::LightMagenta,
            "LightCyan" => Color::LightCyan,
            "White" => Color::White,
            _ => Color::Reset,
        },
        Value::Integer(i) => Color::Indexed(i.clone() as u8),
        Value::Table(t) => {
            let red = t["red"].as_integer().unwrap().clone() as u8;
            let green = t["green"].as_integer().unwrap().clone() as u8;
            let blue = t["blue"].as_integer().unwrap().clone() as u8;
            Color::Rgb(red, green, blue)
        }
        _ => Color::Reset,
    }
}

fn get_default_colors_files() -> HashMap<String, Color> {
    let mut colors_files = HashMap::new();
    colors_files.insert("default".to_string(), Color::White);
    colors_files
}

#[derive(Clone, Debug)]
pub struct ColorsFiles {
    pub colors_files: HashMap<String, Color>,
}

impl Default for ColorsFiles {
    fn default() -> Self {
        ColorsFiles {
            colors_files: get_default_colors_files(),
        }
    }
}
impl ColorsFiles {
    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(colors_files) = cfg.get("colors_files") {
            if let Value::Table(values) = colors_files {
                for (key, value) in values.iter() {
                    self.colors_files.insert(
                        key.clone(),
                        map_color(&Value::String(value.as_str().unwrap().to_string())),
                    );
                }
            }
        }
    }
}
