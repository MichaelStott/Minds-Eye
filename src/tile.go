package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type Tile struct {
	texture    string
	bb         rl.Rectangle
	targetPos  rl.Vector2
	resistance float32
	isWall     bool
	isBlock    bool
	isMoving   bool
}

func (t *Tile) Update(tiles []Tile, dt float32) {
	prevPos := t.bb
	const speed = float32(200)

	if t.targetPos != rl.NewVector2(t.bb.X, t.bb.Y) {
		diff := rl.NewVector2(t.targetPos.X-t.bb.X, t.targetPos.Y-t.bb.Y)
		distance := float32(rl.Vector2Length(diff))
		if speed*dt < distance {
			t.isMoving = true
			t.bb.X += diff.X / distance * speed * dt
			t.bb.Y += diff.Y / distance * speed * dt
		} else {
			t.isMoving = false
			t.bb.X = t.targetPos.X
			t.bb.Y = t.targetPos.Y
		}
	}

	for i := range tiles {
		other := &tiles[i]
		if (!other.isWall && !other.isBlock) ||
			(other.bb.X == t.bb.X && other.bb.Y == t.bb.Y) ||
			(other.bb.X == prevPos.X && other.bb.Y == prevPos.Y) {
			continue
		}
		if rectanglesIntersect(t.bb, other.bb) {
			t.resistance = 30
			t.bb.X = prevPos.X
			t.bb.Y = prevPos.Y
			t.targetPos = rl.NewVector2(prevPos.X, prevPos.Y)
			t.isMoving = false
		}
	}

}

func rectanglesIntersect(a, b rl.Rectangle) bool {
	return a.X < b.X+b.Width &&
		a.X+a.Width > b.X &&
		a.Y < b.Y+b.Height &&
		a.Y+a.Height > b.Y
}

func (t *Tile) HasMoved() bool {
	return absFloat(t.targetPos.X-t.bb.X) == t.bb.Width ||
		absFloat(t.targetPos.Y-t.bb.Y) == t.bb.Height
}

func absFloat(value float32) float32 {
	if value < 0 {
		return -value
	}
	return value
}

func (t *Tile) Draw(texture rl.Texture2D) {
	source := rl.NewRectangle(
		0,
		0,
		float32(texture.Width),
		float32(texture.Height),
	)
	destination := rl.NewRectangle(
		t.bb.X,
		t.bb.Y,
		t.bb.Width,
		t.bb.Height,
	)
	rl.DrawTexturePro(
		texture,
		source,
		destination,
		rl.NewVector2(0, 0),
		0,
		rl.White,
	)

	if Debug && (t.isWall || t.isBlock) {
		color := rl.LightGray
		if t.isBlock {
			color = rl.Red
		}
		rl.DrawRectangleLines(
			int32(destination.X),
			int32(destination.Y),
			int32(destination.Width),
			int32(destination.Height),
			color,
		)
	}
}
