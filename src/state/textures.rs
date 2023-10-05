use std::collections::HashMap;

pub struct Textures {
    textures: HashMap<String, HashMap<String, glium::Texture2d>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}
