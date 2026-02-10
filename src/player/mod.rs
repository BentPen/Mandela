use crate::control::get_input_direction;
use bevy::prelude::*;

const TILE_SIZE: u32 = 64; // 64x64 tiles
const MAX_FRAMES: usize = 9; // 9 columns per walking row
const TICKS_PER_FRAME: u16 = 3;
const MOVE_SPEED: f32 = 140.0; // pixels per second
const ANIM_DT: f32 = 0.1; // seconds per frame (~10 FPS)

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}

// pub fn move_player( input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player_transform: Single<&mut Transform, With<Player>> ) {
//     let mut direction = Vec2::ZERO;
//     if input.pressed(KeyCode::ArrowLeft) {
//         direction.x -= 1.0;
//     }
//     if input.pressed(KeyCode::ArrowRight) {
//         direction.x += 1.0;
//     }
//     if input.pressed(KeyCode::ArrowUp) {
//         direction.y += 1.0;
//     }
//     if input.pressed(KeyCode::ArrowDown) {
//         direction.y -= 1.0;
//     }
//     if direction != Vec2::ZERO {
//         let speed = 300.0; // pixels per second
//         let delta = direction.normalize() * speed * time.delta_secs();
//         player_transform.translation.x += delta.x;
//         player_transform.translation.y += delta.y;
//     }
// }
//
// pub fn spawn_player(mut commands: Commands) {
//     commands.spawn((
//         Text2d::new("@$$"),
//         TextFont {
//             font_size: 12.0,
//             ..default()
//         },
//         TextColor(Color::BLACK),
//         Transform::from_translation(Vec3::ZERO),
//         Player
//     ));
// }

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let direction = get_input_direction(input);

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        anim.moving = true;

        // Update facing based on dominant direction
        if direction.x.abs() > direction.y.abs() {
            anim.facing = if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        } else {
            anim.facing = if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            };
        }
    } else {
        anim.moving = false;
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    // Compute the target row and current position in the atlas (column/row within the 9-column row)
    let target_row = row_start_index(anim.mode, anim.facing);
    let mut current_col = atlas.index % MAX_FRAMES;
    let current_row = atlas.index / MAX_FRAMES;

    // If the facing changed (or we weren't on a walking row), snap to the first frame of the target row
    if current_row != target_row {
        atlas.index = atlas_index_for(anim.mode, anim.facing, 0);
        current_col = 0;
        // current_row = target_row;
        timer.reset();
    }

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;
    if just_started {
        anim.mode = AnimMode::Walking;
    } else if just_stopped {
        anim.mode = AnimMode::Idle;
    } else {
        anim.tick += 1;
        if anim.tick % TICKS_PER_FRAME > 0 {
            return;
        }
    }

    if anim.tick > 10000 {
        anim.tick = 0;
    }

    // Update previous movement state
    anim.was_moving = anim.moving;

    if just_started || just_stopped {
        anim.tick = 0;
        // On tap or movement start, immediately advance one frame for visible feedback
        let row_start = atlas_index_for(anim.mode, anim.facing, 0);
        let next_col = (current_col + 1) % block_width(anim.mode);
        atlas.index = row_start + next_col;
        // Restart the timer so the next advance uses a full interval
        timer.reset();
    } else {
        if anim.tick > 800 {
            match anim.mode {
                AnimMode::Thrusting => {
                    anim.mode = AnimMode::Jumping;
                }
                _ => {}
            }
        } else if anim.tick > 500 {
            match anim.mode {
                AnimMode::Idle => {
                    anim.mode = AnimMode::Thrusting;
                }
                AnimMode::Walking => {
                    anim.mode = AnimMode::Running;
                }
                _ => {}
            }
        }
        // Continuous movement: advance based on timer cadence
        timer.tick(time.delta());
        if timer.just_finished() {
            let row_start = atlas_index_for(anim.mode, anim.facing, 0);
            let next_col = (current_col + 1) % block_width(anim.mode);
            atlas.index = row_start + next_col;
        }
    }

    // if anim.moving {
    //     if just_started {
    //         // On tap or movement start, immediately advance one frame for visible feedback
    //         let row_start = row_start_index(anim.mode, anim.facing);
    //         let next_col = (current_col + 1) % block_width(anim.mode);
    //         atlas.index = row_start + next_col;
    //         // Restart the timer so the next advance uses a full interval
    //         timer.reset();
    //     } else {
    //         // Continuous movement: advance based on timer cadence
    //         timer.tick(time.delta());
    //         if timer.just_finished() {
    //             let row_start = row_start_index(anim.mode, anim.facing);
    //             let next_col = (current_col + 1) % block_width(anim.mode);
    //             atlas.index = row_start + next_col;
    //         }
    //     }
    // } else {
    //     if just_stopped {
    //         // Not moving: keep current frame to avoid snap. Reset timer on transition to idle.
    //         anim.mode = AnimMode::Idle;
    //         timer.reset();
    //     }
    // }
}

/// Row index, not overall index
fn row_start_index(mode: AnimMode, facing: Facing) -> usize {
    block_offset(mode) + row_in_block(facing)
}

/// Overall index
fn atlas_index_for(mode: AnimMode, facing: Facing, frame_in_row: usize) -> usize {
    MAX_FRAMES * row_start_index(mode, facing) + frame_in_row.min(block_width(mode) - 1)
}

/// Row offset for mode (e.g., 0, 4, 8, ...)
fn block_offset(mode: AnimMode) -> usize {
    let num_blocks = match mode {
        AnimMode::Idle => 0,
        AnimMode::Jumping => 1,
        AnimMode::Running => 2,
        AnimMode::Thrusting => 3,
        AnimMode::Walking => 4,
    };
    4 * num_blocks
}

/// Only for knowing how many frames before wrapping (as after the width the PNG is blank)
fn block_width(mode: AnimMode) -> usize {
    match mode {
        AnimMode::Idle => 2,
        AnimMode::Jumping => 5,
        AnimMode::Running => 8,
        AnimMode::Thrusting => 8,
        AnimMode::Walking => 9,
    }
}

fn row_in_block(facing: Facing) -> usize {
    match facing {
        Facing::Up => 0,
        Facing::Left => 1,
        Facing::Down => 2,
        Facing::Right => 3,
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let texture = asset_server.load("character-spritesheet.png");
    let texture = asset_server.load("lpc_ijrtw.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        MAX_FRAMES as u32,
        20,
        None,
        None,
    ));

    // Start facing down (towards user), idle on first frame of that row
    let mode = AnimMode::Idle;
    let facing = Facing::Down;
    let start_index = atlas_index_for(mode, facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            mode,
            facing,
            moving: false,
            was_moving: false,
            tick: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimMode {
    Idle,
    Jumping,
    Running,
    Thrusting,
    Walking,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    mode: AnimMode,
    facing: Facing,
    moving: bool,
    was_moving: bool,
    tick: u16,
}
