package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

const uiFontPath = "res/fonts/VeniceClassic.ttf"

func loadUIFont() rl.Font {
	font := rl.LoadFont(uiFontPath)
	rl.SetTextureFilter(font.Texture, rl.FilterPoint)
	return font
}

func drawCenteredText(font rl.Font, text string, size, y float32, color rl.Color) {
	width := rl.MeasureTextEx(font, text, size, 0).X
	rl.DrawTextEx(font, text, rl.NewVector2(float32(rl.GetScreenWidth())/2-width/2, y), size, 0, color)
}

func drawCenteredTextInBox(font rl.Font, text string, size float32, x, y, width, height int32, color rl.Color) {
	measure := rl.MeasureTextEx(font, text, size, 0)
	rl.DrawTextEx(
		font,
		text,
		rl.NewVector2(
			float32(x)+float32(width)/2-measure.X/2,
			float32(y)+float32(height)/2-measure.Y/2,
		),
		size,
		0,
		color,
	)
}

func drawBottomLeftText(font rl.Font, text string, size, margin float32, color rl.Color) {
	height := rl.MeasureTextEx(font, text, size, 0).Y
	rl.DrawTextEx(
		font,
		text,
		rl.NewVector2(margin, float32(rl.GetScreenHeight())-margin-height),
		size,
		0,
		color,
	)
}

func unloadUIFont(font *rl.Font) {
	if font.Texture.ID != 0 {
		rl.UnloadFont(*font)
		*font = rl.Font{}
	}
}
