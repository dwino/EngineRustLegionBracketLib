#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SmState {
    AwaitingInput,
    EmployeeTurn,
    CustomerTurn,
    GameOver,
    Victory,
    NextLevel,
}
