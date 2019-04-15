use image;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use glium::uniforms::Sampler;
use glium::Display;
use glob::glob;
use xmltree;

pub struct Sprite {
    pub texture: Texture2d,
}

impl Sprite {
    pub fn new<P: AsRef<Path>>(img_path: P, display: &Display) -> Sprite {
        let r = BufReader::new(File::open(img_path).unwrap());
        let image = image::load(r, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        Sprite {
            texture: Texture2d::new(display, image).unwrap()
        }
    }

    pub fn from_sheet<P: AsRef<Path>>(sheet_path: P, display: &Display, x: u32, y: u32, width: u32, height: u32) -> Sprite {
        let r = BufReader::new(File::open(sheet_path).unwrap());
        let mut sheet = image::load(r, image::PNG).unwrap().to_rgba();
        let sub_image = image::imageops::crop(&mut sheet, x, y, width, height).to_image();
        let image_dimensions = sub_image.dimensions();
        let sub_image = RawImage2d::from_raw_rgba_reversed(&sub_image.into_raw(), image_dimensions);
    
        Sprite {
            texture: Texture2d::new(display, sub_image).unwrap()
        }
    }
}

pub struct SpriteMap {
    sprite_map: HashMap<String, Sprite>,
}

impl SpriteMap {
    pub fn from_files(display: &Display, resource_folder: &Path) -> SpriteMap {
        let mut map = HashMap::new();
        for file_path in glob((resource_folder.to_str().unwrap().to_owned() + r#"\**\*.png"#).as_str()).unwrap() {
            let path = file_path.unwrap();
            let id = String::from_str(path.file_stem().unwrap().to_str().unwrap()).unwrap();
            map.insert(id, Sprite::new(path, display));
        }

        SpriteMap {
            sprite_map: map
        }
    }

    pub fn from_sheet(display: &Display, sheet_path: &Path) -> SpriteMap {
        let mut map = HashMap::new();
        
        let metadata_path = sheet_path.with_extension("xml");
        println!("{:?}", &metadata_path);

        let reader = BufReader::new(File::open(metadata_path).unwrap());
        let sheet_element = xmltree::Element::parse(reader).unwrap();
        let dims_element = sheet_element.get_child("dimensions").unwrap();
        let sprites_element = sheet_element.get_child("sprites").unwrap();

        let sprite_width = dims_element.get_child("sprite_width").unwrap().text.clone().unwrap().parse::<u32>().unwrap();
        let sprite_height = dims_element.get_child("sprite_height").unwrap().text.clone().unwrap().parse::<u32>().unwrap();
        let sheet_width = dims_element.get_child("sheet_width").unwrap().text.clone().unwrap().parse::<u32>().unwrap();
        let sheet_height = dims_element.get_child("sheet_height").unwrap().text.clone().unwrap().parse::<u32>().unwrap();

        for sprite_element in &sprites_element.children {
            let id = sprite_element.attributes.get("id").unwrap().clone();
            let x = sprite_element.get_child("x").unwrap().text.clone().unwrap().parse::<u32>().unwrap();
            let y = sprite_element.get_child("y").unwrap().text.clone().unwrap().parse::<u32>().unwrap();
        
            map.insert(id, Sprite::from_sheet(sheet_path, display, x*sprite_width, y*sprite_height, sprite_width, sprite_height));
        }

        SpriteMap {
            sprite_map: map
        }
    }

    pub fn get(&self, id: &String) -> &Sprite {
        self.sprite_map.get(id).unwrap()
    }
}