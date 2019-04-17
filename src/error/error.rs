use std::io;

use std::fmt;

use std::error::Error;

use image;

use glium;

use toml;


pub type OozeResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub enum OozeError {
    IoError(io::Error),
    ImageError(image::ImageError),
    TextureCreationError(glium::texture::TextureCreationError),
    TomlDeserializeError(toml::de::Error),
    DrawError(glium::DrawError),
    DisplayCreationError(glium::backend::glutin::DisplayCreationError),
    BadColorError([f32; 4]),
    OutOfBoundsError,
    PathError,
    GenericError,
}

impl fmt::Display for OozeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OozeError::IoError(err) => err.fmt(f),
            OozeError::ImageError(err) => err.fmt(f),
            OozeError::TextureCreationError(err) => err.fmt(f),
            OozeError::TomlDeserializeError(err) => err.fmt(f),
            OozeError::DrawError(err) => err.fmt(f),
            OozeError::DisplayCreationError(err) => err.fmt(f),
            OozeError::BadColorError(_c) => write!(f, "A color was badly formatted."),
            OozeError::OutOfBoundsError => write!(f, "Something was out of bounds."),
            OozeError::PathError => write!(f, "An error occured with a path."),
            OozeError::GenericError => write!(f, "Generic error occured with ooze."),
        }
    }
}

impl Error for OozeError {
    fn description(&self) -> &str {
        match self {
            OozeError::IoError(err) => err.description(),
            OozeError::ImageError(err) => err.description(),
            OozeError::TextureCreationError(err) => err.description(),
            OozeError::TomlDeserializeError(err) => err.description(),
            OozeError::DrawError(err) => err.description(),
            OozeError::DisplayCreationError(err) => err.description(),
            OozeError::BadColorError(_c) => "A badly formatted color",
            OozeError::OutOfBoundsError => "Out of bounds",
            OozeError::PathError => "Path error",
            OozeError::GenericError => "Generic ooze error",
        }
    }
}

impl From<io::Error> for OozeError {
    fn from(err: io::Error) -> OozeError {
        OozeError::IoError(err)
    }
}

impl From<image::ImageError> for OozeError {
    fn from(err: image::ImageError) -> OozeError {
        OozeError::ImageError(err)
    }
}

impl From<glium::texture::TextureCreationError> for OozeError {
    fn from(err: glium::texture::TextureCreationError) -> OozeError {
        OozeError::TextureCreationError(err)
    }
}

impl From<toml::de::Error> for OozeError {
    fn from(err: toml::de::Error) -> OozeError {
        OozeError::TomlDeserializeError(err)
    }
}

impl From<glium::DrawError> for OozeError {
    fn from(err: glium::DrawError) -> OozeError {
        OozeError::DrawError(err)
    }
}

impl From<glium::backend::glutin::DisplayCreationError> for OozeError {
    fn from(err: glium::backend::glutin::DisplayCreationError) -> OozeError {
        OozeError::DisplayCreationError(err)
    }
}

