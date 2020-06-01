use crate::fire::Fire;
use sdl2::mixer::Chunk;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf::Font;
use crate::camera::Camera;
use crate::eye::Eye;
use crate::input_handler::InputHandler;
use crate::player::Player;
use crate::resource_manager::ResourceManager;
use crate::resource_manager::FontDetails;
use crate::state::State;
use crate::tile::Tile;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use sdl2::mixer::Music;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::video::WindowContext;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub const TILE_WIDTH: u32 = 64;
pub const TILE_HEIGHT: u32 = 64;

pub const TILE_CHARS: [char; 6] = ['*', '5', '5', '6', '8', '9'];

type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

/// Contains all globally shared game data.
pub struct Context<'a> {
    pub flames: Vec<Fire>,
    pub tiles: Vec<Tile>,
    pub blocks: Vec<Tile>,
    pub eyes: Vec<Eye>,
    pub player: Player,
    pub camera: Camera,
    pub flags: HashMap<String, bool>,
    pub input: InputHandler,
    pub music: Music<'a>,
    pub socket_tex: Texture<'a>,
    pub back_fx: Chunk,
    pub move_fx: Chunk,
    pub select_fx: Chunk,
    pub enter_fx: Chunk,
    pub texture_manager: TextureManager<'a, WindowContext>,
    pub font_manager: FontManager<'a>,
    pub font_details: FontDetails,
}

impl<'a> Context<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Self {
        Context {
            flames: Vec::new(),
            tiles: Vec::new(),
            blocks: Vec::new(),
            eyes: Vec::new(),
            player: Player::new(),
            camera: Camera::new(),
            flags: HashMap::new(),
            input: InputHandler::new(),
            texture_manager: TextureManager::new(&texture_creator),
            socket_tex: texture_creator.load_texture(Path::new("res/img/socket.png")).unwrap(),
            back_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/back.ogg")).unwrap(),
            move_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/push.ogg")).unwrap(),
            select_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/select.ogg")).unwrap(),
            enter_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/enter.ogg")).unwrap(),
            music: sdl2::mixer::Music::from_file(Path::new("res/sound/laidback.mp3")).unwrap(),
            font_manager: FontManager::new(ttf_context),
            font_details: FontDetails { path: String::from("res/fonts/VeniceClassic.ttf"), size: 19},
        }
    }

    fn get_texture_name(tile: char) -> String {
        if tile == '8' {
            return String::from("res/img/dbg_floor_shadow_top.png");
        } else if tile == '9' {
            return String::from("res/img/dbg_floor_shadow_ne_corner.png");
        } else if tile == '6' {
            return String::from("res/img/dbg_floor_shadow_right.png");
        } else if tile == '5' {
            return String::from("res/img/dbg_floor_shadow_corner.png");
        } else if tile == '*' {
            return String::from("res/img/dbg_floor.png");
        }
        String::from("res/img/dbg_floor.png")
    }

    pub fn load_level(&mut self, level: String) {       
        self.font_manager.load(&self.font_details).unwrap();
        let f = fs::read_to_string(level).expect("Could not load level!");
        let mut cury: i32 = 10;
        let mut temp_blocks: Vec<Tile> = Vec::new();
        let mut temp_eyes: Vec<Eye> = Vec::new();
        self.tiles.clear();
        self.eyes.clear();
        self.player = Player::new();
        let mut skip = true;
        for line in f.lines() {
            if skip {
                // Skip the first line of the file.
                skip = false;
                continue;
            }
            let mut curx: i32 = 10;
            for c in line.chars() {
                if TILE_CHARS.contains(&c) {
                    self.tiles.push(Tile {
                        texture: Context::get_texture_name(c),
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
                    curx += TILE_WIDTH as i32;
                } else if c == 'f' {
                    let mut flame = Fire::new();
                    flame.x = curx + TILE_WIDTH as i32 / 2 - flame.width as i32/ 2;
                    flame.y = cury;
                    self.flames.push(flame);
                    self.tiles.push(Tile {
                        texture: String::from("res/img/torch.png"),
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
                    curx += TILE_WIDTH as i32;
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
                    curx += TILE_WIDTH as i32;
                } else if c == 'b' {
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
                    self.player.x = curx + (TILE_WIDTH / 2) as i32 - (self.player.width / 2) as i32;
                    self.player.y =
                        cury + 3 - (TILE_HEIGHT / 2) as i32 + (self.player.height / 2) as i32;
                } else if c == ' ' {
                    curx += TILE_WIDTH as i32;
                }
            }
            cury += TILE_HEIGHT as i32;
        }
        for tile in temp_blocks {
            self.tiles.push(tile);
        }
        for eye in temp_eyes {
            self.eyes.push(eye);
        }

        self.camera.minx = self.player.x + self.player.width as i32 / 2;
        self.camera.maxx = self.player.x + self.player.width as i32 / 2;
        self.camera.miny = self.player.y - self.player.height as i32 / 2;
        self.camera.maxy = self.player.y - self.player.height as i32 / 2;
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
