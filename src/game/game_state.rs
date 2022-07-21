use barn::graphics::barn_gfx::BarnGFX;
use barn::game::barn_context::BarnContext;
use barn::graphics::color::Color;
use barn::graphics::fill_type::FillType;
use barn::math::vector2::Vector2;
use barn::math::bounding_box_2d::BoundingBox2D;
use barn::game::state::State;
use crate::game::camera::Camera;
use crate::game::eye::Eye;
use crate::game::fire::Fire;
use crate::game::level_select_state::LevelSelectState;
use crate::game::physics::handle_collisions;
use crate::game::player::Player;
use crate::game::tile::Tile;
use crate::settings;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use std::fs;
use std::time::Instant;

pub const TILE_WIDTH: u32 = 64;
pub const TILE_HEIGHT: u32 = 64;

pub const TILE_CHARS: [char; 6] = ['*', '5', '5', '6', '8', '9'];

pub struct GameState/*<'a>*/ {
    pub level_path: String,
    pub won: bool,
    pub time: Instant,
    pub time_str: String,
    pub moves: u32,
    pub flames: Vec<Fire>,
    pub tiles: Vec<Tile>,
    pub blocks: Vec<Tile>,
    pub eyes: Vec<Eye>,
    pub player: Player,
    pub camera: Camera,
}

impl State<BarnContext> for GameState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> {
        if context.input.key_just_pressed(&Keycode::R) {
            self.load_level(String::from(&self.level_path), context);
        } else if context.input.key_just_pressed(&Keycode::Q) {
            return Some(Box::new(LevelSelectState::new(0)));
        }

        for fire in self.flames.iter_mut() {
            fire.update();
        }

        // TODO: Refactor this.
        let new_tiles = &mut self.tiles.to_vec();
        for tile in self.tiles.iter_mut() {
            if tile.has_moved() {
                self.moves += 1;
            }
            let move_fx = context.load_sound(String::from("res/sound/push.ogg"));
            tile.update(new_tiles, move_fx);
        }

        // Check if the puzzle has been solved.
        self.won = true;
        for eye in self.eyes.iter_mut() {
            eye.update(&mut self.tiles, dt);
            if !eye.solved {
                self.won = false;
            }
        }
        if !self.won {
            // Update the player.
            self.player.update(&mut context.input, dt);
            let move_fx = context.load_sound(String::from("res/sound/push.ogg"));
            handle_collisions(&mut self.player, &mut self.tiles, move_fx);
            self.camera.focus(
                self.player.pos.x as i32 + self.player.width as i32 / 2,
                self.player.pos.y as i32 + self.player.height as i32 / 2,
            );
        } else {
            if self.time_str == "" {
                let elapsed_time = self.time.elapsed().as_secs();
                self.time_str = elapsed_time.to_string();
            }
            if context.input.key_just_pressed(&Keycode::Return) {
                return Some(Box::new(LevelSelectState::new(0)));
            }
        }

        // No state change has occured.
        None
    }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        // Clear screen to black.
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();
        
        // Get all of the image assets.
        for tile in &mut self.tiles {
            if !tile.isblock
                && !tile.iswall
                && self
                    .camera
                    .is_object_visible(tile.bb.origin.x as i32, 
                        tile.bb.origin.y as i32, 
                        tile.bb.width as u32,
                        tile.bb.height as u32)
            {
                tile.draw(
                    &mut context.load_texture(tile.texture.clone()),
                    &mut self.camera,
                    bgfx,
                );
            }
        }
        let tex_shadow = context.load_texture(String::from("res/img/drop_shadow.png"));
        self.player
            .draw_shadow(tex_shadow, &mut self.camera, bgfx);
        for tile in &mut self.tiles {
            if (tile.isblock || tile.iswall)
                && self
                    .camera
                    .is_object_visible(tile.bb.origin.x as i32, 
                        tile.bb.origin.y as i32, 
                        tile.bb.width as u32,
                        tile.bb.height as u32)
            {
                tile.draw(
                    &mut context.load_texture(tile.texture.clone()),
                    &mut self.camera,
                    bgfx,
                );
            }
        }
        // Render menu eyes.
        for eye in self.eyes.iter_mut() {
            let socket_tex =  context.load_texture(String::from("res/img/socket.png"));
            eye.draw_socket(socket_tex, &mut self.camera, bgfx);
            let tex_pupil = if eye.color == "blue" {
                context.load_texture(String::from("res/img/bluepupil.png"))
            } else {
                if eye.color == "red" {
                    context.load_texture(String::from("res/img/redpupil.png"))
                } else {
                    context.load_texture(String::from("res/img/greenpupil.png"))
                }
            };
            eye.draw_iris(tex_pupil, &mut self.camera, bgfx);
        }
        let tex_player = context.load_texture(String::from("res/img/player.png"));
        self.player.draw(tex_player, &mut self.camera, bgfx);
        // let tex_fire = context.load_texture(String::from("res/img/fire2.png"));
        // let tex_glow = context
        //     .texture_manager
        //     .load("res/img/fire_glow.png")
        //     .unwrap();
        // for fire in self.flames.iter_mut() {
        //     fire.draw(&tex_fire, &tex_glow, &mut self.camera, canvas)
        // }
        if self.won {
            bgfx.sdl.set_draw_color(Color::from_rgba(0, 0, 0, 150));

            // TODO: THIS MUST BE ADDED TO BARN
            //canvas.set_blend_mode(BlendMode::Blend);
            bgfx.sdl.draw_rect(0, 0, self.camera.width as u32, self.camera.height as u32, FillType::FILL, false);

            let font = context.load_font(*settings::FONT_DETAILS);

            // Render the title.
            bgfx.sdl.draw_text("Solved!", font, 
            self.camera.width as f32 / 2.0 - 4.0, 30.0, 4.0, 4.0, true, false);

            // Render time result.
            bgfx.sdl.draw_text(&format!("Time: {} seconds", self.time_str), font, 
                self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 3.0, 3.0, true, true);

            // Render number of moves.
            bgfx.sdl.draw_text(&format!("Moves taken: {}", self.moves), font, 
                self.camera.width as f32 / 2.0, self.camera.height as f32 / 2.0, 3.0, 3.0, true, true);

            // Render number of moves.
            bgfx.sdl.draw_text("Press enter to go back", font, 
                0.0, self.camera.height as f32 / 2.0, 2.0, 2.0, false, false);
            
        }
        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        // self.camera.width = context.screen_width as i32;
        // self.camera.height = context.screen_height as i32;
        self.camera = Camera::new();
        self.camera.width = 800;
        self.camera.height = 600;
        self.load_level(String::from(&self.level_path), context);
        self.time = Instant::now();
    }

    fn on_exit(&mut self, context: &mut BarnContext) {
        self.flames.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("game")
    }
}

impl GameState {
    pub fn new(path: String) -> Self {
        GameState {
            level_path: path,
            won: false,
            time: Instant::now(),
            time_str: String::from(""),
            moves: 0,
            flames: Vec::new(),
            tiles: Vec::new(),
            blocks: Vec::new(),
            eyes: Vec::new(),
            player: Player::new(),
            camera: Camera::new()
        }
    }

    pub fn load_level(&mut self, level: String, context: &mut BarnContext) {
        context.load_font(*settings::FONT_DETAILS);
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
                        texture: GameState::get_texture_name(c),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
                        iswall: false,
                        isblock: false,
                    });
                    curx += TILE_WIDTH as i32;
                } else if c == 'f' {
                    let mut flame = Fire::new();
                    flame.x = curx + TILE_WIDTH as i32 / 2 - flame.width as i32 / 2;
                    flame.y = cury;
                    self.flames.push(flame);
                    self.tiles.push(Tile {
                        texture: String::from("res/img/torch.png"),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
                        iswall: true,
                        isblock: false,
                    });
                    curx += TILE_WIDTH as i32;
                } else if c == 'x' {
                    self.tiles.push(Tile {
                        texture: String::from("res/img/grayblock.png"),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
                        iswall: true,
                        isblock: false,
                    });
                    curx += TILE_WIDTH as i32;
                } else if c == 'b' {
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/blueblock.png"),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
                        iswall: false,
                        isblock: true,
                    });
                } else if c == 'g' {
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/greenblock.png"),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
                        iswall: false,
                        isblock: true,
                    });
                } else if c == 'r' {
                    temp_blocks.push(Tile {
                        texture: String::from("res/img/redblock.png"),
                        bb: BoundingBox2D {origin: Vector2 {x: curx as f32, y: cury as f32}, width: TILE_WIDTH, height: TILE_HEIGHT},
                        target_pos: Vector2 {x: curx as f32, y: cury as f32},
                        resistance: 30,
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
                        deltax: 0.0,
                        deltay: 0.0,
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
                        deltax: 0.0,
                        deltay: 0.0,
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
                        deltax: 0.0,
                        deltay: 0.0,
                        anger: 0,
                    });
                } else if c == 'p' {
                    self.player.pos.x = (curx + (TILE_WIDTH / 2) as i32 - (self.player.width / 2) as i32) as f32;
                    self.player.pos.y =
                        (cury + 3 - (TILE_HEIGHT / 2) as i32 + (self.player.height / 2) as i32) as f32;
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

        self.camera.minx = self.player.pos.x as i32 + self.player.width as i32 / 2;
        self.camera.maxx = self.player.pos.x as i32+ self.player.width as i32 / 2;
        self.camera.miny = self.player.pos.y as i32 - self.player.height as i32 / 2;
        self.camera.maxy = self.player.pos.y as i32 - self.player.height as i32 / 2;
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
}
