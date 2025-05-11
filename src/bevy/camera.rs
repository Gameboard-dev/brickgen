use bevy::{
    core_pipeline::{tonemapping::Tonemapping, Skybox}, 
    pbr::VolumetricFog, 
    prelude::*
};
use bevy_flycam::prelude::*;

const POSITION: Vec3 = Vec3::new(2.2716377, 1.2876732, 3.9676127);
const LOOK_AT: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const BRIGHTNESS: f32 = 1000.0;
const FOG_INTENSITY: f32 = 0.1;

pub fn spawn_camera(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
    .spawn((
        Camera3d::default(),
        Camera::default(),
        Msaa::Off,
        Transform::from_xyz(2.2716377, 1.2876732, 3.9676127)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        FlyCam
    ))
    .insert(Tonemapping::TonyMcMapface)
    .insert(Skybox {
        image: asset_server.load("environment_maps/skymap.ktx2"),
        brightness: BRIGHTNESS,
        ..default()
    })
    .insert(VolumetricFog {
        ambient_intensity: FOG_INTENSITY,
        ..default()
    });
    commands.insert_resource(MovementSettings {
        sensitivity: 0.00015, // default: 0.00012
        speed: 5.0, // default: 12.0
    });
    commands.insert_resource(KeyBindings {
        move_ascend: KeyCode::Space,
        move_descend: KeyCode::ShiftLeft,
        ..Default::default()
    });
}
