use std::collections::HashMap;
use toml::Value;
use tui::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct ColorScheme {
    pub foreground: Color,
    pub background: Color,

    pub normal_black: Color,
    pub normal_red: Color,
    pub normal_green: Color,
    pub normal_yellow: Color,
    pub normal_blue: Color,
    pub normal_magneta: Color,
    pub normal_cyan: Color,
    pub normal_white: Color,

    pub light_black: Color,
    pub light_red: Color,
    pub light_green: Color,
    pub light_yellow: Color,
    pub light_blue: Color,
    pub light_magneta: Color,
    pub light_cyan: Color,
    pub light_white: Color,

    pub normal_dir_background: Color,
    pub cursor_dir_background: Color,
    pub select_dir_background: Color,
    pub normal_dir_foreground: Color,
    pub cursor_dir_foreground: Color,
    pub select_dir_foreground: Color,

    pub normal_file_background: Color,
    pub cursor_file_background: Color,
    pub select_file_background: Color,
    pub normal_file_foreground: Color,
    pub cursor_file_foreground: Color,
    pub select_file_foreground: Color,

    pub normal_link_background: Color,
    pub cursor_link_background: Color,
    pub select_link_background: Color,
    pub normal_link_foreground: Color,
    pub cursor_link_foreground: Color,
    pub select_link_foreground: Color,
}

impl ColorScheme {
    pub fn new(
        foreground: Color,
        background: Color,
        normal_black: Color,
        normal_red: Color,
        normal_green: Color,
        normal_yellow: Color,
        normal_blue: Color,
        normal_magneta: Color,
        normal_cyan: Color,
        normal_white: Color,
        light_black: Color,
        light_red: Color,
        light_green: Color,
        light_yellow: Color,
        light_blue: Color,
        light_magneta: Color,
        light_cyan: Color,
        light_white: Color,

        normal_dir_background: Color,
        cursor_dir_background: Color,
        select_dir_background: Color,
        normal_dir_foreground: Color,
        cursor_dir_foreground: Color,
        select_dir_foreground: Color,

        normal_file_background: Color,
        cursor_file_background: Color,
        select_file_background: Color,
        normal_file_foreground: Color,
        cursor_file_foreground: Color,
        select_file_foreground: Color,

        normal_link_background: Color,
        cursor_link_background: Color,
        select_link_background: Color,
        normal_link_foreground: Color,
        cursor_link_foreground: Color,
        select_link_foreground: Color,
    ) -> Self {
        Self {
            foreground,
            background,
            normal_black,
            normal_red,
            normal_green,
            normal_yellow,
            normal_blue,
            normal_magneta,
            normal_cyan,
            normal_white,
            light_black,
            light_red,
            light_green,
            light_yellow,
            light_blue,
            light_magneta,
            light_cyan,
            light_white,

            normal_dir_background,
            cursor_dir_background,
            select_dir_background,
            normal_dir_foreground,
            cursor_dir_foreground,
            select_dir_foreground,

            normal_file_background,
            cursor_file_background,
            select_file_background,
            normal_file_foreground,
            cursor_file_foreground,
            select_file_foreground,

            normal_link_background,
            cursor_link_background,
            select_link_background,
            normal_link_foreground,
            cursor_link_foreground,
            select_link_foreground,
        }
    }

    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(foreground) = cfg.get("foreground") {
            self.foreground = map_color(&foreground);
        }

        if let Some(background) = cfg.get("background") {
            self.background = map_color(&background);
        }

        if let Some(normal_black) = cfg.get("normal_black") {
            self.normal_black = map_color(&normal_black);
        }

        if let Some(normal_red) = cfg.get("normal_red") {
            self.normal_red = map_color(&normal_red);
        }

        if let Some(normal_green) = cfg.get("normal_green") {
            self.normal_green = map_color(&normal_green);
        }

        if let Some(normal_yellow) = cfg.get("normal_yellow") {
            self.normal_yellow = map_color(&normal_yellow);
        }

        if let Some(normal_blue) = cfg.get("normal_blue") {
            self.normal_blue = map_color(&normal_blue);
        }

        if let Some(normal_magneta) = cfg.get("normal_magneta") {
            self.normal_magneta = map_color(&normal_magneta);
        }

        if let Some(normal_cyan) = cfg.get("normal_cyan") {
            self.normal_cyan = map_color(&normal_cyan);
        }

        if let Some(normal_white) = cfg.get("normal_white") {
            self.normal_white = map_color(&normal_white);
        }

        if let Some(light_black) = cfg.get("light_black") {
            self.light_black = map_color(&light_black);
        }

        if let Some(light_red) = cfg.get("light_red") {
            self.light_red = map_color(&light_red);
        }

        if let Some(light_green) = cfg.get("light_green") {
            self.light_green = map_color(&light_green);
        }

        if let Some(light_yellow) = cfg.get("light_yellow") {
            self.light_yellow = map_color(&light_yellow);
        }

        if let Some(light_blue) = cfg.get("light_blue") {
            self.light_blue = map_color(&light_blue);
        }

        if let Some(light_magneta) = cfg.get("light_magneta") {
            self.light_magneta = map_color(&light_magneta);
        }

        if let Some(light_cyan) = cfg.get("light_cyan") {
            self.light_cyan = map_color(&light_cyan);
        }

        if let Some(light_white) = cfg.get("light_white") {
            self.light_white = map_color(&light_white);
        }
        if let Some(item) = cfg.get("normal_dir_background") {
            self.normal_dir_background = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_dir_background") {
            self.cursor_dir_background = map_color(&item);
        }
        if let Some(item) = cfg.get("select_dir_background") {
            self.select_dir_background = map_color(&item);
        }
        if let Some(item) = cfg.get("normal_dir_foreground") {
            self.normal_dir_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_dir_foreground") {
            self.cursor_dir_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("select_dir_foreground") {
            self.select_dir_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("normal_file_background") {
            self.normal_file_background = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_file_background") {
            self.cursor_file_background = map_color(&item);
        }
        if let Some(item) = cfg.get("select_file_background") {
            self.select_file_background = map_color(&item);
        }
        if let Some(item) = cfg.get("normal_file_foreground") {
            self.normal_file_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_file_foreground") {
            self.cursor_file_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("select_file_foreground") {
            self.select_file_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("normal_link_background") {
            self.normal_link_background = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_link_background") {
            self.cursor_link_background = map_color(&item);
        }
        if let Some(item) = cfg.get("select_link_background") {
            self.select_link_background = map_color(&item);
        }
        if let Some(item) = cfg.get("normal_link_foreground") {
            self.normal_link_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("cursor_link_foreground") {
            self.cursor_link_foreground = map_color(&item);
        }
        if let Some(item) = cfg.get("select_link_foreground") {
            self.select_link_foreground = map_color(&item);
        }
    }
}

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

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::new(
            Color::White,
            Color::Reset,
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::Gray,
            Color::LightRed,
            Color::LightGreen,
            Color::LightYellow,
            Color::LightBlue,
            Color::LightMagenta,
            Color::LightCyan,
            Color::White,
            Color::Black,
            Color::Gray,
            Color::Green,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Gray,
            Color::Green,
            Color::Gray,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Gray,
            Color::Green,
            Color::Gray,
            Color::White,
            Color::Black,
        )
    }
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
        if let Value::Table(values) = cfg {
            for (key, value) in values.iter() {
                self.colors_files.insert(
                    key.clone(),
                    map_color(&Value::String(value.as_str().unwrap().to_string())),
                );
            }
        }
    }
}
fn get_default_colors_files() -> HashMap<String, Color> {
    let mut colors_files = HashMap::new();
    colors_files.insert("default".to_string(), Color::White);
    colors_files
}
