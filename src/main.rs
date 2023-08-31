mod cow;
mod tiled_world;
mod character;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::WindowMode;
use character::{spawn_player, move_player_character, animate_characters};
use character::utils::Player;

use crate::tiled_world::*;
use crate::tiled_world::utils::*;

#[derive(Component)]
struct MainCamera;

fn main() {
     let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "cow-sim".into(),
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    };
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(window_plugin))
    .insert_resource(ComponentGrid::default())
    .insert_resource(Grid::default())
    .insert_resource(DirtyTiles::default())
    .insert_resource(Chunks::default())
    .add_plugin(TiledWorldPlugin)
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_system(move_player_character)
    .add_system(update_camera)
    .add_system(animate_characters)
    //.add_startup_system(spawn_cows.after(spawn_world))
    // .add_system(cows_act)
    // .add_system(cows_determine_goal)
    // .add_system(recalculate_world)
    .run();
}

pub fn spawn_camera(
     mut commands: Commands,
) {

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
    ).insert(MainCamera);
}

fn update_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
)
{
    if let Ok(mut transform) = camera.get_single_mut()
    {
        if let Ok(player_transform) = player.get_single() {
            transform.translation = player_transform.translation;
            if keyboard_input.pressed(KeyCode::Equals)
            {
                transform.scale *= Vec3::splat(1.2);
            }
            else if keyboard_input.pressed(KeyCode::Minus)
            {
                transform.scale *= Vec3::splat(0.8);
            }
        }
    }
}

fn update_floor_at_location(
    location: Vec2,
    mut dirty_tiles: ResMut<DirtyTiles>,
    grid: Res<Grid>
) {
    let key = ((location.x/16.0) as usize, (location.y/16.0) as usize);

    if !(grid.cells.contains_key(&key) && grid.cells.get(&key).unwrap() == &FloorType::Dirt ||
    (dirty_tiles.0.iter().any(|&((x, y), t)| x == key.0 && y == key.1 && t == FloorType::Dirt))){
        dirty_tiles.0.push((key, FloorType::Dirt));
    }
}

