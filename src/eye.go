package mindseye

import (
	"math/rand"
	"strings"

	rl "github.com/gen2brain/raylib-go/raylib"
)

const eyeSize = float32(TileWidth)

type Eye struct {
	pos       rl.Vector2
	color     string
	solved    bool
	direction string
	deltaX    float32
	deltaY    float32
	anger     uint8
}

func (e *Eye) Update(tiles []Tile, dt float32) {
	e.solved = false
	distance := float32(-1)
	isBlock := false

	// Look for the closest wall or block in each direction. The search order
	// matches the original implementation: left, right, up, then down.
	for _, tile := range tiles {
		if tile.bb.Y == e.pos.Y && e.pos.X > tile.bb.X && (tile.isBlock || tile.isWall) {
			if distance == -1 || e.pos.X-tile.bb.X < distance {
				distance = e.pos.X - tile.bb.X
				isBlock = tile.isBlock && strings.Contains(tile.texture, e.color)
				e.direction = "left"
			}
		}
	}
	if !isBlock {
		distance = -1
		for _, tile := range tiles {
			if tile.bb.Y == e.pos.Y && e.pos.X < tile.bb.X && (tile.isBlock || tile.isWall) {
				if distance == -1 || tile.bb.X-e.pos.X < distance {
					distance = tile.bb.X - e.pos.X
					isBlock = tile.isBlock && strings.Contains(tile.texture, e.color)
					e.direction = "right"
				}
			}
		}
	}
	if !isBlock {
		distance = -1
		for _, tile := range tiles {
			if tile.bb.X == e.pos.X && e.pos.Y < tile.bb.Y && (tile.isBlock || tile.isWall) {
				if distance == -1 || tile.bb.Y-e.pos.Y < distance {
					distance = tile.bb.Y - e.pos.Y
					isBlock = tile.isBlock && strings.Contains(tile.texture, e.color)
					e.direction = "up"
				}
			}
		}
	}
	if !isBlock {
		distance = -1
		for _, tile := range tiles {
			if tile.bb.X == e.pos.X && e.pos.Y > tile.bb.Y && (tile.isBlock || tile.isWall) {
				if distance == -1 || e.pos.Y-tile.bb.Y < distance {
					distance = e.pos.Y - tile.bb.Y
					isBlock = tile.isBlock && strings.Contains(tile.texture, e.color)
					e.direction = "down"
				}
			}
		}
	}

	if isBlock {
		e.solved = true
		switch e.direction {
		case "left":
			e.deltaX = maxFloat(e.deltaX-dt*100, -12)
			e.deltaY = 0
		case "right":
			e.deltaX = minFloat(e.deltaX+dt*100, 12)
			e.deltaY = 0
		case "up":
			e.deltaY = minFloat(e.deltaY+dt*100, 6)
			e.deltaX = 0
		case "down":
			e.deltaY = maxFloat(e.deltaY-dt*100, -6)
			e.deltaX = 0
		}
		if e.anger > 251 {
			e.anger = 255
		} else {
			e.anger += 4
		}
		return
	}

	// An unsolved eye jitters and gradually calms down.
	e.deltaX = rand.Float32()*6 - 3
	e.deltaY = rand.Float32()*6 - 3
	if e.anger > 10 {
		e.anger -= 10
	} else {
		e.anger = 0
	}
}

func (e *Eye) Draw(socket, pupil rl.Texture2D) {
	tint := rl.White
	switch e.color {
	case "red":
		tint = rl.NewColor(255, e.anger, e.anger, 255)
	case "green":
		tint = rl.NewColor(e.anger, 255, e.anger, 255)
	case "blue":
		tint = rl.NewColor(e.anger, e.anger, 255, 255)
	}

	source := rl.NewRectangle(0, 0, float32(socket.Width), float32(socket.Height))
	destination := rl.NewRectangle(e.pos.X, e.pos.Y, eyeSize, eyeSize)
	rl.DrawTexturePro(socket, source, destination, rl.NewVector2(0, 0), 0, tint)

	source = rl.NewRectangle(0, 0, float32(pupil.Width), float32(pupil.Height))
	destination.X += e.deltaX
	destination.Y += e.deltaY
	rl.DrawTexturePro(pupil, source, destination, rl.NewVector2(0, 0), 0, rl.White)
}

func minFloat(a, b float32) float32 {
	if a < b {
		return a
	}
	return b
}

func maxFloat(a, b float32) float32 {
	if a > b {
		return a
	}
	return b
}
