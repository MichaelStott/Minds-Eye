package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type Fire struct {
	pos    rl.Vector2
	width  float32
	height float32
	delay  int
	frame  int
}

func NewFire() Fire {
	return Fire{
		width:  24,
		height: 42,
		delay:  10,
		frame:  1,
	}
}

func (f *Fire) Update() {
	f.delay--
	if f.delay <= 0 {
		f.frame++
		if f.frame >= 2 {
			f.frame = 0
		}
		f.delay = 5
	}
}

func (f *Fire) Draw(flameTexture, glowTexture rl.Texture2D) {
	rl.DrawTexturePro(
		flameTexture,
		rl.NewRectangle(float32(f.frame*8), 0, 8, 14),
		rl.NewRectangle(f.pos.X, f.pos.Y, f.width, f.height),
		rl.NewVector2(0, 0),
		0,
		rl.White,
	)
	rl.DrawTexturePro(
		glowTexture,
		rl.NewRectangle(float32(f.frame*32), 0, 32, 32),
		rl.NewRectangle(f.pos.X-20, f.pos.Y-4, 64, 64),
		rl.NewVector2(0, 0),
		0,
		rl.White,
	)
}
