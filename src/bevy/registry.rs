use bevy::{
    asset::{Assets, Handle}, color::palettes::tailwind::AMBER_700, ecs::system::ResMut, pbr::StandardMaterial, prelude::*, utils::hashbrown::HashMap
};
use voxelis::world::VoxChunk;

// Storing handles avoids duplication in `Assets<StandardMaterial>`
#[derive(Resource)]
pub struct Registry {
    // &'static str avoids heap allocation overhead for keys which are known when the code compiles
    pub materials: HashMap<&'static str, Handle<StandardMaterial>>,
    // meshes need to be hash mapped to voxel chunks
    pub meshes: HashMap<IVec3, Handle<Mesh>>,
}
impl Registry {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            meshes: HashMap::new()
        }
    }
}

pub fn add_materials(
    mut assets: ResMut<Assets<StandardMaterial>>,
    mut registry: ResMut<Registry>
) {
    registry.materials.insert(
        "amber",
        assets.add(StandardMaterial { // reference //
            base_color: Color::from(AMBER_700),
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            thickness: 0.1,
            ior: 1.46,
            ..default()
        }),
    );
}

