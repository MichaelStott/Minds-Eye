package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type HelpState struct {
	font rl.Font
}

func NewHelpState() *HelpState {
	return &HelpState{}
}

func (s *HelpState) OnEnter(gc *GameContext) error {
	s.font = loadUIFont()
	return nil
}

func (s *HelpState) Update(gc *GameContext, dt float32) (State, error) {
	if rl.IsKeyPressed(rl.KeyEnter) {
		return NewStartMenuState(1), nil
	}
	return nil, nil
}

func (s *HelpState) Draw(gc *GameContext) error {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.Black)

	drawCenteredText(s.font, "Help", 48, 30, rl.White)
	drawCenteredText(s.font, "Movement: Arrow Keys", 36, 200, rl.White)
	drawCenteredText(s.font, "Reset Puzzle: R", 36, 300, rl.White)
	drawCenteredText(s.font, "Exit Puzzle: Q", 36, 400, rl.White)
	drawBottomLeftText(s.font, "Press enter to return to the menu.", 24, 10, rl.White)
	return nil
}

func (s *HelpState) OnExit(gc *GameContext) error {
	unloadUIFont(&s.font)
	return nil
}
