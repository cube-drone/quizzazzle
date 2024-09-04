use std::collections::HashMap;
use std::cell::LazyCell;

#[derive(FromForm)]
pub struct FileDirectives{
    pub unmodified: Option<bool>, // return the image as-is
    pub grayscale: Option<bool>, // remove color from the image
    pub tall: Option<bool>,      // ignore max height
    pub wide: Option<bool>,      // ignore max width
    pub width: Option<u32>,      // force the width of the image
    pub height: Option<u32>,     // force the height of the image
    pub blur: Option<f32>,       // apply a blur to the image
    pub flip_horizontal: Option<bool>, // flip the image horizontally
    pub flip_vertical: Option<bool>,   // flip the image vertically
    pub flip_turnwise: Option<bool>,   // rotate the image 180 degrees (note: this is exactly the same as flipping both horizontally and vertically)
    pub color: Option<String>,   // grayscale, but instead: the whole image will be tinted this #hex color
}

const COLOR_STRING: &str = include_str!("./rgb.txt");

fn generate_color_map() -> HashMap<&'static str, (u8, u8, u8)>{
    let mut map = HashMap::new();
    for line in COLOR_STRING.lines(){
        if line.starts_with("#") {
            continue;
        }
        let mut parts = line.split('\t');
        let name = parts.next().unwrap();
        let color_hex = parts.next().unwrap();
        let r = u8::from_str_radix(&color_hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&color_hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&color_hex[5..7], 16).unwrap();

        map.insert(name, (r, g, b));
    }
    map
}

const COLOR_MAP: LazyCell<HashMap<&'static str, (u8, u8, u8)>> = LazyCell::new(|| generate_color_map());


impl FileDirectives{
    pub fn to_string(&self) -> String{
        let mut directives = vec![];
        // unmodified is not included because... if the file isn't modified, we don't need to save anything
        if self.grayscale.unwrap_or(false){
            directives.push("grayscale".to_string());
        }
        if self.tall.unwrap_or(false){
            directives.push("tall".to_string());
        }
        if self.wide.unwrap_or(false){
            directives.push("wide".to_string());
        }
        if let Some(width) = self.width{
            directives.push(format!("width{}", width));
        }
        if let Some(height) = self.height{
            directives.push(format!("height{}", height));
        }
        if let Some(blur) = self.blur{
            directives.push(format!("blur{}", blur));
        }
        if self.flip_horizontal.unwrap_or(false){
            directives.push("flip_horizontal".to_string());
        }
        if self.flip_vertical.unwrap_or(false){
            directives.push("flip_vertical".to_string());
        }
        if self.flip_turnwise.unwrap_or(false){
            directives.push("flip_turnwise".to_string());
        }
        if let Some(color) = &self.color{
            directives.push(format!("color{}", color));
        }
        directives.join("_")
    }

    pub fn color(&self) -> Option<(u8, u8, u8)>{
        if self.color.is_none(){
            return None;
        }
        let color = self.color.as_ref().unwrap();
        if let Some(color) = COLOR_MAP.get(color.replace("_", " ").to_lowercase().as_str()){
            return Some(*color);
        }

        let r = u8::from_str_radix(&color[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&color[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&color[4..6], 16).unwrap_or(0);
        Some((r, g, b))
    }
}