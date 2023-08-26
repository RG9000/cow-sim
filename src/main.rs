mod cow;
mod gameutils;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cow::{spawn_cows, cows_act, cows_determine_goal};
use noise::{NoiseFn, Perlin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_startup_system(spawn_world)
    .add_startup_system(spawn_cows.after(spawn_world))
    .add_startup_system(spawn_camera)
    .add_system(cows_act)
    .add_system(cows_determine_goal)
    .run();
}


#[derive(Component)]
pub struct Floor {
    grass_level: f32
}

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

fn perlin_to_floor_types(perlin: Perlin, y_size: usize, x_size: usize) -> Result<Vec<Vec<FloorType>>, bool> {
    let mut matrix: Vec<Vec<FloorType>> = Vec::new();
    //first pass: add the dirt and grass 
        if y_size == 0 || x_size == 0
        {
           return Err(true); 
        }
    for i in 0..y_size{
        let mut row: Vec<FloorType> = Vec::new();
        for j in 0..x_size {
            row.push(if perlin.get([(j as f64)*0.1, (i as f64)*0.1, 0.1]) > -0.2 { FloorType::Grass } else { FloorType::Dirt });
        }
        matrix.push(row);
    }
    
    //now figure out the edges
for i in 1..y_size - 1 {
    for j in 1..x_size - 1 {
        if matrix[i][j] == FloorType::Grass{
            let is_edge_cell = [
                matrix[i-1][j],
                matrix[i+1][j],
                matrix[i][j-1],
                matrix[i][j+1],
            ]
            .iter()
            .any(|&neighbor| neighbor == FloorType::Dirt);

            if !is_edge_cell {
                continue;
            }

            // Check corners first, because they are more specific conditions
            if matrix[i-1][j] == FloorType::Dirt && matrix[i][j-1] == FloorType::Dirt {
                matrix[i][j] = FloorType::LeftTop;
            } else if matrix[i-1][j] == FloorType::Dirt && matrix[i][j+1] == FloorType::Dirt {
                matrix[i][j] = FloorType::RightTop;
            } else if matrix[i+1][j] == FloorType::Dirt && matrix[i][j-1] == FloorType::Dirt {
                matrix[i][j] = FloorType::LeftBottom;
            } else if matrix[i+1][j] == FloorType::Dirt && matrix[i][j+1] == FloorType::Dirt {
                matrix[i][j] = FloorType::RightBottom;
            } else {
                // After checking for corners, check for other types
                if matrix[i-1][j] == FloorType::Dirt {
                    matrix[i][j] = FloorType::Top;
                } else if matrix[i][j+1] == FloorType::Dirt {
                    matrix[i][j] = FloorType::Right;
                } else if matrix[i+1][j] == FloorType::Dirt {
                    matrix[i][j] = FloorType::Bottom;
                } else if matrix[i][j-1] == FloorType::Dirt {
                    matrix[i][j] = FloorType::Left;
                }
            }
        }
    }
    }

        return Ok(matrix);
 }


fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("simple-tiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 13,3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let perlin = Perlin::new(1);
    match perlin_to_floor_types(perlin, 100, 100) {
        Ok(matrix) => {
        for y in (0..100).map(|i| i as usize){
            for x in (0..100).map(|i| i as usize){
                let noise_level = perlin.get([(x as f64)*0.1, (y as f64)*0.1, 0.1]);
                commands.spawn(
                    (SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(matrix[y][x] as usize),
                        transform: Transform::from_xyz((x as f32)*16.0, (y as f32)*16.0 , 0.0),
                        ..default()
                    },
                    Floor{grass_level: (if noise_level > -0.2 { 1.0 } else { 0.0 } )}));

            }
        }
        }
        Err(_) => println!("An error occurred."),
    }

}