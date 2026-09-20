package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type Player struct {
	width      int32
	height     int32
	pos        rl.Vector2
	vel        rl.Vector2
	xrect      int32
	delay      float32
	frame      int32
	activeAnim string
	animations map[string][]rl.Rectangle
}

func NewPlayer() *Player {
	p := &Player{
		width:      36,
		height:     60,
		pos:        rl.NewVector2(0, 0),
		vel:        rl.NewVector2(0, 0),
		xrect:      0,
		delay:      13.0 / 60.0,
		frame:      0,
		activeAnim: "walk_down",
		animations: generateAnimations(),
	}
	return p
}

func generateAnimations() map[string][]rl.Rectangle {
	animations := make(map[string][]rl.Rectangle)

	rect := func(x, y, w, h float32) rl.Rectangle {
		return rl.NewRectangle(x, y, w, h)
	}

	down := []rl.Rectangle{
		rect(18, 0, 9, 15),
		rect(9, 0, 9, 15),
		rect(27, 0, 9, 15),
		rect(9, 0, 9, 15),
	}
	up := []rl.Rectangle{
		rect(9, 15, 9, 15),
		rect(0, 15, 9, 15),
		rect(18, 15, 9, 15),
		rect(0, 15, 9, 15),
	}
	left := []rl.Rectangle{
		rect(9, 30, 9, 15),
		rect(0, 30, 9, 15),
		rect(18, 30, 9, 15),
		rect(0, 30, 9, 15),
	}
	right := []rl.Rectangle{
		rect(9, 30, 9, 15),
		rect(0, 30, 9, 15),
		rect(18, 30, 9, 15),
		rect(0, 30, 9, 15),
	}
	downIdle := []rl.Rectangle{rect(0, 0, 9, 15)}
	upIdle := []rl.Rectangle{rect(0, 15, 9, 15)}
	leftIdle := []rl.Rectangle{rect(27, 15, 9, 15)}
	rightIdle := []rl.Rectangle{rect(27, 15, 9, 15)}

	animations["walk_up"] = up
	animations["walk_down"] = down
	animations["walk_left"] = left
	animations["walk_right"] = right
	animations["idle_up"] = upIdle
	animations["idle_down"] = downIdle
	animations["idle_left"] = leftIdle
	animations["idle_right"] = rightIdle

	return animations
}

func (p *Player) Update(dt float32) {
	prevAnim := p.activeAnim

	leftPressed := rl.IsKeyDown(rl.KeyLeft) || rl.IsKeyDown(rl.KeyA)
	rightPressed := rl.IsKeyDown(rl.KeyRight) || rl.IsKeyDown(rl.KeyD)
	upPressed := rl.IsKeyDown(rl.KeyUp) || rl.IsKeyDown(rl.KeyW)
	downPressed := rl.IsKeyDown(rl.KeyDown) || rl.IsKeyDown(rl.KeyS)

	left := leftPressed && !rightPressed
	right := rightPressed && !leftPressed
	up := upPressed && !downPressed
	down := downPressed && !upPressed

	if left {
		p.vel.X -= 225.0 * dt
		if p.activeAnim != "walk_left" {
			p.activeAnim = "walk_left"
		}
	} else if right {
		p.vel.X += 225.0 * dt
		if p.activeAnim != "walk_right" {
			p.activeAnim = "walk_right"
		}
	}

	if up {
		p.vel.Y -= 225.0 * dt
		if p.activeAnim != "walk_up" {
			p.activeAnim = "walk_up"
		}
	} else if down {
		p.vel.Y += 225.0 * dt
		if p.activeAnim != "walk_down" {
			p.activeAnim = "walk_down"
		}
	}

	if !(left || right || up || down) {
		switch p.activeAnim {
		case "walk_left":
			p.activeAnim = "idle_left"
		case "walk_right":
			p.activeAnim = "idle_right"
		case "walk_up":
			p.activeAnim = "idle_up"
		case "walk_down":
			p.activeAnim = "idle_down"
		}
	}

	if p.activeAnim != prevAnim {
		p.frame = 0
		p.delay = 13.0 / 60.0
	}

	p.delay -= dt
	frames := p.animations[p.activeAnim]
	if len(frames) == 0 {
		return
	}
	if p.delay <= 0.0 {
		p.frame++
		if p.frame >= int32(len(frames)) {
			p.frame = 0
		}
		p.delay = 13.0 / 60.0
	}
}

func (p *Player) DrawShadow(texture rl.Texture2D) {
	shadowWidth := float32(texture.Width)
	shadowHeight := float32(texture.Height)
	if shadowWidth == 0 || shadowHeight == 0 {
		return
	}
	src := rl.NewRectangle(0, 0, shadowWidth, shadowHeight)
	dst := rl.NewRectangle(
		p.pos.X,
		p.pos.Y+24,
		float32(p.width),
		float32(p.height),
	)
	rl.DrawTexturePro(texture, src, dst, rl.NewVector2(0, 0), 0, rl.White)
}

func (p *Player) Draw(texture rl.Texture2D) {
	frames := p.animations[p.activeAnim]
	if len(frames) == 0 {
		return
	}
	frameIdx := int(p.frame)
	if frameIdx < 0 || frameIdx >= len(frames) {
		frameIdx = 0
	}

	src := frames[frameIdx]
	if p.activeAnim == "walk_left" || p.activeAnim == "idle_left" {
		src.Width = -src.Width
	}

	dst := rl.NewRectangle(
		p.pos.X,
		p.pos.Y,
		float32(p.width),
		float32(p.height),
	)

	rl.DrawTexturePro(texture, src, dst, rl.NewVector2(0, 0), 0, rl.White)
}
