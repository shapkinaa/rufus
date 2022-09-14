// use std::collections::HashMap;

use toml::Value;

#[derive(Debug, Clone)]
pub enum SortEnum {
    ASC,
    DESC,
    NONE,
}

#[derive(Debug, Clone)]
pub struct TabConfig {
    pub directory_first: bool,
    pub sort_by_name: SortEnum,
    pub sort_by_date: SortEnum,
    pub sort_by_attr: SortEnum,
}

impl Default for TabConfig {
    fn default() -> Self {
        TabConfig {
            directory_first: false,
            sort_by_name: SortEnum::NONE,
            sort_by_date: SortEnum::NONE,
            sort_by_attr: SortEnum::NONE,
        }
    }
}

fn get_bool_by_name(cfg: &Value, section: String, name: String) -> Value {
    if let Some(core) = cfg.get(section) {
        if let Some(param) = core.get(name) {
            return param.clone();
        }
    }
    Value::Boolean(false)
}
fn get_sort_by_name(cfg: &Value, section: String, name: String) -> SortEnum {
    if let Some(core) = cfg.get(section) {
        if let Some(param) = core.get(name) {
            match param.as_str().unwrap_or("none") {
                "asc" => return SortEnum::ASC,
                "desc" => return SortEnum::DESC,
                _ => return SortEnum::NONE,
            }
        }
    }
    return SortEnum::NONE;
}
impl TabConfig {
    pub fn update_from_file(&mut self, cfg: &Value) {
        self.directory_first =
            get_bool_by_name(cfg, String::from("core"), String::from("directory_first"))
                .as_bool()
                .unwrap();
        self.sort_by_name =
            get_sort_by_name(cfg, String::from("core"), String::from("sort_by_name"));
        self.sort_by_date =
            get_sort_by_name(cfg, String::from("core"), String::from("sort_by_date"));
        self.sort_by_attr =
            get_sort_by_name(cfg, String::from("core"), String::from("sort_by_attr"));
    }
}
