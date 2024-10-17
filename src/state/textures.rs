use std::{collections::HashMap, fs, path::PathBuf};

use glium::{glutin::surface::WindowSurface, texture::RawImage2d, Display, Texture2d};
use image::GenericImageView;

const TEXTURES_PATH: &str = "textures";

pub struct Textures {
    textures: HashMap<String, HashMap<String, Texture2d>>,
}

impl Textures {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        Self {
            textures: Self::load_textures(display),
        }
    }
    fn load_texture(display: &Display<WindowSurface>, image_path: PathBuf) -> Option<Texture2d> {
        let dynamic_image = image::open(image_path).ok()?;
        let (width, height) = dynamic_image.dimensions();
        let raw_image =
            RawImage2d::from_raw_rgba_reversed(dynamic_image.as_bytes(), (width, height));
        Texture2d::new(display, raw_image).ok()
    }
    fn load_textures(
        display: &Display<WindowSurface>,
    ) -> HashMap<String, HashMap<String, Texture2d>> {
        let mut textures = HashMap::new();

        if let Ok(directories) = fs::read_dir(TEXTURES_PATH) {
            for directory in directories.flatten() {
                if let Ok(images) = fs::read_dir(directory.path()) {
                    let subtextures = textures
                        .entry(directory.file_name().to_string_lossy().to_string())
                        .or_insert(HashMap::new());
                    for image in images.flatten() {
                        if let Some(texture) = Self::load_texture(display, image.path()) {
                            let name = image.file_name().to_string_lossy().to_string();
                            let name = &name[0..name.len() - 4];
                            subtextures.insert(name.to_owned(), texture);
                        }
                    }
                }
            }
        }
        textures
    }
    pub fn get_texture(&self, group: &str, texture: &str) -> Option<&Texture2d> {
        self.textures.get(group)?.get(texture)
    }
}
