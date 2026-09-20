package main

import (
	mindseye "mindseye/src"

	rl "github.com/gen2brain/raylib-go/raylib"
)

func main() {
	// Configure HiDPI before creating the window so raylib creates the
	// correct render target from the start.
	rl.SetConfigFlags(rl.FlagWindowResizable | rl.FlagWindowHighdpi)
	rl.InitWindow(800, 600, "Minds Eye")
	defer rl.CloseWindow()

	gc := &mindseye.GameContext{}
	var currentState mindseye.State
	currentState = mindseye.NewStartMenuState(0)
	currentState.OnEnter(gc)

	for !rl.WindowShouldClose() {
		dt := rl.GetFrameTime()
		newState, err := currentState.Update(gc, dt)
		if err != nil {
			panic(err)
		}
		if newState != nil {
			currentState.OnExit(gc)
			currentState = newState
			currentState.OnEnter(gc)
		}

		err = currentState.Draw(gc)
		if err != nil {
			panic(err)
		}
	}
}
