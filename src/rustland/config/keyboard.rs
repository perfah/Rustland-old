use std::collections::HashMap;
use std::iter::Peekable;

use serde::ser::Serialize;
use serde::de::Deserialize;

use wlc::input::keyboard::Key;
use wlc::Modifier;
use wlc::Modifier::Flags;

use toml;
use toml::de::Deserializer;

static KEY_DIVISOR: char = '-';

#[derive(Serialize, Deserialize, PartialEq)]
enum KeySequence {
    Mod(Modifier::Flags),
    RegularKey(Key),
    Combo(Box<KeySequence>, Box<KeySequence>),
    End,
    Unrecognized
}

impl KeySequence {
    pub fn new(src: String) -> KeySequence {
        if !src.contains(KEY_DIVISOR) {
            #[derive(Serialize, Deserialize)]
            struct KeyHolder { value: Key }
            
            match toml::from_str::<KeyHolder>(&format!(r#"value = "{}""#, src)) {
                Ok(holder) => KeySequence::RegularKey(holder.value),
                Err(msg) => match src.as_ref(){
                    "Alt" => KeySequence::Mod(Modifier::Alt),
                    "Caps" => KeySequence::Mod(Modifier::Caps),
                    "Ctrl" => KeySequence::Mod(Modifier::Ctrl),
                    "Logo" => KeySequence::Mod(Modifier::Logo),
                    "Mod2" => KeySequence::Mod(Modifier::Mod2),
                    "Mod3" => KeySequence::Mod(Modifier::Mod3),
                    "Mod5" => KeySequence::Mod(Modifier::Mod5),
                    "Shift" => KeySequence::Mod(Modifier::Shift),
                    _ => KeySequence::Unrecognized
                }            
            }
        }
        else {
            let mut iter = src.split(KEY_DIVISOR).peekable();
            Self::construct_tail(iter)
        }
    }

    fn construct_tail<'a, I>(mut iter: Peekable<I>) -> KeySequence where I: Iterator<Item = &'a str>{
        KeySequence::Combo (
            box match iter.next() {
                Some(next) => KeySequence::new(next.to_string().clone()),
                None => KeySequence::End
            },
            box match iter.peek() {
                Some(_) => Self::construct_tail(iter),
                None => KeySequence::End
            }
        )
    }

    pub fn new_combo() -> KeySequence {
        KeySequence::Combo(
            box KeySequence::End,
            box KeySequence::End
        )
    }

    pub fn matches(&self, mods: Flags, key: Key) -> bool {
        match self {
            &KeySequence::Mod(m) => mods.contains(m),
            &KeySequence::RegularKey(k) => key == k,
            &KeySequence::Combo(ref former, ref latter) => 
                former.matches(mods, key) && 
                latter.matches(mods, key),
            &KeySequence::End => true,
            &KeySequence::Unrecognized => false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyboardConfig {
    pub mod_key: String,
    pub meta_view_key: Key,
    pub hotkeys: HashMap<String, String>
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            mod_key: "Logo".to_string(),
            meta_view_key: Key::Tab,
            hotkeys: [
                (format!("Space{}Ctrl", KEY_DIVISOR), "/usr/bin/dmenu_run".to_string()),
                ("T".to_string(), "/usr/bin/terminator".to_string()),
            ].iter().cloned().collect()
        }
    }
}

impl KeyboardConfig {
    pub fn modifier(&self) -> Modifier::Flags {
        match self.mod_key.as_ref() {
            "Alt" => Modifier::Alt,
            "Caps" => Modifier::Caps,
            "Ctrl" => Modifier::Ctrl,
            "Logo" => Modifier::Logo,
            "Mod2" => Modifier::Mod2,
            "Mod3" => Modifier::Mod3,
            "Mod5" => Modifier::Mod5,
            "Shift" => Modifier::Shift,
            _ => Modifier::Flags::empty()
        }
    }


    pub fn mod_key_is_pressed(&self, mods: Flags) -> bool {
        mods.contains(self.modifier())
    }

    pub fn matching_hotkey(&self, mods: Flags, key: Key) -> Option<String>{        
        for (str_seq, bin) in &self.hotkeys {
           
            let sequence = KeySequence::new(str_seq.clone());

            if sequence.matches(mods, key) {    
                return Some(bin.clone());
            }
        }

        return None;
    }
}