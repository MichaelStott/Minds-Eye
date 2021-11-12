use crate::barn::game::context::Context;
use crate::barn::game::state::State;
use crate::camera::Camera;
use crate::eye::Eye;
use crate::fire::Fire;
use crate::level_select_state::LevelSelectState;
use crate::physics::handle_collisions;
use crate::player::Player;
use crate::tile::Tile;
use sdl2::mixer::Chunk;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::WindowCanvas;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
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
    pub move_fx: Chunk,
    pub camera: Camera,
    // pub socket_tex: Texture<'a>,
}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> {
        if context.input.key_just_pressed(&Keycode::R) {
            self.load_level(String::from(&self.level_path), context);
        } else if context.input.key_just_pressed(&Keycode::Q) {
            return Some(Box::new(LevelSelectState {
                levels: HashMap::new(),
                options: Vec::new(),
                selected_option: 0,
                camera: Camera::new(),
                back_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/back.ogg")).unwrap(),
                select_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/select.ogg")).unwrap(),
                enter_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/enter.ogg")).unwrap(),
                tiles: Vec::new(),
                eyes: Vec::new(),
            }));
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
            tile.update(new_tiles, &self.move_fx);
        }

        // Check if the puzzle has been solved.
        self.won = true;
        for eye in self.eyes.iter_mut() {
            eye.update(&mut self.tiles);
            if !eye.solved {
                self.won = false;
            }
        }
        if !self.won {
            // Update the player.
            self.player.update(&mut context.input);
            handle_collisions(&mut self.player, &mut self.tiles, &self.move_fx);
            self.camera.focus(
                self.player.x + self.player.width as i32 / 2,
                self.player.y + self.player.height as i32 / 2,
            );
        } else {
            if self.time_str == "" {
                let elapsed_time = self.time.elapsed().as_secs();
                self.time_str = elapsed_time.to_string();
            }
            if context.input.key_just_pressed(&Keycode::Return) {
                return Some(Box::new(LevelSelectState {
                    levels: HashMap::new(),
                    options: Vec::new(),
                    selected_option: 0,
                    camera: Camera::new(),
                    back_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/back.ogg")).unwrap(),
                    select_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/select.ogg")).unwrap(),
                    enter_fx: sdl2::mixer::Chunk::from_file(Path::new("res/sound/enter.ogg")).unwrap(),
                    tiles: Vec::new(),
                    eyes: Vec::new(),
                }));
            }
        }

        // No state change has occured.
        None
    }

    fn draw(&mut self, context: &mut Context, canvas: &mut WindowCanvas) {
        self.camera.width = (canvas.output_size().unwrap().0) as i32;
        self.camera.height = (canvas.output_size().unwrap().1) as i32;
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Get all of the image assets.
        let tex_player = context.texture_manager.load("res/img/player.png").unwrap();
        let tex_shadow = context
            .texture_manager
            .load("res/img/drop_shadow.png")
            .unwrap();
        // Draw the scene.
        for tile in &mut self.tiles {
            if !tile.isblock
                && !tile.iswall
                && self
                    .camera
                    .is_object_visible(tile.x, tile.y, tile.width, tile.height)
            {
                tile.draw(
                    &context.texture_manager.load(&tile.texture).unwrap(),
                    &mut self.camera,
                    canvas,
                );
            }
        }
        self.player
            .draw_shadow(&tex_shadow, &mut self.camera, canvas);
        for tile in &mut self.tiles {
            if (tile.isblock || tile.iswall)
                && self
                    .camera
                    .is_object_visible(tile.x, tile.y, tile.width, tile.height)
            {
                tile.draw(
                    &context.texture_manager.load(&tile.texture).unwrap(),
                    &mut self.camera,
                    canvas,
                );
            }
        }
        for eye in self.eyes.iter_mut() {
            if self
                .camera
                .is_object_visible(eye.x, eye.y, eye.width, eye.height)
            {
                let tex_pupil = if eye.color == "blue" {
                    context
                        .texture_manager
                        .load("res/img/bluepupil.png")
                        .unwrap()
                } else {
                    if eye.color == "red" {
                        context
                            .texture_manager
                            .load("res/img/redpupil.png")
                            .unwrap()
                    } else {
                        context
                            .texture_manager
                            .load("res/img/greenpupil.png")
                            .unwrap()
                    }
                };
                // eye.draw(&mut self.socket_tex, &tex_pupil, &mut self.camera, canvas);
            }
        }
        self.player.draw(&tex_player, &mut self.camera, canvas);
        let tex_fire = context.texture_manager.load("res/img/fire2.png").unwrap();
        let tex_glow = context
            .texture_manager
            .load("res/img/fire_glow.png")
            .unwrap();
        for fire in self.flames.iter_mut() {
            fire.draw(&tex_fire, &tex_glow, &mut self.camera, canvas)
        }
        if self.won {
            canvas.set_draw_color(Color::RGBA(0, 0, 0, 150));
            canvas.set_blend_mode(BlendMode::Blend);
            canvas
                .fill_rect(Rect::new(
                    0,
                    0,
                    self.camera.width as u32,
                    self.camera.height as u32,
                ))
                .unwrap();
            let font = context.font_manager.load(&context.font_details).unwrap();
            let texture_creator = canvas.texture_creator();

            // Render the title.
            let title = font
                .render("Solved!")
                .blended(Color::RGBA(255, 255, 255, 255))
                .unwrap();
            let title_tex = texture_creator.create_texture_from_surface(&title).unwrap();
            canvas
                .copy(
                    &title_tex,
                    None,
                    Rect::new(
                        self.camera.width / 2 - 4 * title.size().0 as i32 / 2,
                        30,
                        title.size().0 * 4,
                        title.size().1 * 4,
                    ),
                )
                .unwrap();

            // Render time result.
            let time = font
                .render(&format!("Time: {} seconds", self.time_str))
                .blended(Color::RGBA(255, 255, 255, 255))
                .unwrap();
            let time_tex = texture_creator.create_texture_from_surface(&time).unwrap();
            canvas
                .copy(
                    &time_tex,
                    None,
                    Rect::new(
                        self.camera.width / 2 - 3 * time.size().0 as i32 / 2,
                        self.camera.height / 2 - 3 * time.size().1 as i32 / 2,
                        time.size().0 * 3,
                        time.size().1 * 3,
                    ),
                )
                .unwrap();

            // Render number of moves.
            let moves = font
                .render(&format!("Moves taken: {}", self.moves))
                .blended(Color::RGBA(255, 255, 255, 255))
                .unwrap();
            let moves_tex = texture_creator.create_texture_from_surface(&moves).unwrap();
            canvas
                .copy(
                    &moves_tex,
                    None,
                    Rect::new(
                        self.camera.width / 2 - 3 * moves.size().0 as i32 / 2,
                        self.camera.height / 2 + 4 * moves.size().1 as i32 / 2,
                        moves.size().0 * 3,
                        moves.size().1 * 3,
                    ),
                )
                .unwrap();

            // Render number of moves.
            let back = font
                .render("Press enter to go back")
                .blended(Color::RGBA(255, 255, 255, 255))
                .unwrap();
            let back_tex = texture_creator.create_texture_from_surface(&back).unwrap();
            canvas
                .copy(
                    &back_tex,
                    None,
                    Rect::new(
                        0,
                        self.camera.height - back.size().1 as i32 * 2,
                        back.size().0 * 2,
                        back.size().1 * 2,
                    ),
                )
                .unwrap();
        }
        canvas.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        self.load_level(String::from(&self.level_path), context);
        self.time = Instant::now();
    }

    fn on_exit(&mut self, context: &mut Context) {
        self.flames.clear();
    }

    fn get_name(&mut self) -> String {
        String::from("game")
    }
}

impl/*<'a>*/ GameState/*<'_>*/ {
    pub fn load_level(&mut self, level: String, context: &mut Context) {
        context.font_manager.load(&context.font_details).unwrap();
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
                    flame.x = curx + TILE_WIDTH as i32 / 2 - flame.width as i32 / 2;
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
