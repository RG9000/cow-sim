// use bevy::prelude::*;
// use rand::Rng;
// use crate::{gameutils::{DecisionTimer, Direction, AnimationTimer, AnimationIndices, Goal}, tiled_world::utils::{Floor, FloorType, DirtyTiles, Grid, ComponentGrid}};

// #[derive(Component)]
// pub struct Cow {
//     target_direction: Direction 
// }

// #[derive(Component)]
// pub struct Hunger {
//     hungriness: f32 
// }

// #[derive(Component)]
//  pub struct GoalHaver {
//     goal: Goal,
//     goal_location: Vec2
// }

// #[derive(Component)]
// pub struct GrassEater {}

// pub fn cows_determine_goal(
//     grid: Res<ComponentGrid>,
//     mut query: Query<(&Transform, &mut Cow, &mut DecisionTimer, &mut Hunger, &mut GoalHaver), Without<Floor>>,
//     mut floor_query: Query<(&mut Floor, &Transform), With<Floor>>, 
//     time: Res<Time>,
//     mut dirty_tiles: ResMut<DirtyTiles>,
// ) {
//     let mut rng = rand::thread_rng();
     
//     for (transform, mut cow, mut timer, mut hunger, mut goal_haver) in &mut query {
//         hunger.hungriness += time.delta_seconds() * 0.1;
//         timer.tick(time.delta());
//         if goal_haver.goal == Goal::Wonder {
//             if hunger.hungriness >= 0.5
//             {
//                 goal_haver.goal = Goal::FindGrass;
//                 continue;
//             }
//             else {
//                 cows_decision_wonder(transform, &mut cow, &mut timer, &mut rng);
//             }
//         }
//         else if goal_haver.goal == Goal::FindGrass {
//                 find_nearest_floor(&grid, transform, &mut goal_haver, &floor_query);
//         }
//         else if goal_haver.goal == Goal::MoveTowardsTile {
//             if transform.translation.x == goal_haver.goal_location.x && transform.translation.y == goal_haver.goal_location.y {
//                 goal_haver.goal = Goal::EatGrass;
//             }
              
//                 if ((transform.translation.x - goal_haver.goal_location.x).powf(2.0) + (transform.translation.y - goal_haver.goal_location.y).powf(2.0)).sqrt() < 1.0
//                 {
//                     cow.target_direction = Direction::None;
//                 }
//                 else if transform.translation.x > goal_haver.goal_location.x {
//                     cow.target_direction = Direction::Left;
//                 }
//                 else if transform.translation.x < goal_haver.goal_location.x {
//                     cow.target_direction = Direction::Right;
//                 }
//                 else if transform.translation.y < goal_haver.goal_location.y {
//                     cow.target_direction = Direction::Up;
//                 }
//                 else if transform.translation.y > goal_haver.goal_location.y {
//                     cow.target_direction = Direction::Down;
//                 }
//             }
//         else if goal_haver.goal == Goal::EatGrass {
//             update_floor_at_location(goal_haver.goal_location, &mut floor_query, &mut dirty_tiles);
//             hunger.hungriness = 0.0;
//             goal_haver.goal = Goal::Wonder;
//         }
//     }
// }

// pub fn cows_decision_wonder(
//     transform: &Transform, 
//     cow: &mut Cow, 
//     timer: &mut DecisionTimer, 
//     rng: &mut rand::rngs::ThreadRng
// ) {
//     if cow.target_direction == Direction::None {
//         if timer.just_finished() {
//             if rng.gen_range(0..5) == 4 {
//                 cow.target_direction = match rng.gen_range(1..5) {
//                     1 => Direction::Left,
//                     2 => Direction::Right,
//                     3 => Direction::Up,
//                     4 => Direction::Down,
//                     _ => Direction::None,
//                 };
//             }
//         }
//     } else {
//         // Cow has arrived
//         if transform.translation.x.floor() % 16.0 == 0.0 && transform.translation.y.floor() % 16.0 == 0.0 {
//             cow.target_direction = Direction::None;
//         }
//     }
// }

// pub fn cows_act(
//     time: Res<Time>,
//     mut query: Query<(
//         &AnimationIndices,
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &mut Transform,
//         &Cow
//     )>,
// ) {
//     for (indices, mut timer, mut sprite, mut transform, cow) in &mut query {
//         timer.tick(time.delta());

//         if cow.target_direction == Direction::None 
//          {
//             // Round to the nearest multiple of grid_size
//             let new_x = (transform.translation.x / 16.0).round() * 16.0;
//             let new_y = (transform.translation.y / 16.0).round() * 16.0;

//             transform.translation = Vec3::new(new_x, new_y, transform.translation.z);

//         }

//         if cow.target_direction == Direction::Left {
//             let movement = Vec3::new(-40.0 * time.delta_seconds(), 0.0, 0.0);
//             transform.translation += movement;
//         }
//         else if cow.target_direction == Direction::Right {
//             let movement = Vec3::new(40.0 * time.delta_seconds(), 0.0, 0.0);
//             transform.translation += movement;
//         }
//         else if cow.target_direction == Direction::Up{
//             let movement = Vec3::new(0.0, 40.0 * time.delta_seconds(), 0.0);
//             transform.translation += movement;
//         }
//         else if cow.target_direction == Direction::Down{
//             let movement = Vec3::new(0.0, -40.0 * time.delta_seconds(), 0.0);
//             transform.translation += movement;
//         }

//         if timer.just_finished() {
//             if sprite.index < indices.first
//             {
//                 sprite.index = indices.first 
//             }
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }


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

// fn update_floor_at_location(
//     goal_location: Vec2,
//     floor_query: &mut Query<(&mut Floor, &Transform), With<Floor>>,
//     dirty_tiles: &mut ResMut<DirtyTiles>,
// ) {
//     for (mut floor, transform) in floor_query.iter_mut() {
//         let distance_x = goal_location.x - transform.translation.x;
//         let distance_y = goal_location.y - transform.translation.y;
        
//         // You might want to adjust the "close enough" condition based on your specific requirements.
//         if distance_x.abs() < 1.0 && distance_y.abs() < 1.0 {
//             // Update the floor type
//             floor.floor_type = FloorType::Dirt; // Replace with the FloorType you want to set
//             dirty_tiles.0.push((((transform.translation.x / 16.0) as usize, (transform.translation.y / 16.0) as usize), FloorType::Dirt));

//             // If only one floor entity is expected at this location, break after updating.
//             break;
//         }
//     }
// }

// fn find_nearest_floor(
//     grid: &Res<ComponentGrid>,
//     cow_transform: &Transform,
//     goal_haver: &mut GoalHaver,
//     floor_query: &Query<(&mut Floor, &Transform), With<Floor>>, // Add this query to fetch Transforms for Floor entities
// ) {
//     let cow_x = cow_transform.translation.x as usize / 16;
//     let cow_y = cow_transform.translation.y as usize / 16;

//     if cow_x < 5 || cow_y < 5
//     {
//         return;
//     } 
    
//     let adjacent_cells = vec![
//         (cow_x - 2, cow_y),
//         (cow_x - 1, cow_y),
//         (cow_x, cow_y),
//         (cow_x + 1, cow_y),
//         (cow_x + 2, cow_y),
//         (cow_x - 2, cow_y - 1),
//         (cow_x - 1, cow_y - 1),
//         (cow_x, cow_y - 1),
//         (cow_x + 1, cow_y - 1),
//         (cow_x + 2, cow_y - 1),
//         (cow_x - 2, cow_y + 1),
//         (cow_x - 1, cow_y + 1),
//         (cow_x, cow_y + 1),
//         (cow_x + 1, cow_y + 1),
//         (cow_x + 2, cow_y + 1),
//         (cow_x - 2, cow_y - 2),
//         (cow_x - 1, cow_y - 2),
//         (cow_x, cow_y - 2),
//         (cow_x + 1, cow_y - 2),
//         (cow_x + 2, cow_y - 2),
//         (cow_x - 2, cow_y + 2),
//         (cow_x - 1, cow_y + 2),
//         (cow_x, cow_y + 2),
//         (cow_x + 1, cow_y + 2),
//         (cow_x + 2, cow_y + 2),
//     ];
   
//     let mut closest_distance = f32::MAX;
//     let mut closest_floor: Option<Vec2> = None;

//     for (x, y) in adjacent_cells {
//         if let Some(entities) = grid.cells.get(&(x, y)) {
//             for &entity in entities {
//                 if let Ok(floor_transform) = floor_query.get_component::<Transform>(entity) {
//                     let dx = cow_transform.translation.x - floor_transform.translation.x;
//                     let dy = cow_transform.translation.y - floor_transform.translation.y;
//                     let distance = (dx * dx + dy * dy).sqrt();

//                     if distance < closest_distance { 
//                         if let Ok(floor) = floor_query.get_component::<Floor>(entity) {
//                             if floor.floor_type == FloorType::Grass
//                             {
//                                 closest_distance = distance;
//                                 closest_floor = Some(Vec2::new(floor_transform.translation.x, floor_transform.translation.y));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     if let Some(closest_floor_location) = closest_floor {
//         goal_haver.goal = Goal::MoveTowardsTile;
//         goal_haver.goal_location = closest_floor_location;
//     }
// } 