package mindseye

import rl "github.com/gen2brain/raylib-go/raylib"

type StartMenuState struct {
	selectedOption int
	tiles          []Tile
	eyes           []Eye
	textures       map[string]rl.Texture2D
	font           rl.Font
}

func NewStartMenuState(selectedOption int) *StartMenuState {
	return &StartMenuState{selectedOption: selectedOption}
}

func (s *StartMenuState) OnEnter(gc *GameContext) error {
	s.font = loadUIFont()
	s.textures = make(map[string]rl.Texture2D)
	s.eyes = []Eye{
		{pos: rl.NewVector2(150, 200), color: "blue", direction: "left"},
		{pos: rl.NewVector2(600, 300), color: "green", direction: "left"},
		{pos: rl.NewVector2(368, 500), color: "red", direction: "left"},
	}
	s.refreshPreview()
	return nil
}

func (s *StartMenuState) refreshPreview() {
	s.tiles = nil
	switch s.selectedOption {
	case 0:
		s.tiles = append(s.tiles, previewTile("res/img/blueblock.png", 200, 200, 200, 200))
	case 1:
		s.tiles = append(s.tiles, previewTile("res/img/greenblock.png", 200, 300, 200, 500))
	case 2:
		s.tiles = append(s.tiles, previewTile("res/img/redblock.png", 368, 300, 200, 200))
	}
}

func previewTile(texture string, x, y, targetX, targetY float32) Tile {
	return Tile{
		texture:    texture,
		bb:         rl.NewRectangle(x, y, TileWidth, TileHeight),
		targetPos:  rl.NewVector2(targetX, targetY),
		resistance: 30,
		isBlock:    true,
	}
}

func (s *StartMenuState) Update(gc *GameContext, dt float32) (State, error) {
	for i := range s.eyes {
		s.eyes[i].Update(s.tiles, dt)
	}

	previous := s.selectedOption
	if rl.IsKeyPressed(rl.KeyDown) {
		s.selectedOption = (s.selectedOption + 1) % 3
	} else if rl.IsKeyPressed(rl.KeyUp) {
		s.selectedOption = (s.selectedOption + 2) % 3
	}
	if previous != s.selectedOption {
		s.refreshPreview()
	}

	if rl.IsKeyPressed(rl.KeyEnter) {
		switch s.selectedOption {
		case 0:
			return NewLevelSelectState(), nil
		case 1:
			return NewHelpState(), nil
		case 2:
			return NewCreditsState(), nil
		}
	}
	return nil, nil
}

func (s *StartMenuState) Draw(gc *GameContext) error {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.Black)

	drawCenteredText(s.font, "Mind's Eye", 48, 30, rl.White)

	colors := []rl.Color{rl.NewColor(0, 0, 180, 255), rl.NewColor(0, 180, 0, 255), rl.NewColor(180, 0, 0, 255)}
	for i := 0; i < 3; i++ {
		y := int32(200 + i*100)
		x := int32(rl.GetScreenWidth()/2 - 125)
		if i == s.selectedOption {
			rl.DrawRectangle(x, y, 250, 65, colors[i])
		}
		rl.DrawRectangleLines(x, y, 250, 65, rl.White)
	}

	buttonX := int32(rl.GetScreenWidth()/2 - 125)
	drawCenteredTextInBox(s.font, "Play", 36, buttonX, 200, 250, 65, rl.White)
	drawCenteredTextInBox(s.font, "Help", 36, buttonX, 300, 250, 65, rl.White)
	drawCenteredTextInBox(s.font, "Credits", 36, buttonX, 400, 250, 65, rl.White)

	socket := s.texture("res/img/socket.png")
	for _, eye := range s.eyes {
		var pupilPath string
		switch eye.color {
		case "red":
			pupilPath = "res/img/redpupil.png"
		case "green":
			pupilPath = "res/img/greenpupil.png"
		default:
			pupilPath = "res/img/bluepupil.png"
		}
		eye.Draw(socket, s.texture(pupilPath))
	}
	return nil
}

func (s *StartMenuState) texture(path string) rl.Texture2D {
	texture, ok := s.textures[path]
	if !ok {
		texture = rl.LoadTexture(path)
		rl.SetTextureFilter(texture, rl.FilterPoint)
		s.textures[path] = texture
	}
	return texture
}

func (s *StartMenuState) OnExit(gc *GameContext) error {
	for path, texture := range s.textures {
		if texture.ID != 0 {
			rl.UnloadTexture(texture)
		}
		delete(s.textures, path)
	}
	unloadUIFont(&s.font)
	return nil
}
