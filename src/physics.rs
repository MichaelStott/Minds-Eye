use sdl2::mixer::Chunk;
use crate::player::Player;
use crate::tile::Tile;

pub fn handle_collisions(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk) {
    handle_collision_x(player, tiles, move_fx);
    handle_collision_y(player, tiles, move_fx);
}

pub fn does_intersect(player: &mut Player, tile: &mut Tile) -> bool {
    (player.x < tile.x + tile.width as i32)
        && (player.x + player.width as i32 > tile.x)
        && (player.y + 25 < tile.y + tile.height as i32)
        && (player.y + player.height as i32 > tile.y)
}

fn handle_collision_x(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk) {
    if player.velx != 0 {
        player.move_player(player.velx, 0);
        for tile in tiles {
            if tile.isblock || tile.iswall {
                if does_intersect(player, tile) {
                    let dir: i32 = if player.velx > 0 { -1 } else { 1 };
                    if tile.isblock {
                        if tile.resistancex <= 0 {
                            tile.targetx = dir * -1 * tile.width as i32 + tile.x;
                            tile.resistancex = 30;
                            tile.resistancey = 30;
                        } else {
                            tile.resistancex -= 1;
                        }
                    }
                    player.x = if dir == 1 {
                        tile.x + tile.width as i32
                    } else {
                        tile.x - player.width as i32
                    };
                } else {
                    tile.resistancex = 30;
                }
            }
        }
        player.velx = 0;
    }
}

fn handle_collision_y(player: &mut Player, tiles: &mut Vec<Tile>, move_fx: &Chunk) {
    if player.vely != 0 {
        player.move_player(0, player.vely);
        for tile in tiles {
            if tile.isblock || tile.iswall {
                if does_intersect(player, tile) {
                    let dir: i32 = if player.vely > 0 { -1 } else { 1 };
                    if tile.isblock {
                        if tile.resistancey <= 0 {
                            tile.targety = dir * -1 * tile.height as i32 + tile.y;
                            tile.resistancex = 30;
                            tile.resistancey = 30;
                        } else {
                            tile.resistancey -= 1;
                        }
                    }
                    player.y = if dir == 1 {
                        tile.y - 25  + tile.height as i32
                    } else {
                        tile.y - player.height as i32
                    };
                } else {
                    tile.resistancey = 30;
                }
            }
        }
        player.vely = 0;
    }
}
