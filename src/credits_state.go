package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type CreditsState struct {
	font rl.Font
}

func NewCreditsState() *CreditsState {
	return &CreditsState{}
}

func (s *CreditsState) OnEnter(gc *GameContext) error {
	s.font = loadUIFont()
	return nil
}

func (s *CreditsState) Update(gc *GameContext, dt float32) (State, error) {
	if rl.IsKeyPressed(rl.KeyEnter) {
		return NewStartMenuState(2), nil
	}
	return nil, nil
}

func (s *CreditsState) Draw(gc *GameContext) error {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.Black)

	drawCenteredText(s.font, "Credits", 48, 30, rl.White)
	drawCenteredText(s.font, "Developed by yam-head", 36, 200, rl.White)
	drawCenteredText(s.font, "Music by Kevin MacLeod", 36, 300, rl.White)
	drawBottomLeftText(s.font, "Press enter to return to the menu.", 24, 10, rl.White)
	return nil
}

func (s *CreditsState) OnExit(gc *GameContext) error {
	unloadUIFont(&s.font)
	return nil
}
