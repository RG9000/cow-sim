pub(crate) mod utils;

use bevy::{prelude::*, utils::HashSet};
use noise::Perlin;
use utils::*;

pub struct TiledWorldPlugin;

impl Plugin for TiledWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_world)
           .add_system(recalculate_world);
    }
}

pub const WORLD_SIZE : usize = 100;
pub const TILE_SIZE  : f32 = 16.0;

pub fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut component_grid: ResMut<ComponentGrid>,
    mut grid: ResMut<Grid>,

) {
    let texture_handle = asset_server.load("simple-tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE,TILE_SIZE), 13, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let perlin = Perlin::new(1);

    match perlin_to_floor_types(perlin, WORLD_SIZE, WORLD_SIZE) {
        Ok(matrix) => {
            for y in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    let entity = commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite::new(matrix[y][x] as usize),
                            transform: Transform::from_xyz((x as f32) * TILE_SIZE, (y as f32) * TILE_SIZE, 0.0),
                            ..Default::default()
                        })
                        .insert(Floor {floor_type: matrix[y][x]})
                        .id();

                    component_grid.cells
                        .entry((x as usize, y as usize))
                        .or_insert_with(Vec::new)
                        .push(entity);

                    grid.cells
                        .entry((x as usize, y as usize))
                        .insert(matrix[y][x]);
                }
            }
        }
        Err(_) => println!("An error occurred."),
    }
}

pub fn recalculate_world(
    mut dirty_tiles: ResMut<DirtyTiles>,
    mut query: Query<(&Floor, &Transform, &mut TextureAtlasSprite)>,
    component_grid: ResMut<ComponentGrid>,
    mut grid: ResMut<Grid>,
) {
    let mut new_dirty_tiles = Vec::new();
    let mut rec = true ;
    'outer: while rec == true {
        rec = false;
        if dirty_tiles.0.len() == 0 {
            break;
        }
        for ((x, y), t) in &dirty_tiles.0 {
            grid.cells.insert((*x as usize, *y as usize), *t);
            for y2 in y-2..=y+2 {
                for x2 in x-2..=x+2 {
                    if &y2 == y && &x2 == x {
                        continue;
                    }
                    else {
                        let (new_type, recalc) = recalculate_tile(&grid, y2, x2);
                        if recalc {
                            rec = true;
                            grid.cells
                                .entry((x2 as usize, y2 as usize))
                                .insert(new_type);
                            new_dirty_tiles = Vec::new();
                            continue 'outer;
                        }
                        else {
                            rec = false;
                        }
                        new_dirty_tiles.push(((x2, y2), new_type));
                        grid.cells
                            .entry((x2 as usize, y2 as usize))
                            .insert(new_type);
                    }
                }
            }
        }
    }

    // Then, update the sprites for all dirty tiles (old and new)
    let all_dirty_tiles = [dirty_tiles.0.clone(), new_dirty_tiles.clone()].concat();
    for ((x, y), floor_type) in all_dirty_tiles.iter() {
        if let Some(entities) = component_grid.cells.get(&(*x, *y)) {
            for &entity in entities {
                if let Ok((_, _, mut sprite)) = query.get_mut(entity) {
                    sprite.index = *floor_type as usize;
                }
            }
        }
    }
    if all_dirty_tiles.len() > 0 {
        println!("all: {}", all_dirty_tiles.len());
        println!("original: {}", dirty_tiles.0.clone().len());
    }


    dirty_tiles.0.clear();
}
