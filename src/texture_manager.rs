use sdl2::video::WindowContext;
use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use std::path::Path;

pub struct TextureManager<'a> {
    tc: &'a TextureCreator<WindowContext>,
    root_path: String,
    textures: HashMap<String, Texture<'a>>,
}

impl<'s> TextureManager<'s> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>, initial_path: String) -> TextureManager {
        return TextureManager { tc: &texture_creator, root_path: initial_path,
                                textures: HashMap::new() };
    }

    pub fn load(&mut self, resource: &str) -> &Texture {
        let full_resource_path = String::from(Path::new(self.root_path.as_str())
            .join(resource).to_str().unwrap());

        {
            if self.textures.contains_key(&full_resource_path) {
                return self.textures.get(&full_resource_path).unwrap();
            }
        }
        let new_texture = self.tc.load_texture(Path::new(self.root_path.as_str())
            .join(resource).as_path()).unwrap();
        self.textures.insert(full_resource_path.clone(), new_texture);

        return self.textures.get(&full_resource_path).unwrap().clone();
    }

    fn unload_all(&mut self) {
        self.textures.clear();
    }
}