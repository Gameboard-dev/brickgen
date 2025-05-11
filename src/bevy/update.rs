
use bevy::picking::pointer::PointerLocation;
use bevy::{
    asset::RenderAssetUsages, 
    pbr::wireframe::WireframeConfig, 
    picking::pointer::PointerInteraction, 
    prelude::*, 
    input::ButtonInput,
    input::mouse::MouseButtonInput,
    input::ButtonState,
};
use bevy::render::{
    mesh::{Indices, PrimitiveTopology}, 
    primitives::Aabb,
};
use voxelis::world::VoxChunk;
use voxelis::Lod;
use crate::utils::voxels::VoxBevy;

use super::cursor::RayCursor;
use super::registry::Registry;


pub fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

/// Move point lights to mesh interaction points.
pub fn update_lamp(
    mut lights: Query<&mut Transform, With<PointLight>>,
    cursor: ResMut<RayCursor>,
) {
    if let Some(mouse_position) = cursor.get() {
            for mut transform in &mut lights {
                transform.translation = mouse_position;
            }
    }
}


/// Rotate a mesh when dragged
pub fn rotate_on_drag(
    drag: Trigger<Pointer<Drag>>, 
    mut transforms: Query<(&mut Transform, &Aabb)>
) {
    if drag.button == PointerButton::Primary {
        if let Ok((mut transform, aabb)) = transforms.get_mut(drag.entity()) {
            let center_offset = transform.rotation * Vec3::from(aabb.center);
            let world_center = transform.translation + center_offset;

            transform.rotation *= Quat::from_rotation_y(drag.delta.x * 0.02)
                * Quat::from_rotation_x(drag.delta.y * 0.02);

            transform.translation = world_center - transform.rotation * Vec3::from(aabb.center);
        }
    }
}


pub fn add_voxels_on_keypress(
    cursor: ResMut<RayCursor>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut vox: ResMut<VoxBevy>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut registry: ResMut<Registry>,
) {
    if let Some(mouse_position) = cursor.get() {
        if mouse_button.pressed(MouseButton::Left) {
            if let Some(amber_material) = registry.materials.get("amber").cloned() {
                vox.put_voxel(mouse_position, amber_material, meshes, registry, commands);
            }
        }
    }
}