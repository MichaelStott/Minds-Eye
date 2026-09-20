package mindseye

import (
	"bufio"
	"fmt"
	"os"
	"time"

	rl "github.com/gen2brain/raylib-go/raylib"
)

const TileWidth = 64
const TileHeight = 64

var TileChars = [6]string{"*", "5", "5", "6", "8", "9"}

type GameState struct {
	levelPath      string
	player         *Player
	camera         rl.Camera2D
	cameraMin      rl.Vector2
	cameraMax      rl.Vector2
	playerTexture  rl.Texture2D
	shadowTexture  rl.Texture2D
	completionFont rl.Font
	tileTextures   map[string]rl.Texture2D
	tiles          []Tile
	flames         []Fire
	eyes           []Eye
	won            bool
	moves          int
	startTime      time.Time
	solveTime      int64
}

func NewGameState(levelPath string) *GameState {
	return &GameState{levelPath: levelPath}
}

func (gs *GameState) Update(gc *GameContext, dt float32) (State, error) {
	if gs.player == nil {
		return nil, nil
	}
	if rl.IsKeyPressed(rl.KeyQ) {
		return NewLevelSelectState(), nil
	}
	if rl.IsKeyPressed(rl.KeyR) {
		if err := gs.loadLevel(gc, gs.levelPath); err != nil {
			return nil, err
		}
		return nil, nil
	}

	for i := range gs.flames {
		gs.flames[i].Update()
	}

	tileSnapshot := append([]Tile(nil), gs.tiles...)
	for i := range gs.tiles {
		if gs.tiles[i].HasMoved() {
			gs.moves++
		}
		gs.tiles[i].Update(tileSnapshot, dt)
	}

	for i := range gs.eyes {
		gs.eyes[i].Update(gs.tiles, dt)
	}

	gs.won = true
	for _, eye := range gs.eyes {
		if !eye.solved {
			gs.won = false
			break
		}
	}
	if !gs.won {
		gs.player.Update(dt)
		HandleCollisions(gs.player, gs.tiles, dt)
		gs.focusCamera()
	} else if gs.startTime.IsZero() {
		gs.startTime = time.Now()
	} else if gs.solveTime == 0 {
		gs.solveTime = int64(time.Since(gs.startTime).Seconds())
	} else if rl.IsKeyPressed(rl.KeyEnter) {
		return NewLevelSelectState(), nil
	}

	return nil, nil
}

func (gs *GameState) Draw(gc *GameContext) error {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.Black)
	rl.BeginMode2D(gs.camera)

	if gs.tileTextures == nil {
		gs.tileTextures = make(map[string]rl.Texture2D)
	}
	for _, tile := range gs.tiles {
		if tile.isWall || tile.isBlock {
			continue
		}
		gs.drawTile(tile)
	}
	for _, tile := range gs.tiles {
		if !tile.isWall && !tile.isBlock {
			continue
		}
		gs.drawTile(tile)
	}

	for _, eye := range gs.eyes {
		var pupil string
		switch eye.color {
		case "red":
			pupil = "res/img/redpupil.png"
		case "green":
			pupil = "res/img/greenpupil.png"
		default:
			pupil = "res/img/bluepupil.png"
		}
		socketTexture := gs.getLevelTexture("res/img/socket.png")
		pupilTexture := gs.getLevelTexture(pupil)
		eye.Draw(socketTexture, pupilTexture)
	}

	if gs.player != nil {
		gs.player.DrawShadow(gs.shadowTexture)
		gs.player.Draw(gs.playerTexture)
	}

	flameTexture := gs.getLevelTexture("res/img/fire2.png")
	glowTexture := gs.getLevelTexture("res/img/fire_glow.png")
	for i := range gs.flames {
		gs.flames[i].Draw(flameTexture, glowTexture)
	}

	rl.EndMode2D()
	if gs.won {
		gs.drawCompletionOverlay()
	}

	return nil
}

func (gs *GameState) drawCompletionOverlay() {
	width := int32(rl.GetScreenWidth())
	height := int32(rl.GetScreenHeight())
	rl.DrawRectangle(0, 0, width, height, rl.NewColor(0, 0, 0, 150))

	title := "Solved!"
	titleSize := int32(48)
	titlePosition := centeredTextPosition(gs.completionFont, title, float32(titleSize), width, height/6)
	rl.DrawTextEx(gs.completionFont, title, titlePosition, float32(titleSize), 0, rl.White)

	timeText := fmt.Sprintf("Time: %d seconds", gs.solveTime)
	movesText := fmt.Sprintf("Moves taken: %d", gs.moves)
	bodySize := float32(28)
	rl.DrawTextEx(gs.completionFont, timeText, centeredTextPosition(gs.completionFont, timeText, bodySize, width, height/2), bodySize, 0, rl.White)
	rl.DrawTextEx(gs.completionFont, movesText, centeredTextPosition(gs.completionFont, movesText, bodySize, width, height*3/5), bodySize, 0, rl.White)

	prompt := "Press enter to go back"
	promptSize := float32(22)
	rl.DrawTextEx(gs.completionFont, prompt, centeredTextPosition(gs.completionFont, prompt, promptSize, width, height*9/10), promptSize, 0, rl.White)
}

func centeredTextPosition(font rl.Font, text string, size float32, screenWidth, y int32) rl.Vector2 {
	measure := rl.MeasureTextEx(font, text, size, 0)
	return rl.NewVector2(float32(screenWidth)/2-measure.X/2, float32(y))
}

func (gs *GameState) OnEnter(gc *GameContext) error {
	gs.player = NewPlayer()
	gs.player.pos = rl.NewVector2(400, 300)
	gs.camera = rl.NewCamera2D(
		rl.NewVector2(400, 300),
		rl.NewVector2(0, 0),
		0,
		1,
	)
	gs.cameraMin = rl.NewVector2(0, 0)
	gs.cameraMax = rl.NewVector2(0, 0)

	gs.playerTexture = rl.LoadTexture("res/img/player.png")
	rl.SetTextureFilter(gs.playerTexture, rl.FilterPoint)

	gs.shadowTexture = rl.LoadTexture("res/img/drop_shadow.png")
	rl.SetTextureFilter(gs.shadowTexture, rl.FilterPoint)
	gs.completionFont = rl.LoadFont("res/fonts/VeniceClassic.ttf")
	rl.SetTextureFilter(gs.completionFont.Texture, rl.FilterPoint)

	levelPath := gs.levelPath
	if levelPath == "" {
		levelPath = "res/levels/level1.txt"
	}
	gs.levelPath = levelPath
	return gs.loadLevel(gc, levelPath)
}

func (gs *GameState) OnExit(gc *GameContext) error {
	if gs.playerTexture.ID != 0 {
		rl.UnloadTexture(gs.playerTexture)
		gs.playerTexture = rl.Texture2D{}
	}

	if gs.shadowTexture.ID != 0 {
		rl.UnloadTexture(gs.shadowTexture)
		gs.shadowTexture = rl.Texture2D{}
	}
	if gs.completionFont.Texture.ID != 0 {
		rl.UnloadFont(gs.completionFont)
		gs.completionFont = rl.Font{}
	}
	for path, texture := range gs.tileTextures {
		if texture.ID != 0 {
			rl.UnloadTexture(texture)
		}
		delete(gs.tileTextures, path)
	}

	return nil
}

func (gs *GameState) getLevelTexture(path string) rl.Texture2D {
	texture, ok := gs.tileTextures[path]
	if !ok {
		texture = rl.LoadTexture(path)
		rl.SetTextureFilter(texture, rl.FilterPoint)
		gs.tileTextures[path] = texture
	}
	return texture
}

func (gs *GameState) drawTile(tile Tile) {
	texture := gs.getLevelTexture(tile.texture)
	tile.Draw(texture)
}

func (gs *GameState) drawLevelTexture(path string, pos rl.Vector2) {
	gs.drawAnimatedTexture(
		path,
		rl.NewRectangle(0, 0, float32(TileWidth), float32(TileHeight)),
		pos,
		rl.NewVector2(TileWidth, TileHeight),
	)
}

func (gs *GameState) drawAnimatedTexture(path string, source rl.Rectangle, pos, size rl.Vector2) {
	texture, ok := gs.tileTextures[path]
	if !ok {
		texture = rl.LoadTexture(path)
		rl.SetTextureFilter(texture, rl.FilterPoint)
		gs.tileTextures[path] = texture
	}
	destination := rl.NewRectangle(
		pos.X,
		pos.Y,
		size.X,
		size.Y,
	)
	rl.DrawTexturePro(texture, source, destination, rl.NewVector2(0, 0), 0, rl.White)
}

func (gs *GameState) loadLevel(gc *GameContext, levelName string) error {
	file, err := os.Open(levelName)
	if err != nil {
		return err
	}
	defer file.Close()

	gs.tiles = nil
	gs.flames = nil
	gs.eyes = nil
	gs.won = false
	gs.moves = 0
	gs.startTime = time.Now()
	gs.solveTime = 0
	gs.player = NewPlayer()

	scanner := bufio.NewScanner(file)
	skip := true
	const levelOrigin = int32(10)
	cury := levelOrigin
	levelMaxX := float32(0)
	for scanner.Scan() {
		if skip {
			skip = false
			continue
		}
		line := scanner.Text()
		curx := levelOrigin
		for _, char := range line {
			switch char {
			case '*', '5', '6', '8', '9':
				gs.tiles = append(gs.tiles, Tile{
					texture:    tileTextureName(char),
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
				})
				curx += TileWidth
			case 'f':
				fire := NewFire()
				fire.pos = rl.NewVector2(
					float32(curx+TileWidth/2)-fire.width/2,
					float32(cury),
				)
				gs.flames = append(gs.flames, fire)
				gs.tiles = append(gs.tiles, Tile{
					texture:    "res/img/torch.png",
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
					isWall:     true,
				})
				curx += TileWidth
			case 'x':
				gs.tiles = append(gs.tiles, Tile{
					texture:    "res/img/grayblock.png",
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
					isWall:     true,
				})
				curx += TileWidth
			case 'b', 'g', 'r':
				texture := map[rune]string{
					'b': "res/img/blueblock.png",
					'g': "res/img/greenblock.png",
					'r': "res/img/redblock.png",
				}[char]
				gs.tiles = append(gs.tiles, Tile{
					texture:    "res/img/dbg_floor.png",
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
				})
				gs.tiles = append(gs.tiles, Tile{
					texture:    texture,
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
					isBlock:    true,
				})
			case 'B', 'R', 'G':
				gs.tiles = append(gs.tiles, Tile{
					texture:    "res/img/dbg_floor.png",
					bb:         rl.NewRectangle(float32(curx), float32(cury), TileWidth, TileHeight),
					targetPos:  rl.NewVector2(float32(curx), float32(cury)),
					resistance: 30,
				})
				color := map[rune]string{'B': "blue", 'R': "red", 'G': "green"}[char]
				gs.eyes = append(gs.eyes, Eye{
					pos:       rl.NewVector2(float32(curx), float32(cury)),
					color:     color,
					direction: "left",
				})
			case 'p':
				gs.player.pos = rl.NewVector2(
					float32(curx+TileWidth/2-gs.player.width/2),
					float32(cury+3-TileHeight/2+gs.player.height/2),
				)
			case ' ':
				curx += TileWidth
			}
		}
		if float32(curx) > levelMaxX {
			levelMaxX = float32(curx)
		}
		cury += TileHeight
	}

	if err := scanner.Err(); err != nil {
		return err
	}
	gs.cameraMin = rl.NewVector2(float32(levelOrigin), float32(levelOrigin))
	gs.cameraMax = rl.NewVector2(levelMaxX, float32(cury))
	gs.focusCamera()
	return nil
}

func (gs *GameState) focusCamera() {
	if gs.player == nil {
		return
	}

	focus := rl.NewVector2(
		gs.player.pos.X+float32(gs.player.width)/2,
		gs.player.pos.Y+float32(gs.player.height)/2,
	)
	focusCamera(&gs.camera, focus, gs.cameraMin, gs.cameraMax)
}

func tileTextureName(tile rune) string {
	switch tile {
	case '8':
		return "res/img/dbg_floor_shadow_top.png"
	case '9':
		return "res/img/dbg_floor_shadow_ne_corner.png"
	case '6':
		return "res/img/dbg_floor_shadow_right.png"
	case '5':
		return "res/img/dbg_floor_shadow_corner.png"
	default:
		return "res/img/dbg_floor.png"
	}
}
