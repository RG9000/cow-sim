mod cow;
mod tiled_world;
mod gameutils;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::tiled_world::*;
use crate::tiled_world::utils::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .insert_resource(ComponentGrid::default())
    .insert_resource(Grid::default())
    .insert_resource(DirtyTiles::default())
    .add_plugin(TiledWorldPlugin)
    .add_startup_system(spawn_camera)
    .add_system(move_camera)
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
        }
    );
}

pub fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    grid: Res<Grid>,
    input: Res<Input<KeyCode>>,
    dirty_tiles: ResMut<DirtyTiles>
)
{
    if let Ok(mut transform) = camera.get_single_mut()
    {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 16.0;
        } else if input.pressed(KeyCode::S) {
            transform.translation.y -= 16.0;
        } else if input.pressed(KeyCode::A) {
            transform.translation.x -= 16.0;
        } else if input.pressed(KeyCode::D) {
            transform.translation.x += 16.0;
        }
        if input.pressed(KeyCode::Space) {
            update_floor_at_location(Vec2::new(transform.translation.x,transform.translation.y), dirty_tiles, grid);
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
