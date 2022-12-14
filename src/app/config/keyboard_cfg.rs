use crossterm::event::{KeyCode, KeyModifiers};
use toml::Value;

use crate::core::key_binding::KeyBinding;

#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    pub quit: KeyBinding,
    pub focus_left_panel: KeyBinding,
    pub focus_right_panel: KeyBinding,
    pub move_down: KeyBinding,
    pub move_up: KeyBinding,
    pub next_tab: KeyBinding,
    pub prev_tab: KeyBinding,
    pub close: KeyBinding,
    pub open: KeyBinding,
    pub open_as_tab: KeyBinding,
    pub navigate_up: KeyBinding,
    pub delete: KeyBinding,
    pub move_left: KeyBinding,
    pub move_right: KeyBinding,
    pub move_fs_item: KeyBinding,
    pub rename: KeyBinding,
    pub create: KeyBinding,
    pub accept: KeyBinding,
    pub copy_to_left: KeyBinding,
    pub copy_to_right: KeyBinding,
    pub copy_fs_item: KeyBinding,
    pub search_in_panel: KeyBinding,
    pub select_prev: KeyBinding,
    pub select_next: KeyBinding,
    pub change_focus_panels: KeyBinding,

    pub command_1: KeyBinding,
    pub command_2: KeyBinding,
    pub command_3: KeyBinding,
    pub command_4: KeyBinding,
    pub command_5: KeyBinding,
    pub command_6: KeyBinding,
    pub command_7: KeyBinding,
    pub command_8: KeyBinding,
    pub command_9: KeyBinding,

    pub print_test_info: KeyBinding,
    pub filesystem_item_props: KeyBinding,

    pub sort_by_name_asc: KeyBinding,
    pub sort_by_name_desc: KeyBinding,
    pub sort_by_date_asc: KeyBinding,
    pub sort_by_date_desc: KeyBinding,
    pub sort_by_attr_asc: KeyBinding,
    pub sort_by_attr_desc: KeyBinding,
}

impl KeyboardConfig {
    pub fn update_from_file(&mut self, cfg: &Value) {
        if let Some(keyboard_cfg) = cfg.get("keyboard_cfg") {
            if let Value::Table(keyboard_cfg) = keyboard_cfg {
                if let Some(quit) = keyboard_cfg.get("quit") {
                    if let Value::Table(key_binding) = quit {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.quit = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(focus_left_panel) = keyboard_cfg.get("focus_left_panel") {
                    if let Value::Table(key_binding) = focus_left_panel {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.focus_left_panel = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(focus_right_panel) = keyboard_cfg.get("focus_right_panel") {
                    if let Value::Table(key_binding) = focus_right_panel {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.focus_right_panel = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(move_down) = keyboard_cfg.get("move_down") {
                    if let Value::Table(key_binding) = move_down {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.move_down = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(move_up) = keyboard_cfg.get("move_up") {
                    if let Value::Table(key_binding) = move_up {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.move_up = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(next_tab) = keyboard_cfg.get("next_tab") {
                    if let Value::Table(key_binding) = next_tab {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.next_tab = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(prev_tab) = keyboard_cfg.get("prev_tab") {
                    if let Value::Table(key_binding) = prev_tab {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.prev_tab = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(close) = keyboard_cfg.get("close") {
                    if let Value::Table(key_binding) = close {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.close = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(open) = keyboard_cfg.get("open") {
                    if let Value::Table(key_binding) = open {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.open = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(open_as_tab) = keyboard_cfg.get("open_as_tab") {
                    if let Value::Table(key_binding) = open_as_tab {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.open_as_tab = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(navigate_up) = keyboard_cfg.get("navigate_up") {
                    if let Value::Table(key_binding) = navigate_up {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.navigate_up = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(delete) = keyboard_cfg.get("delete") {
                    if let Value::Table(key_binding) = delete {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.delete = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(move_left) = keyboard_cfg.get("move_left") {
                    if let Value::Table(key_binding) = move_left {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.move_left = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(move_right) = keyboard_cfg.get("move_right") {
                    if let Value::Table(key_binding) = move_right {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.move_right = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(move_fs_item) = keyboard_cfg.get("move_fs_item") {
                    if let Value::Table(key_binding) = move_fs_item {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.move_fs_item = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(rename) = keyboard_cfg.get("rename") {
                    if let Value::Table(key_binding) = rename {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.rename = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(create) = keyboard_cfg.get("create") {
                    if let Value::Table(key_binding) = create {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.create = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(accept) = keyboard_cfg.get("accept") {
                    if let Value::Table(key_binding) = accept {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.accept = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(copy_to_left) = keyboard_cfg.get("copy_to_left") {
                    if let Value::Table(key_binding) = copy_to_left {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.copy_to_left = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(copy_to_right) = keyboard_cfg.get("copy_to_right") {
                    if let Value::Table(key_binding) = copy_to_right {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.copy_to_right = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(copy_fs_item) = keyboard_cfg.get("copy_fs_item") {
                    if let Value::Table(key_binding) = copy_fs_item {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.copy_fs_item = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(search_in_panel) = keyboard_cfg.get("search_in_panel") {
                    if let Value::Table(key_binding) = search_in_panel {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.search_in_panel = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(select_prev) = keyboard_cfg.get("select_prev") {
                    if let Value::Table(key_binding) = select_prev {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.select_prev = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(select_next) = keyboard_cfg.get("select_next") {
                    if let Value::Table(key_binding) = select_next {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.select_next = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(change_focus_panels) = keyboard_cfg.get("change_focus_panels") {
                    if let Value::Table(key_binding) = change_focus_panels {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.change_focus_panels = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_1") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_1 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_2") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_2 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_3") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_3 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_4") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_4 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_5") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_5 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_6") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_6 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_7") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_7 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_8") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_8 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("command_9") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.command_9 = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                /*
                pub sort_by_name_asc: KeyBinding,
                pub sort_by_name_desc: KeyBinding,
                pub sort_by_date_asc: KeyBinding,
                pub sort_by_date_desc: KeyBinding,
                pub sort_by_attr_asc: KeyBinding,
                pub sort_by_attr_desc: KeyBinding,
                            */
                if let Some(command_string) = keyboard_cfg.get("sort_by_name_asc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_name_asc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("sort_by_name_desc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_name_desc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(command_string) = keyboard_cfg.get("sort_by_date_asc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_date_asc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("sort_by_date_desc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_date_desc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(command_string) = keyboard_cfg.get("sort_by_attr_asc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_attr_asc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
                if let Some(command_string) = keyboard_cfg.get("sort_by_attr_desc") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.sort_by_attr_desc = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }

                if let Some(command_string) = keyboard_cfg.get("filesystem_item_props") {
                    if let Value::Table(key_binding) = command_string {
                        let key_code = map_key(key_binding["key"].as_str().unwrap());
                        let modifier = if key_binding.contains_key("modifier") {
                            map_modifier(key_binding["modifier"].as_str().unwrap())
                        } else {
                            KeyModifiers::empty()
                        };

                        self.filesystem_item_props = KeyBinding::with_modifiers(key_code, modifier);
                    }
                }
            }
        }
    }
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            quit: KeyBinding::with_modifiers(KeyCode::Char('q'), KeyModifiers::CONTROL),
            focus_left_panel: KeyBinding::new(KeyCode::Char('h')),
            focus_right_panel: KeyBinding::new(KeyCode::Char('l')),
            move_down: KeyBinding::new(KeyCode::Char('j')),
            move_up: KeyBinding::new(KeyCode::Char('k')),
            next_tab: KeyBinding::new(KeyCode::Char('n')),
            prev_tab: KeyBinding::new(KeyCode::Char('p')),
            close: KeyBinding::new(KeyCode::Esc),
            open: KeyBinding::new(KeyCode::Char('o')),
            open_as_tab: KeyBinding::with_modifiers(KeyCode::Char('o'), KeyModifiers::CONTROL),
            navigate_up: KeyBinding::new(KeyCode::Backspace),
            delete: KeyBinding::with_modifiers(KeyCode::Char('d'), KeyModifiers::CONTROL),
            move_left: KeyBinding::with_modifiers(KeyCode::Char('h'), KeyModifiers::CONTROL),
            move_right: KeyBinding::with_modifiers(KeyCode::Char('l'), KeyModifiers::CONTROL),
            move_fs_item: KeyBinding::with_modifiers(KeyCode::Char('m'), KeyModifiers::CONTROL),
            rename: KeyBinding::with_modifiers(KeyCode::Char('r'), KeyModifiers::CONTROL),
            create: KeyBinding::with_modifiers(KeyCode::Char('c'), KeyModifiers::CONTROL),
            accept: KeyBinding::new(KeyCode::Enter),
            copy_to_right: KeyBinding::with_modifiers(KeyCode::Char('x'), KeyModifiers::CONTROL),
            copy_to_left: KeyBinding::with_modifiers(KeyCode::Char('z'), KeyModifiers::CONTROL),
            copy_fs_item: KeyBinding::new(KeyCode::Char('m')),
            search_in_panel: KeyBinding::with_modifiers(KeyCode::Char('s'), KeyModifiers::CONTROL),
            select_prev: KeyBinding::with_modifiers(KeyCode::Char('k'), KeyModifiers::CONTROL),
            select_next: KeyBinding::with_modifiers(KeyCode::Char('j'), KeyModifiers::CONTROL),
            change_focus_panels: KeyBinding::new(KeyCode::Tab),
            command_1: KeyBinding::new(KeyCode::Char('1')),
            command_2: KeyBinding::new(KeyCode::Char('2')),
            command_3: KeyBinding::new(KeyCode::Char('3')),
            command_4: KeyBinding::new(KeyCode::Char('4')),
            command_5: KeyBinding::new(KeyCode::Char('5')),
            command_6: KeyBinding::new(KeyCode::Char('6')),
            command_7: KeyBinding::new(KeyCode::Char('7')),
            command_8: KeyBinding::new(KeyCode::Char('8')),
            command_9: KeyBinding::new(KeyCode::Char('9')),

            sort_by_name_asc: KeyBinding::new(KeyCode::F(1)),
            sort_by_name_desc: KeyBinding::with_modifiers(KeyCode::F(1), KeyModifiers::CONTROL),
            sort_by_date_asc: KeyBinding::new(KeyCode::F(2)),
            sort_by_date_desc: KeyBinding::with_modifiers(KeyCode::F(2), KeyModifiers::CONTROL),
            sort_by_attr_asc: KeyBinding::new(KeyCode::F(3)),
            sort_by_attr_desc: KeyBinding::with_modifiers(KeyCode::F(3), KeyModifiers::CONTROL),

            print_test_info: KeyBinding::with_modifiers(KeyCode::Char('t'), KeyModifiers::CONTROL),

            filesystem_item_props: KeyBinding::new(KeyCode::Char('i')),
        }
    }
}

fn map_key(key: &str) -> KeyCode {
    // comment, because i want capital char would be hot key
    //match key.to_lowercase().as_str() {
    match key {
        "backspace" => KeyCode::Backspace,
        "enter" => KeyCode::Enter,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "page_up" => KeyCode::PageUp,
        "page_down" => KeyCode::PageDown,
        "tab" => KeyCode::Tab,
        "back_tab" => KeyCode::BackTab,
        "delete" => KeyCode::Delete,
        "insert" => KeyCode::Insert,
        "esc" => KeyCode::Esc,
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        n => {
            let mut chars = n.chars();
            KeyCode::Char(chars.next().unwrap())
        }
    }
}

fn map_modifier(modifier: &str) -> KeyModifiers {
    match modifier.to_lowercase().as_str() {
        "c" => KeyModifiers::CONTROL,
        "s" => KeyModifiers::SHIFT,
        "a" => KeyModifiers::ALT,
        _ => KeyModifiers::NONE,
    }
}
