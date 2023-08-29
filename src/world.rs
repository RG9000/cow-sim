
// use noise::{NoiseFn, Perlin};
// use bevy::{prelude::*, utils::{HashMap, HashSet}};
// use crate::gameutils::{FloorType, DirtyTiles, FloorMatrix};
// use std::fmt::Debug;

// #[derive(Component)]
// pub struct Floor {pub floor_type: FloorType}


// #[derive(Default, Debug, Resource)]
// pub struct Grid {
//     pub cells: HashMap<(usize, usize), Vec<Entity>>,
// }

// fn perlin_to_floor_types(perlin: Perlin, y_size: usize, x_size: usize) -> Result<Vec<Vec<FloorType>>, bool> {
//     let mut matrix: Vec<Vec<FloorType>> = Vec::new();
//     //first pass: add the dirt and grass 
//         if y_size == 0 || x_size == 0
//         {
//            return Err(true); 
//         }
//     for i in 0..y_size{
//         let mut row: Vec<FloorType> = Vec::new();
//         for j in 0..x_size {
//             row.push(if perlin.get([(j as f64)*0.1, (i as f64)*0.1, 0.1]) > -0.2 { FloorType::Grass } else { FloorType::Dirt });
//         }
//         matrix.push(row);
//     }
    
//     //now figure out the edges
// for i in 1..y_size - 1 {
//     for j in 1..x_size - 1 {
//         if matrix[i][j] == FloorType::Grass{
//             let is_edge_cell = [
//                 matrix[i-1][j],
//                 matrix[i+1][j],
//                 matrix[i][j-1],
//                 matrix[i][j+1],
//             ]
//             .iter()
//             .any(|&neighbor| neighbor == FloorType::Dirt);

//             if !is_edge_cell {
//                 continue;
//             }

//             // Check corners first, because they are more specific conditions
//             if matrix[i-1][j] == FloorType::Dirt && matrix[i][j-1] == FloorType::Dirt {
//                 matrix[i][j] = FloorType::LeftTop;
//             } else if matrix[i-1][j] == FloorType::Dirt && matrix[i][j+1] == FloorType::Dirt {
//                 matrix[i][j] = FloorType::RightTop;
//             } else if matrix[i+1][j] == FloorType::Dirt && matrix[i][j-1] == FloorType::Dirt {
//                 matrix[i][j] = FloorType::LeftBottom;
//             } else if matrix[i+1][j] == FloorType::Dirt && matrix[i][j+1] == FloorType::Dirt {
//                 matrix[i][j] = FloorType::RightBottom;
//             } else {
//                 // After checking for corners, check for other types
//                 if matrix[i-1][j] == FloorType::Dirt {
//                     matrix[i][j] = FloorType::Top;
//                 } else if matrix[i][j+1] == FloorType::Dirt {
//                     matrix[i][j] = FloorType::Right;
//                 } else if matrix[i+1][j] == FloorType::Dirt {
//                     matrix[i][j] = FloorType::Bottom;
//                 } else if matrix[i][j-1] == FloorType::Dirt {
//                     matrix[i][j] = FloorType::Left;
//                 }
//             }
//         }
//     }
//     }

//         return Ok(matrix);
//  }

//  fn update_tile_and_neighbors(
//     x: usize,
//     y: usize,
//     depth: usize,
//     max_depth: usize,
//     floor_matrix: &mut ResMut<FloorMatrix>,
//     new_dirty_tiles: &mut Vec<((usize, usize), FloorType)>,
// ) {
//     if depth > max_depth || x < 5 || y < 5 {
//         return;
//     }

//     for dx in -1..=1 {
//         for dy in -1..=1 {
//             println!("depth: {}", depth);
//             let nx = (x as isize + dx) as usize;
//             let ny = (y as isize + dy) as usize;

//             if nx == x && ny == y {
//                 continue;
//             }

//             // Check boundaries
//             if nx >= floor_matrix.0.len() || ny >= floor_matrix.0[0].len() {
//                 continue;
//             }
            
//             let current_type = floor_matrix.0[ny][nx];
//             let mut new_type = FloorType::Grass;
            
//             if depth == 0 {
//                 floor_matrix.0[ny][nx] = new_type;
//                 new_dirty_tiles.push(((nx, ny), new_type));
//                 update_tile_and_neighbors(nx, ny, depth + 1, max_depth, floor_matrix, new_dirty_tiles);
//                 continue;
//             }

//             let is_edge_cell = [
//                 floor_matrix.0[ny-1][nx],
//                 floor_matrix.0[ny+1][nx],
//                 floor_matrix.0[ny][nx-1],
//                 floor_matrix.0[ny][nx+1],
//             ]
//             .iter()
//             .any(|&neighbor| neighbor == FloorType::Dirt);

//             if !is_edge_cell || current_type == FloorType::Dirt{
//                 continue;
//             }
            
//             // Check corners first, because they are more specific conditions
//             if floor_matrix.0[ny-1][nx] == FloorType::Dirt && floor_matrix.0[ny][nx-1] == FloorType::Dirt {
//                 new_type = FloorType::LeftTop;
//             } else if floor_matrix.0[ny-1][nx] == FloorType::Dirt && floor_matrix.0[ny][nx+1] == FloorType::Dirt {
//                 new_type = FloorType::RightTop;
//             } else if floor_matrix.0[ny+1][nx] == FloorType::Dirt && floor_matrix.0[ny][nx-1] == FloorType::Dirt {
//                 new_type = FloorType::LeftBottom;
//             } else if floor_matrix.0[ny+1][nx] == FloorType::Dirt && floor_matrix.0[ny][nx+1] == FloorType::Dirt {
//                 new_type = FloorType::RightBottom;
//             } else {
//                 // After checking for corners, check for other types
//                 if floor_matrix.0[ny-1][nx] == FloorType::Dirt {
//                     new_type = FloorType::Top;
//                 } else if floor_matrix.0[ny][nx+1] == FloorType::Dirt {
//                     new_type = FloorType::Right;
//                 } else if floor_matrix.0[ny+1][nx] == FloorType::Dirt {
//                     new_type = FloorType::Bottom;
//                 } else if floor_matrix.0[ny][nx-1] == FloorType::Dirt {
//                     new_type = FloorType::Left;
//                 }
//             }

//             if new_type != current_type {
//                 floor_matrix.0[ny][nx] = new_type;
//                 new_dirty_tiles.push(((nx, ny), new_type));
//                 // Recursively update the neighbors of this tile
//                 update_tile_and_neighbors(nx, ny, depth + 1, max_depth, floor_matrix, new_dirty_tiles);
//             }
//         }
//     }
// }


// pub fn recalculate_world(
//     mut floor_matrix: ResMut<FloorMatrix>,
//     mut dirty_tiles: ResMut<DirtyTiles>,
//     mut query: Query<(&Floor, &Transform, &mut TextureAtlasSprite)>,
//     grid: Res<Grid>,
// ) {
//     let mut new_dirty_tiles = Vec::new();
//     let max_depth = 3; // Replace with whatever maximum depth you want

//     for &((x, y), _) in &dirty_tiles.0 {
//         update_tile_and_neighbors(x, y, 0, max_depth, &mut floor_matrix, &mut new_dirty_tiles);
//     }

//     // Then, update the sprites for all dirty tiles (old and new)
//     let all_dirty_tiles = [dirty_tiles.0.clone(), new_dirty_tiles.clone()].concat();
//     for ((x, y), floor_type) in all_dirty_tiles.iter() {
//         if let Some(entities) = grid.cells.get(&(*x, *y)) {
//             for &entity in entities {
//                 if let Ok((_, _, mut sprite)) = query.get_mut(entity) {
//                     sprite.index = *floor_type as usize;
//                 }
//             }
//         }
//     }

//     dirty_tiles.0 = new_dirty_tiles;
// }




// unsafe impl Send for Grid {}
// unsafe impl Sync for Grid {}