use serde::ser::Serialize;
use serde::de::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct BackgroundConfig {
    pub wallpaper_path: Option<String>,
    rgb_color: Vec<u8>
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        BackgroundConfig {
            wallpaper_path: None,
            rgb_color: vec![25u8, 25u8, 25u8]
        }
    }
}

impl BackgroundConfig {
    pub fn color_for_gl(&self) -> Option<(f32, f32, f32)>{
        let num_colors = 3; 
        let max_val = 255f32; 
        
        if self.rgb_color.len() == num_colors {
            Some((
                *self.rgb_color.get(0).unwrap() as f32 / max_val, 
                *self.rgb_color.get(1).unwrap() as f32 / max_val, 
                *self.rgb_color.get(2).unwrap() as f32 / max_val
            ))
        }
        else{
            None
        }
    }
}   