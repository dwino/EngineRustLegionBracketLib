#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RlState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
