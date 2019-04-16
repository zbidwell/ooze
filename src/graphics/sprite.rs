use image;
use std::io::BufReader;
use std::path::{Path};
use std::fs::{File, read_to_string};
use std::collections::HashMap;
use std::str::FromStr;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use glium::Display;
use glob::glob;
use toml::Value;

use crate::app::{OozeResult, OozeError};

pub struct Sprite {
    pub texture: Texture2d,
}

impl Sprite {
    pub fn new<P: AsRef<Path>>(img_path: P, display: &Display) -> OozeResult<Sprite> {
        let r = BufReader::new(File::open(img_path)?);
        let image = image::load(r, image::PNG)?.to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let sprite = Sprite {
            texture: Texture2d::new(display, image)?
        };

        Ok(sprite)
    }

    pub fn from_sheet<P: AsRef<Path>>(sheet_path: P, display: &Display, x: u32, y: u32, width: u32, height: u32) -> OozeResult<Sprite> {
        let r = BufReader::new(File::open(sheet_path)?);
        let mut sheet = image::load(r, image::PNG)?.to_rgba();
        let sub_image = image::imageops::crop(&mut sheet, x, y, width, height).to_image();
        let image_dimensions = sub_image.dimensions();
        let sub_image = RawImage2d::from_raw_rgba_reversed(&sub_image.into_raw(), image_dimensions);
    
        let sprite = Sprite {
            texture: Texture2d::new(display, sub_image)?
        };

        Ok(sprite)
    }
}

pub struct SpriteMap {
    sprite_map: HashMap<String, Sprite>,
}

impl SpriteMap {
    pub fn from_files(display: &Display, resource_folder: &Path) -> OozeResult<SpriteMap> {
        let mut map = HashMap::new();
        for file_path in glob((resource_folder.to_str().ok_or(OozeError)?.to_owned() + r#"\**\*.png"#).as_str()).unwrap() {
            let path = file_path?;
            let id = String::from_str(path.file_stem().ok_or(OozeError)?.to_str().ok_or(OozeError)?)?;
            map.insert(id, Sprite::new(path, display)?);
        }

        let sprite_map = SpriteMap {
            sprite_map: map
        };

        Ok(sprite_map)
    }

    pub fn from_sheet(display: &Display, sheet_path: &Path) -> OozeResult<SpriteMap> {
        let metadata_path = sheet_path.with_extension("toml");

        let root_table = read_to_string(metadata_path)?.parse::<Value>()?;

        let sprite_width = root_table["dimensions"]["sprite_width"].as_integer().ok_or(OozeError)? as u32;
        let sprite_height = root_table["dimensions"]["sprite_height"].as_integer().ok_or(OozeError)? as u32;
        
        let sprites = root_table["sprites"].as_table().ok_or(OozeError)?;

        let mut map = HashMap::new();
        for name in sprites.keys() {
            let id = name.clone();
            let x = sprites[name][0].as_integer().ok_or(OozeError)? as u32;
            let y = sprites[name][1].as_integer().ok_or(OozeError)? as u32;

            map.insert(
                id,
                Sprite::from_sheet(
                    sheet_path,
                    display,
                    x * sprite_width,
                    y * sprite_height,
                    sprite_width,
                    sprite_height
                )?
            );
        }

        let sprite_map = SpriteMap {
            sprite_map: map
        };

        Ok(sprite_map)
    }

    pub fn get(&self, id: &String) -> OozeResult<&Sprite> {
        match self.sprite_map.get(id) {
            Some(s) => Ok(s),
            None => Err(Box::new(OozeError)),
        }
    }
}