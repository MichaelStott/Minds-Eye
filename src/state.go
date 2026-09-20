package mindseye

type State interface {
	OnEnter(gtx *GameContext) error

	OnExit(gtx *GameContext) error

	Update(gtx *GameContext, dt float32) (State, error)

	Draw(gtx *GameContext) error
}
