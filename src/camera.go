package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

// focusCamera keeps focus mapped to the camera offset while preventing the
// viewport from moving outside the level bounds.
func focusCamera(camera *rl.Camera2D, focus, min, max rl.Vector2) {
	screenWidth := float32(rl.GetScreenWidth())
	screenHeight := float32(rl.GetScreenHeight())
	renderWidth := float32(rl.GetRenderWidth())
	renderHeight := float32(rl.GetRenderHeight())
	scaleX := renderWidth / screenWidth
	scaleY := renderHeight / screenHeight
	if scaleX <= 0 {
		scaleX = 1
	}
	if scaleY <= 0 {
		scaleY = 1
	}

	camera.Offset = rl.NewVector2(
		renderWidth/2,
		renderHeight/2,
	)
	camera.Zoom = (scaleX + scaleY) / 2

	halfWidth := camera.Offset.X / camera.Zoom
	halfHeight := camera.Offset.Y / camera.Zoom

	camera.Target = focus
	camera.Target.X = clampCameraAxis(camera.Target.X, min.X+halfWidth, max.X-halfWidth)
	camera.Target.Y = clampCameraAxis(camera.Target.Y, min.Y+halfHeight, max.Y-halfHeight)
}

func clampCameraAxis(value, min, max float32) float32 {
	if min > max {
		return (min + max) / 2
	}
	if value < min {
		return min
	}
	if value > max {
		return max
	}
	return value
}
