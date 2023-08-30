pub mod utils;

use bevy::prelude::*;

use self::utils::{body_type_to_walk_sprite, AnimationIndices, AnimationTimer, Player, Character};

pub fn spawn_player(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    spawn_character(commands, 50, 50, asset_server, 1, texture_atlases, true);
}

pub fn animate_characters(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &AnimationIndices, &mut AnimationTimer), With<Character>>,
) {
    for (mut sprite, indices, mut timer) in query.iter_mut() {
        if sprite.index > indices.last || sprite.index < indices.first
        {
            sprite.index = indices.first;
        }
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if sprite.index == indices.last {
                sprite.index = indices.first;
            } else {
                sprite.index += 1;
            }
        }
    }
}

pub fn move_player_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Character, &mut AnimationIndices), With<Player>>,
    time: Res<Time>
) {
    if let Ok((mut transform, character, mut indices)) = player_query.get_single_mut() {
        let mut is_moving: bool = false;
        let mut animation_indices = AnimationIndices{first: 0, last: 0 };
        let mut direction:Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(KeyCode::W) {
            is_moving = true;
            animation_indices = AnimationIndices {first: 8, last: 15};
            direction += Vec3::new(0.0, 1.0 * character.run_speed * time.delta_seconds(), 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            is_moving = true;
            animation_indices = AnimationIndices {first: 0, last: 7};
            direction += Vec3::new(0.0, -1.0 * character.run_speed * time.delta_seconds(), 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            is_moving = true;
            animation_indices = AnimationIndices {first: 24, last: 31};
            direction += Vec3::new(-1.0 * character.run_speed * time.delta_seconds(), 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            is_moving = true;
            animation_indices = AnimationIndices {first: 16, last: 23};
            direction += Vec3::new(1.0 * character.run_speed * time.delta_seconds(), 0.0, 0.0);
        }

        if !is_moving{
            animation_indices = AnimationIndices {first: 4, last: 4};
        }
        
        indices.first = animation_indices.first;
        indices.last = animation_indices.last;
        
        transform.translation += direction;
    }
}

fn spawn_character(
    mut commands: Commands,
    x:usize,
    y:usize,
    asset_server: Res<AssetServer>,
    body_type: usize,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    is_player: bool
)
{
    let body_texture_handle = asset_server.load(body_type_to_walk_sprite(body_type));
    let body_texture_atlas =
        TextureAtlas::from_grid(body_texture_handle, Vec2::new(32.0, 32.0), 8,4, None, None);
    let texture_atlas_handle = texture_atlases.add(body_texture_atlas);

    let mut entity_builder = commands.spawn_empty();
    entity_builder.insert(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(x as f32*16.0, y as f32*16.0, 1.0),
        ..Default::default()
    });
    entity_builder.insert(Character{run_speed: 60.0});
    entity_builder.insert(AnimationIndices{first: 0, last: 7});
    entity_builder.insert(AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)));

    if is_player {
        entity_builder.insert(Player);
    } 
}

// pub fn spawn_cows(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture_handle = asset_server.load("cow.png");
//     let texture_atlas =
//         TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4,5, None, None);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     for _i in 0..10 {
//         let animation_indices = AnionIndices { first: 16, last: 18 };
//         let mut rng = rand::thread_rng();
//         commands.spawn(
//                         (SpriteSheetBundle {
//                             texture_atlas: texture_atlas_handle.clone(),
//                             sprite: TextureAtlasSprite::new(0),
//                             transform: Transform::from_xyz(rng.gen_range(0.0..50.0)*16.0, rng.gen_range(0.0..50.0)*16.0, 1.0),
//                             ..default()
//                         },
//                         animation_indices,
//                         AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
//                         DecisionTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
//                         Hunger{hungriness: 0.0},
//                         GoalHaver{goal: Goal::Wonder, goal_location: Vec2::new(0.0,0.0)},
//                         GrassEater{},
//                         Cow{target_direction: Direction::None}));

//     }

// }