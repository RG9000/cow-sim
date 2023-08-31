pub(crate) mod utils;

use bevy::{prelude::*, utils::HashSet};
use noise::Perlin;
use utils::*;

use crate::character::utils::Player;

pub struct TiledWorldPlugin;

impl Plugin for TiledWorldPlugin {
    fn build(&self, app: &mut App) {
        app
           .add_system(manage_chunks);
    }
}

pub const CHUNK_SIZE : usize = 100;
pub const TILE_SIZE  : f32 = 16.0;

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

fn manage_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut chunks: ResMut<Chunks>
) {
    for transform in player_query.iter() {
        let player_x = transform.translation.x;
        let player_y = transform.translation.y;

        let current_chunk_x = (player_x / TILE_SIZE / CHUNK_SIZE as f32) as i32;
        let current_chunk_y = (player_y / TILE_SIZE / CHUNK_SIZE as f32) as i32;

        // Load current and adjacent chunks, unload others
        let mut chunks_to_keep = HashSet::new();
        let mut chunks_to_load = Vec::new();

        // Determine which chunks to keep and which to load
        for x in current_chunk_x - 2..=current_chunk_x + 1 {
            for y in current_chunk_y - 1..=current_chunk_y + 2 {
                chunks_to_keep.insert((x, y));
                if !chunks.cells.contains_key(&(x, y)) {
                    chunks_to_load.push((x, y));
                }
            }
        }

        // Load new chunks
        for (x, y) in chunks_to_load.iter() {
            load_chunk(&mut commands, &mut chunks, &mut texture_atlases, &asset_server, *x, *y);
        }

        // Unload old chunks
        let chunks_to_unload: Vec<_> = chunks.cells.keys().filter(|&k| !chunks_to_keep.contains(k)).cloned().collect();
        for k in chunks_to_unload {
            unload_chunk(&mut commands, &mut chunks, k);
        }

    }
}