// pub mod flicker;

use bevy::prelude::*;
use bevy::render::view::screenshot::save_to_disk;

use crate::player::Player;

/// Constant offset of light relative to camera
const LIGHT_OFFSET: (f32, f32) = (-250.0, 250.0);

// #[derive(Resource)]
// pub struct LightQualities {
//     pub color: Color,
//     pub radius: f32,
// }

#[derive(Component)]
pub struct LightSource;

pub fn setup_main_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        LightSource,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(LIGHT_OFFSET.0 + 20000.0, LIGHT_OFFSET.1 - 20000.0, 2.),
    ));
}

/// Update the main light position to move toward the player, with an offset.
pub fn update_main_light(
    camera: Single<&mut Transform, (With<Camera2d>, Without<Player>, Without<LightSource>)>,
    mut light: Single<&mut Transform, (With<LightSource>, Without<Player>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = camera.translation;
    let direction = Vec3::new(x + LIGHT_OFFSET.0, y + LIGHT_OFFSET.1, camera.translation.z);
    light
        .translation
        .smooth_nudge(&direction, 50., time.delta_secs());
}
