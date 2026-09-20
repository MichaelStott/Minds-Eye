package mindseye

func DoesIntersect(player *Player, tile *Tile) bool {
	return player.pos.X < tile.bb.X+tile.bb.Width &&
		player.pos.X+float32(player.width) > tile.bb.X &&
		player.pos.Y+25 < tile.bb.Y+tile.bb.Height &&
		player.pos.Y+float32(player.height) > tile.bb.Y
}

func HandleCollisions(player *Player, tiles []Tile, dt float32) {
	handleCollisionX(player, tiles, dt)
	handleCollisionY(player, tiles, dt)
}

func handleCollisionX(player *Player, tiles []Tile, dt float32) {
	if player.vel.X == 0 {
		return
	}

	player.pos.X += player.vel.X
	for i := range tiles {
		tile := &tiles[i]
		if !tile.isBlock && !tile.isWall {
			continue
		}
		if DoesIntersect(player, tile) {
			dir := float32(1)
			if player.vel.X > 0 {
				dir = -1
			}
			if tile.isBlock && !tile.isMoving {
				if tile.resistance <= 0 {
					tile.targetPos.X = tile.bb.X - dir*tile.bb.Width
					tile.resistance = 30
				} else {
					tile.resistance -= dt * 200
				}
			}
			if dir == 1 {
				player.pos.X = tile.bb.X + tile.bb.Width
			} else {
				player.pos.X = tile.bb.X - float32(player.width)
			}
		} else {
			tile.resistance = 30
		}
	}
	player.vel.X = 0
}

func handleCollisionY(player *Player, tiles []Tile, dt float32) {
	if player.vel.Y == 0 {
		return
	}

	player.pos.Y += player.vel.Y
	for i := range tiles {
		tile := &tiles[i]
		if !tile.isBlock && !tile.isWall {
			continue
		}
		if DoesIntersect(player, tile) {
			dir := float32(1)
			if player.vel.Y > 0 {
				dir = -1
			}
			if tile.isBlock && !tile.isMoving {
				if tile.resistance <= 0 {
					tile.targetPos.Y = tile.bb.Y - dir*tile.bb.Height
					tile.resistance = 30
				} else {
					tile.resistance -= dt * 200
				}
			}
			if dir == 1 {
				player.pos.Y = -25 + tile.bb.Y + tile.bb.Height
			} else {
				player.pos.Y = tile.bb.Y - float32(player.height)
			}
		} else {
			tile.resistance = 30
		}
	}
	player.vel.Y = 0
}
