use bevy::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4
}


#[derive(PartialEq, Copy, Clone)]
pub enum FloorType {
    Grass = 14,
    LeftTop = 26 ,
    Top = 27,
    RightTop = 28,
    Right = 15,
    RightBottom = 2,
    Bottom = 1,
    LeftBottom = 0,
    Left = 13,
    Dirt = 6 
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