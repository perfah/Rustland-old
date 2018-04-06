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

#[derive(Serialize, Deserialize)]
pub struct KeyboardConfig {
    pub mod_key: String,
    pub meta_view_key: Key,
    pub hotkeys: HashMap<String, String>,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        KeyboardConfig {
            mod_key: "Logo".to_string(),
            meta_view_key: Key::Tab,
            hotkeys: [
                (format!("mod{}Space",    KEY_DIVISOR), "/usr/bin/dmenu_run".to_string()),
                (format!("mod{}T",        KEY_DIVISOR), "/usr/bin/terminator".to_string()),
                (format!("VolumeUp"),                   "pactl set-sink-volume 0 +5%".to_string()),
                (format!("VolumeDown"),                 "pactl set-sink-volume 0 -5%".to_string()),
                (format!("BrightnessUp"),               "gksu brightnessctl s 50+".to_string()),
                (format!("BrightnessDown"),             "gksu brightnessctl s 50-".to_string()),
            ].iter().cloned().collect()
        }
    }
}

impl KeyboardConfig {
    pub fn mod_key_is_pressed(&self, mods: Flags) -> bool {
        KeySequence::new(self.mod_key.clone()).matches(mods, None)
    }

    pub fn matching_hotkey(&self, mods: Flags, key: Key) -> Option<String>{        
        for (str_seq, bin) in &self.hotkeys {
            if KeySequence::new(str_seq.replace("mod", &self.mod_key)).matches(mods, Some(key)) {    
                return Some(bin.clone());
            }
        }

        return None;
    }
}

// Data structure for interpreting keyboard input sequences (strings)
// ==================================================================

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
        if src.is_empty() {
            KeySequence::Unrecognized
        }
        else if !src.contains(KEY_DIVISOR) {
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
        KeySequence::Combo (
            box KeySequence::End,
            box KeySequence::End
        )
    }

    pub fn matches(&self, mods: Flags, key: Option<Key>) -> bool {
        match self {
            &KeySequence::Mod(m) => mods.contains(m),
            &KeySequence::RegularKey(k1) => if let Some(k2) = key { k1 == k2 } else { false },
            &KeySequence::Combo(ref former, ref latter) => 
                former.matches(mods, key) && 
                latter.matches(mods, key),
            &KeySequence::End => true,
            &KeySequence::Unrecognized => false
        }
    }
}
