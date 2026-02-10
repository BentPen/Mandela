use bevy::asset::AssetMetaCheck;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

mod control;
mod player;
mod camera;
mod lights;

use player::PlayerPlugin;
use camera::*;
use lights::*;

fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2d);
    commands.spawn((Camera2d, Bloom::NATURAL));
}

fn main() {
	App::new()
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            file_path: "src/assets".into(),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))  // We have updated the bg color to white
        .add_systems(Startup, (setup_main_tile, setup_main_light, setup_camera).chain())
        .add_systems(Update, (update_camera, update_main_light).chain())
        .add_plugins(PlayerPlugin)
        .run();
}

