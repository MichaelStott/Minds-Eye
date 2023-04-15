use barn::math::vector2::Vector2;
use crate::game::player::Player;
use crate::game::tile::Tile;

use sdl2::mixer::Chunk;

pub fn handle_collisions(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk, dt: f32) {
    handle_collision_x(player, tiles, move_fx, dt);
    handle_collision_y(player, tiles, move_fx, dt);
}

pub fn does_intersect(player: &mut Player, tile: &mut Tile) -> bool {
    (player.pos.x < (tile.bb.origin.x + tile.bb.width as f32) as f32)
        && (player.pos.x + player.width as f32 > tile.bb.origin.x as f32)
        && (player.pos.y + 25.0 < (tile.bb.origin.y + tile.bb.height as f32) as f32)
        && (player.pos.y + player.height as f32 > tile.bb.origin.y as f32)
}

fn handle_collision_x(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk, dt: f32) {
    if player.vel.x != 0.0 {
        player.pos += Vector2{x: player.vel.x, y: 0.0};
        for tile in tiles {
            if tile.isblock || tile.iswall {
                if does_intersect(player, tile) {
                    let dir: f32 = if player.vel.x > 0.0 { -1.0 } else { 1.0 };
                    if tile.isblock && !tile.moving {
                        if tile.resistance <= 0.0 {
                            tile.target_pos.x = dir * -1.0 * tile.bb.width as f32 + tile.bb.origin.x;
                            tile.resistance = 30.0;
                        } else {
                            tile.resistance -= dt * 200.0;
                        }
                    }
                    player.pos.x = if dir == 1.0 {
                        (tile.bb.origin.x + tile.bb.width as f32) as f32
                    } else {
                        (tile.bb.origin.x - player.width as f32) as f32
                    };
                } else {
                    tile.resistance = 30.0;
                }
            }
        }
        player.vel.x = 0.0;
    }
}

fn handle_collision_y(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk, dt: f32) {
    if player.vel.y != 0.0 {
        player.pos += Vector2{x: 0.0, y: player.vel.y};
        for tile in tiles {
            if tile.isblock || tile.iswall {
                if does_intersect(player, tile) {
                    let dir: f32 = if player.vel.y > 0.0 { -1.0 } else { 1.0 };
                    if tile.isblock && !tile.moving {
                        if tile.resistance <= 0.0 {
                            tile.target_pos.y = dir * -1.0 * tile.bb.height as f32 + tile.bb.origin.y;
                            tile.resistance = 30.0;
                        } else {
                            tile.resistance -= dt * 200.0;
                        }
                    }
                    player.pos.y = if dir == 1.0 {
                        -25.0 + (tile.bb.origin.y + tile.bb.height as f32) as f32
                    } else {
                        (tile.bb.origin.y - player.height as f32) as f32
                    };
                } else {
                    tile.resistance = 30.0;
                }
            }
        }
        player.vel.y = 0.0;
    }
}
