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
use rand::thread_rng;
use rand::seq::IteratorRandom;

use crate::error::{OozeError, OozeResult};

/// A sprite that can be drawn to the window. Currently only contains a texture.
pub struct Sprite {
    pub texture: Texture2d,
}

impl Sprite {
    /// Create a new Sprite from a png image and a display context.
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

    /// Create a new Sprite from a png spritesheet and image coordinates.
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

/// Maps strings to a Sprite and handles loading of sprites from files or spritesheets.
pub struct SpriteMap {
    sprite_map: HashMap<String, Sprite>,
}

impl SpriteMap {
    /// Create a SpriteMap and load from individual png images located in the given folder and sub-folder.
    /// Each Sprite is stored in the map under it's filename without an extension. e.g. the sprite loaded from "test.png" would be accessed as SpriteMap.get("test"). 
    pub fn from_files(display: &Display, resource_folder: &Path) -> OozeResult<SpriteMap> {
        let mut map = HashMap::new();
        for file_path in glob((resource_folder.to_str().unwrap().to_owned() + r#"\**\*.png"#).as_str()).unwrap() {
            let path = file_path.unwrap();
            let id = String::from_str(path.file_stem().ok_or(OozeError::PathError)?.to_str().ok_or(OozeError::PathError)?).unwrap();
            map.insert(id, Sprite::new(path, display)?);
        }

        let sprite_map = SpriteMap {
            sprite_map: map
        };

        Ok(sprite_map)
    }

    /// Create a SpriteMap and load the sprites from a spritesheet and metadata file.
    /// The metadata file needs to be located in the same folder as the spritesheet. 
    /// It is a .toml file that contains the locations and names of each sprite in the image.
    /// See the resources folder for examples.
    pub fn from_sheet(display: &Display, sheet_path: &Path) -> OozeResult<SpriteMap> {
        let metadata_path = sheet_path.with_extension("toml");

        let root_table = read_to_string(metadata_path)?.parse::<Value>()?;

        let sprite_width = root_table["dimensions"]["sprite_width"].as_integer().ok_or(OozeError::GenericError)? as u32;
        let sprite_height = root_table["dimensions"]["sprite_height"].as_integer().ok_or(OozeError::GenericError)? as u32;
        
        let sprites = root_table["sprites"].as_table().ok_or(OozeError::GenericError)?;

        let mut map = HashMap::new();
        for name in sprites.keys() {
            let id = name.clone();
            let x = sprites[name][0].as_integer().unwrap() as u32;
            let y = sprites[name][1].as_integer().unwrap() as u32;

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

    /// Get the sprite with the given id from this SpriteMap.
    pub fn get(&self, id: &str) -> OozeResult<&Sprite> {
        Ok(self.sprite_map.get(id).ok_or(OozeError::GenericError)?)
    }

    /// Get a random sprite from this SpriteMap.
    pub fn get_random(&self) -> OozeResult<&Sprite> {
        let id = self.sprite_map.keys().choose(&mut thread_rng()).ok_or(OozeError::GenericError)?;
        Ok(self.get(id)?)
    }
}