
use bevy::color::palettes::css::BLACK;
use bevy::color::palettes::css::WHITE;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;
use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::RED;

use crate::player::Player;
use crate::lights::LightSource;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;

pub fn setup_main_tile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Node {
            display: Display::Grid,
            width: percent(100),
            height: percent(100),
            ..Default::default()
        },
        BackgroundColor(Color::linear_rgba(0., 0., 0., 0.1)),
        BackgroundGradient(vec![
            LinearGradient::to_top_right(vec![
                // ColorStop::auto(RED),
                ColorStop::auto(RED.with_alpha(0.1)),
                ColorStop::auto(BLUE.with_alpha(0.1)),
            ])
            .into(),
            Gradient::Radial(RadialGradient{
                position: UiPosition::new(Vec2::ZERO, Val::Percent(-20.), Val::Percent(30.)),
                stops: vec![
                    ColorStop::auto(BLACK.with_alpha(0.1)),
                    ColorStop::auto(WHITE.with_alpha(0.05)),
                    ColorStop::auto(BLACK.with_alpha(0.1)),
                ],
                ..Default::default()
            })
            // (vec![
            //     // ColorStop::auto(RED),
            //     ColorStop::auto(RED.with_alpha(0.1)),
            //     ColorStop::auto(BLUE.with_alpha(0.1)),
            // ])
            // .into(),
        ])
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 200.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));
}

// pub fn setup_instructions(mut commands: Commands) {
//     commands.spawn((
//         Text::new("Move the light with WASD.\nThe camera will smoothly track the light."),
//         Node {
//             position_type: PositionType::Absolute,
//             bottom: px(12),
//             left: px(12),
//             ..default()
//         },
//     ));
// }

/// Update the camera position by tracking the player.
pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>, Without<LightSource>)>,
    player: Single<&Transform, (With<Player>, Without<Mesh2d>)>,
    // mut light: Single<&mut Transform, (With<LightSource>, Without<Player>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    let delta = time.delta_secs();
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, delta);
    // let direction = Vec3::new(x+LIGHT_OFFSET.0, y+LIGHT_OFFSET.1, camera.translation.z);
    // light
    //     .translation
    //     .smooth_nudge(&direction, CAMERA_DECAY_RATE*0.6, delta);
}
