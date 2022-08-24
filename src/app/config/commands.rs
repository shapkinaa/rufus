use std::collections::HashMap;

use toml::Value;

#[derive(Debug, Clone)]
pub struct HotkeyCommandsPrograms {
    bindings: HashMap<String, String>,
}

impl Default for HotkeyCommandsPrograms {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        // bindings.insert("default".to_string(), "nvim".to_string());

        HotkeyCommandsPrograms { bindings }
    }
}

impl HotkeyCommandsPrograms {
    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(hotkey_commands_programs) = cfg.get("hotkey_commands_programs") {
            if let Value::Table(associated_programs_map) = hotkey_commands_programs {
                for (key, val) in associated_programs_map.iter() {
                    self.bindings
                        .insert(key.clone(), val.as_str().unwrap().to_string());
                }
            }
        }
    }
    pub fn get_path(&self, command_name: String) -> String {
        //println!("{:?}", command_name.clone());
        //        println!("{:?}", self.bindings.get("command_1"));
        //      String::from("")
        match self.bindings.get(&command_name) {
            Some(name) => name.clone(),
            None => String::from(""),
            //self.bindings[&"default".to_string()].clone(),
        }
    }
}
