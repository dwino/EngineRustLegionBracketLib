pub use crate::prelude::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToDig {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMoveCamera {
    pub delta: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToRangedAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsCycleTarget {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsEndTargeting {}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsEndInput(pub RlState);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsEndSmInput(pub SmState);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsStateSwitch(pub EcoState);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToPatrolRandomly {}
