mod cow;
mod gameutils;
mod world;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cow::{spawn_cows, cows_act, cows_determine_goal};
use gameutils::{FloorMatrix, DirtyTiles};
use world::{spawn_world, Grid, recalculate_world};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .insert_resource(Grid::default())
    .insert_resource(FloorMatrix::default())
    .insert_resource(DirtyTiles::default())
    .add_startup_system(spawn_world)
    .add_startup_system(spawn_cows.after(spawn_world))
    .add_startup_system(spawn_camera)
    .add_system(cows_act)
    .add_system(cows_determine_goal)
    .add_system(recalculate_world)
    .run();
}

pub fn spawn_camera(
     mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 5.0),
            ..default()
        }
    );
    
}
