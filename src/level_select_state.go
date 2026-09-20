package mindseye

import (
	"bufio"
	"os"
	"sort"
	"strings"

	rl "github.com/gen2brain/raylib-go/raylib"
)

type levelOption struct {
	name string
	path string
}

type LevelSelectState struct {
	options        []levelOption
	selectedOption int
	font           rl.Font
}

func NewLevelSelectState() *LevelSelectState {
	return &LevelSelectState{}
}

func (s *LevelSelectState) OnEnter(gc *GameContext) error {
	s.font = loadUIFont()
	entries, err := os.ReadDir("res/levels")
	if err != nil {
		return err
	}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".txt") {
			continue
		}
		path := "res/levels/" + entry.Name()
		file, err := os.Open(path)
		if err != nil {
			return err
		}
		scanner := bufio.NewScanner(file)
		name := entry.Name()
		if scanner.Scan() {
			name = scanner.Text()
		}
		file.Close()
		s.options = append(s.options, levelOption{name: name, path: path})
	}
	sort.Slice(s.options, func(i, j int) bool {
		return s.options[i].name < s.options[j].name
	})
	return nil
}

func (s *LevelSelectState) Update(gc *GameContext, dt float32) (State, error) {
	if len(s.options) == 0 {
		if rl.IsKeyPressed(rl.KeyEnter) || rl.IsKeyPressed(rl.KeyB) {
			return NewStartMenuState(0), nil
		}
		return nil, nil
	}
	if rl.IsKeyPressed(rl.KeyDown) {
		if s.selectedOption == -1 {
			s.selectedOption = 0
		} else {
			s.selectedOption = (s.selectedOption + 1) % len(s.options)
		}
	} else if rl.IsKeyPressed(rl.KeyUp) {
		if s.selectedOption <= 0 {
			s.selectedOption = len(s.options) - 1
		} else {
			s.selectedOption--
		}
	} else if rl.IsKeyPressed(rl.KeyLeft) {
		s.selectedOption = -1
	} else if rl.IsKeyPressed(rl.KeyRight) && s.selectedOption == -1 {
		s.selectedOption = 0
	} else if rl.IsKeyPressed(rl.KeyB) {
		return NewStartMenuState(0), nil
	}

	if rl.IsKeyPressed(rl.KeyEnter) {
		if s.selectedOption == -1 {
			return NewStartMenuState(0), nil
		}
		if len(s.options) > 0 {
			return NewGameState(s.options[s.selectedOption].path), nil
		}
	}
	return nil, nil
}

func (s *LevelSelectState) Draw(gc *GameContext) error {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.Black)

	drawCenteredText(s.font, "Level Select", 48, 30, rl.White)
	for i, option := range s.options {
		y := float32(200 + 50*(i-1))
		x := int32(rl.GetScreenWidth()/2 - 250)
		if s.selectedOption == i {
			rl.DrawRectangle(x, int32(y), 500, 50, rl.White)
		}
		color := rl.White
		if s.selectedOption == i {
			color = rl.Black
		}
		drawCenteredTextAt(s.font, option.name, 24, y+12, color)
		rl.DrawRectangleLines(x, int32(y), 500, 50, color)
	}

	backColor := rl.White
	if s.selectedOption == -1 {
		rl.DrawRectangle(0, int32(rl.GetScreenHeight()-50), 100, 50, rl.White)
		backColor = rl.Black
	}
	backText := "< Back"
	backSize := float32(24)
	backHeight := rl.MeasureTextEx(s.font, backText, backSize, 0).Y
	rl.DrawTextEx(
		s.font,
		backText,
		rl.NewVector2(0, float32(rl.GetScreenHeight())-10-backHeight),
		backSize,
		0,
		backColor,
	)
	return nil
}

func drawCenteredTextAt(font rl.Font, text string, size, y float32, color rl.Color) {
	width := rl.MeasureTextEx(font, text, size, 0).X
	rl.DrawTextEx(font, text, rl.NewVector2(float32(rl.GetScreenWidth())/2-width/2, y), size, 0, color)
}

func (s *LevelSelectState) OnExit(gc *GameContext) error {
	unloadUIFont(&s.font)
	return nil
}
