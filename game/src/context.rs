use crate::camera::Camera;
use crate::eye::Eye;
use crate::input_handler::InputHandler;
use crate::player::Player;
use crate::resource_manager::ResourceManager;
use crate::state::State;
use crate::tile::Tile;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use sdl2::mixer::Music;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

const TILE_WIDTH: u32 = 64;
const TILE_HEIGHT: u32 = 64;

type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

/// Contains all globally shared game data.
pub struct Context<'a> {
    pub tiles: Vec<Tile>,
    pub blocks: Vec<Tile>,
    pub eyes: Vec<Eye>,
    pub player: Player,
    pub camera: Camera,
    pub flags: HashMap<String, bool>,
    pub input: InputHandler,
    pub music: Music<'a>,
    // Textures
    pub texture_manager: TextureManager<'a, WindowContext>,
}

impl<'a> Context<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Context {
            tiles: Vec::new(),
            blocks: Vec::new(),
            eyes: Vec::new(),
            player: Player::new(),
            camera: Camera::new(),
            flags: HashMap::new(),
            input: InputHandler::new(),
            texture_manager: TextureManager::new(&texture_creator),
            music: sdl2::mixer::Music::from_file(Path::new("res/sound/sanchopanza.mp3")).unwrap(),
        }
    }

    pub fn load_level(&mut self, level: String) {
        self.music.play(-1).unwrap();
        let f = fs::read_to_string(level).expect("Could not load level!");
        let mut cury: i32 = 10;
        let mut temp_blocks: Vec<Tile> = Vec::new();
        let mut temp_eyes: Vec<Eye> = Vec::new();
        for line in f.lines() {
            let mut curx: i32 = 10;
            for c in line.chars() {
                if c == '*' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/dbg_floor.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: false,
                        isblock: false,
                    });
                } else if c == 'x' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/grayblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: true,
                        isblock: false,
                    });
                } else if c == 'b' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/dbg_floor.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: false,
                        isblock: false,
                    });
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/blueblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 30,
                        resistancey: 30,
                        iswall: false,
                        isblock: true,
                    });
                } else if c == 'g' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/dbg_floor.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: false,
                        isblock: false,
                    });
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/greenblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 30,
                        resistancey: 30,
                        iswall: false,
                        isblock: true,
                    });
                } else if c == 'r' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/dbg_floor.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: false,
                        isblock: false,
                    });
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/redblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 30,
                        resistancey: 30,
                        iswall: false,
                        isblock: true,
                    });
                } else if c == 'B' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/grayblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: true,
                        isblock: false,
                    });
                    temp_eyes.push(Eye {
                        direction: String::from("left"),
                        x: curx,
                        y: cury,
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        color: String::from("blue"),
                        solved: false,
                        deltax: 0,
                        deltay: 0,
                        anger: 0,
                    });
                } else if c == 'R' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/grayblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: true,
                        isblock: false,
                    });
                    temp_eyes.push(Eye {
                        direction: String::from("left"),
                        x: curx,
                        y: cury,
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        color: String::from("red"),
                        solved: false,
                        deltax: 0,
                        deltay: 0,
                        anger: 0,
                    });
                } else if c == 'G' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/grayblock.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: true,
                        isblock: false,
                    });
                    temp_eyes.push(Eye {
                        direction: String::from("left"),
                        x: curx,
                        y: cury,
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        color: String::from("green"),
                        solved: false,
                        deltax: 0,
                        deltay: 0,
                        anger: 0,
                    });
                } else if c == 'p' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/dbg_floor.png"),
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                        x: curx,
                        y: cury,
                        targetx: curx,
                        targety: cury,
                        resistancex: 0,
                        resistancey: 0,
                        iswall: false,
                        isblock: false,
                    });
                    self.player.x = curx + (TILE_WIDTH / 2) as i32 - (self.player.width / 2) as i32;
                    self.player.y =
                        cury - (TILE_HEIGHT / 2) as i32 + (self.player.height / 2) as i32;
                }
                curx += TILE_WIDTH as i32;
            }
            cury += TILE_HEIGHT as i32;
        }
        for tile in temp_blocks {
            self.tiles.push(tile);
        }
        for eye in temp_eyes {
            self.eyes.push(eye);
        }
    }

    pub fn update(
        &mut self,
        state: &mut dyn State,
        event: &mut EventPump,
    ) -> Option<Box<dyn State>> {
        self.input.update(event);
        state.update(self)
    }

    pub fn draw(&mut self, state: &mut dyn State, canvas: &mut WindowCanvas) {
        state.draw(self, canvas);
        self.input.refresh_prev();
    }
}
