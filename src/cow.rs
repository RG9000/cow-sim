use bevy::prelude::*;
use rand::Rng;
use crate::gameutils::{DecisionTimer, Direction, AnimationTimer, AnimationIndices};

#[derive(Component)]
pub struct Cow {
    target_direction: Direction 
}

pub fn cows_determine_goal(
    mut query: Query<(&Transform, &mut Cow, &mut DecisionTimer)>,
    time: Res<Time>
) {
    for (transform, mut cow, mut timer) in &mut query {
        timer.tick(time.delta());
        if cow.target_direction == Direction::None
        {
            if timer.just_finished() {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..5) == 4{
                    cow.target_direction = match rng.gen_range(1..5)
                    {
                        1 => Direction::Left,
                        2 => Direction::Right,
                        3 => Direction::Up,
                        4 => Direction::Down,
                        _ => Direction::None
                    };
                }
            }
        }
        else {
            //cow has arrived
            if transform.translation.x.floor() % 16.0 == 0.0 && transform.translation.y.floor() % 16.0 == 0.0 {
                cow.target_direction = Direction::None
            }
        }
    }

}

pub fn cows_act(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
        &Cow
    )>,
) {
    for (indices, mut timer, mut sprite, mut transform, cow) in &mut query {
        timer.tick(time.delta());

        if cow.target_direction == Direction::None {
            // Round to the nearest multiple of grid_size
        let new_x = (transform.translation.x / 16.0).round() * 16.0;
        let new_y = (transform.translation.y / 16.0).round() * 16.0;

        transform.translation = Vec3::new(new_x, new_y, transform.translation.z);

        }

        if cow.target_direction == Direction::Left {
            let movement = Vec3::new(-40.0 * time.delta_seconds(), 0.0, 0.0);
            transform.translation += movement;
        }
        else if cow.target_direction == Direction::Right {
            let movement = Vec3::new(40.0 * time.delta_seconds(), 0.0, 0.0);
            transform.translation += movement;
        }
        else if cow.target_direction == Direction::Up{
            let movement = Vec3::new(0.0, 40.0 * time.delta_seconds(), 0.0);
            transform.translation += movement;
        }
        else if cow.target_direction == Direction::Down{
            let movement = Vec3::new(0.0, -40.0 * time.delta_seconds(), 0.0);
            transform.translation += movement;
        }

        if timer.just_finished() {
            if sprite.index < indices.first
            {
                sprite.index = indices.first 
            }
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


pub fn spawn_cows(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("cow.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4,5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for _i in 0..10 {
        let animation_indices = AnimationIndices { first: 16, last: 18 };
        let mut rng = rand::thread_rng();
        commands.spawn(
                        (SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: Transform::from_xyz(rng.gen_range(0.0..50.0)*16.0, rng.gen_range(0.0..50.0)*16.0, 1.0),
                            ..default()
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                        DecisionTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        Cow{target_direction: Direction::None}));

    }

}