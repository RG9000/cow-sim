use bevy::{prelude::*, utils::HashSet};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4
}

#[derive(PartialEq, Copy, Clone)]
pub enum Goal {
    Idle = 0,
    Wonder = 1,
    FindGrass = 2,
    MoveTowardsTile = 3,
    EatGrass = 4
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DecisionTimer(pub Timer);

