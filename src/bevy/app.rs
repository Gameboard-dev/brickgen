use crate::utils::voxels::{show_chunks, VoxBevy, VoxConfig, HIGHEST_RESOLUTION};

use super::{
    camera::spawn_camera, cursor::{update_cursor, RayCursor}, lights::spawn_lights, plugins::plugins, registry::{add_materials, Registry}, update::{add_voxels_on_keypress, toggle_wireframe, update_lamp}
};
use bevy::{
    color::palettes::tailwind::BLUE_300, pbr::DirectionalLightShadowMap, picking::pointer::PointerInteraction, prelude::*
};
use bevy_flycam::prelude::*;
use bevy_window_utils::WindowUtils;
use voxelis::{Lod, MaxDepth};


fn setup_icon(mut window: ResMut<WindowUtils>, asset_server: Res<AssetServer>) {
    window.window_icon = Some(asset_server.load("icon.png"));
}

/// Spawns a simple cube mesh with a material
#[allow(dead_code)]
fn create_debug_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let cube: Handle<Mesh> = meshes.add(Cuboid::default());
    let material = materials.add(Color::WHITE);
    commands.spawn((
                Mesh3d(cube),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn application() {

    let vox_config = VoxConfig {
        max_depth: MaxDepth::new(6), // 6 voxels per axis
        chunk_world_size: 1.36, 
        world_bounds: IVec3::new(4, 4, 4), // X*Y*Z chunks
        memory: 1024 * 1024 * 256, // Example: 256 MB
        lod: Lod::new(HIGHEST_RESOLUTION)
    };

    App::new()
        .add_plugins(plugins())
        .insert_resource(ClearColor(Color::from(BLUE_300)))
        .insert_resource(DirectionalLightShadowMap { size: 8000 })
        .insert_resource(Registry::new())
        .insert_resource(VoxBevy::new(vox_config))
        .insert_resource(RayCursor::new())
        .add_systems(PreStartup, (
            add_materials, 
            setup_icon
        ))
        .add_systems(Startup, (
            spawn_lights,
            spawn_camera,
        ))
        .add_systems(Update, (
            //toggle_wireframe, 
            show_chunks,
            update_cursor,
        ))
        .add_systems(PostUpdate, (
            update_lamp,
            add_voxels_on_keypress
        ))
        .run();
}