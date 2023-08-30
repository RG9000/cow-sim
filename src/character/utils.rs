use bevy::prelude::*;

pub fn body_type_to_walk_sprite(bt: usize) -> String 
{
    return match bt {
        1 => "char1_walk.png".to_string(),
        _ => "".to_string()
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Player;

#[derive(Component)]

pub struct Character {
    pub run_speed: f32,
}
