use bevy::{prelude::*, utils::HashMap};
use noise::{Perlin, NoiseFn};

#[derive(Component)]
pub struct Floor {pub floor_type: FloorType}

#[derive(Default, Debug, Resource)]
pub struct ComponentGrid {
    pub cells: HashMap<(usize, usize), Vec<Entity>>,
}

#[derive(Default, Debug, Resource)]
pub struct Grid {
    pub cells: HashMap<(usize, usize), FloorType>,
}

#[derive(Default, Resource)]
pub struct DirtyTiles(pub Vec<((usize, usize), FloorType)>);

#[derive(PartialEq, Copy, Clone, Debug)]
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

pub fn perlin_to_floor_types(perlin: Perlin, y_size: usize, x_size: usize) -> Result<Vec<Vec<FloorType>>, bool> {
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

                calculate_tile(&mut matrix, i, j);
            }
        }
    }
    return Ok(matrix);
 }

 pub fn calculate_tile(matrix: &mut Vec<Vec<FloorType>>, i : usize, j : usize) {
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

pub fn recalculate_tile(grid: &ResMut<Grid>, j: usize, i: usize) -> (FloorType, bool) {
    if grid.cells.get(&(i, j)).unwrap() != &FloorType::Dirt {
        let is_edge_cell = [
            grid.cells.get(&(i - 1, j)),
            grid.cells.get(&(i + 1, j)),
            grid.cells.get(&(i, j - 1)),
            grid.cells.get(&(i, j + 1)),
        ]
        .iter()
        .any(|&neighbor| neighbor.unwrap() == &FloorType::Dirt);

        if (grid.cells.get(&(i - 1, j)).unwrap() == &FloorType::Dirt
            && grid.cells.get(&(i + 1, j)).unwrap() == &FloorType::Dirt)
            || (grid.cells.get(&(i, j - 1)).unwrap() == &FloorType::Dirt
                && grid.cells.get(&(i, j + 1)).unwrap() == &FloorType::Dirt)
        {
            return (FloorType::Dirt, true);
        }

        if !is_edge_cell {
            return (FloorType::Grass, false);
        }

        if grid.cells.get(&(i - 1, j)).unwrap() == &FloorType::Dirt
            && grid.cells.get(&(i, j - 1)).unwrap() == &FloorType::Dirt
        {
            return (FloorType::LeftTop, false);
        } else if grid.cells.get(&(i - 1, j)).unwrap() == &FloorType::Dirt
            && grid.cells.get(&(i, j + 1)).unwrap() == &FloorType::Dirt
        {
            return (FloorType::LeftBottom, false);
        } else if grid.cells.get(&(i + 1, j)).unwrap() == &FloorType::Dirt
            && grid.cells.get(&(i, j - 1)).unwrap() == &FloorType::Dirt
        {
            return (FloorType::RightTop, false);
        } else if grid.cells.get(&(i + 1, j)).unwrap() == &FloorType::Dirt
            && grid.cells.get(&(i, j + 1)).unwrap() == &FloorType::Dirt
        {
            return (FloorType::RightBottom, false);
        } else {
            if grid.cells.get(&(i - 1, j)).unwrap() == &FloorType::Dirt {
                return (FloorType::Left, false);
            } else if grid.cells.get(&(i, j + 1)).unwrap() == &FloorType::Dirt {
                return (FloorType::Bottom, false);
            } else if grid.cells.get(&(i + 1, j)).unwrap() == &FloorType::Dirt {
                return (FloorType::Right, false);
            } else if grid.cells.get(&(i, j - 1)).unwrap() == &FloorType::Dirt {
                return (FloorType::Top, false);
            }
        }
    }

    (FloorType::Dirt, false)
}
