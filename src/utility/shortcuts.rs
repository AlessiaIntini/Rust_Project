use egui::{Key, Modifiers};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

pub fn modifier_to_string(modifier: Modifiers) -> String {
    match modifier {
        Modifiers::COMMAND => {
            if cfg!(target_os = "macos") {
                String::from("Command")
            } else {
                String::from("Ctrl")
            }
        }
        Modifiers::ALT => String::from("Alt"),
        Modifiers::SHIFT => String::from("Shift"),
        _ => String::from(""),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub enum KeyCommand {
    SaveScreenshot,
    QuickSaveScreenshot,
    TakeScreenshot,
    Edit,
    Copy,
    None,
}

impl KeyCommand {
    fn to_string(&self) -> String {
        match self {
            KeyCommand::Edit => String::from("Edit"),
            KeyCommand::SaveScreenshot => String::from("Save as"),
            KeyCommand::TakeScreenshot => String::from("Take Screenshot"),
            KeyCommand::QuickSaveScreenshot => String::from("Quick save"),
            KeyCommand::None => String::from("None"),
            KeyCommand::Copy => String::from("Copy"),
        }
    }
}

impl Display for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: Key,
    pub modifier: Modifiers,
}
impl Shortcut {
    pub fn new(modifier: Modifiers, key: Key) -> Self {
        Self { modifier, key }
    }
}
// implement the trait Display for Key and Modifiers to be able to print them
impl Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {}",
            modifier_to_string(self.modifier),
            self.key.name()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutManager {
    shortcuts: HashMap<KeyCommand, Shortcut>,
    is_shortcut_changing: bool,
    #[serde(skip)]
    pub is_editing_shortcut: bool,
    pub tmp_shortcut: Option<Shortcut>,
    pub tmp_key: Option<Key>,
    pub tmp_modifier: Option<Modifiers>,
    pub tmp_command: Option<KeyCommand>,
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutManager {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            KeyCommand::SaveScreenshot,
            Shortcut::new(Modifiers::COMMAND, Key::A),
        );
        map.insert(KeyCommand::Edit, Shortcut::new(Modifiers::COMMAND, Key::E));
        map.insert(KeyCommand::Copy, Shortcut::new(Modifiers::COMMAND, Key::C));
        map.insert(
            KeyCommand::QuickSaveScreenshot,
            Shortcut::new(Modifiers::ALT, Key::Q),
        );
        map.insert(
            KeyCommand::TakeScreenshot,
            Shortcut::new(Modifiers::ALT, Key::S),
        );
        Self {
            shortcuts: map,
            is_shortcut_changing: false,
            tmp_shortcut: None,
            is_editing_shortcut: false,
            tmp_key: None,
            tmp_modifier: None,
            tmp_command: None,
        }
    }
    pub fn get_shortcuts(&self) -> &HashMap<KeyCommand, Shortcut> {
        &self.shortcuts
    }
    pub fn get_is_shortcut_changing(&self) -> bool {
        self.is_shortcut_changing
    }
    pub fn set_is_shortcut_changing(&mut self, is_shortcut_changing: bool) {
        self.is_shortcut_changing = is_shortcut_changing;
    }
    pub fn update_shortcut(&mut self, command: KeyCommand, shortcut: Shortcut) {
        // if shortcut is already used, do nothing
        if self.shortcuts.values().any(|s| *s == shortcut) {
            return;
        }
        self.shortcuts.insert(command, shortcut);
    }
}
