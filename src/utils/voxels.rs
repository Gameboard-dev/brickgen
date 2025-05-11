
use bevy::{asset::RenderAssetUsages, input::mouse::MouseButtonInput, prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use voxelis::{spatial::{VoxOpsBatch, VoxOpsConfig, VoxOpsDirty, VoxOpsWrite}, world::{VoxChunk, VoxModel, VoxWorld}, Batch, Lod, MaxDepth, VoxInterner};

use crate::bevy::registry::Registry;

pub const HIGHEST_RESOLUTION: u8 = 0;
const MAX_ALLOWED_DEPTH: usize = 7;
const VOXELS_PER_AXIS: usize = 1 << MAX_ALLOWED_DEPTH;
const FILLED: i32 = 1;


// Going to directly regenerate mesh rather than relying on events during testing
// Going to manually draw the 3D 'chunk' edges so I can see them in the Bevy world
// Going to figure out how to 
//      A: have a world with N * N Chunks and 
//      B: map Bevy cursor coordinates for placing voxels to Chunks and Chunk coordinates

pub struct VoxConfig {
    /// The maximum depth of the voxel tree determines the level of detail 
    /// and the number of voxels per axis in each chunk.
    pub max_depth: MaxDepth,
    /// A scaling factor which determines the size of a chunk in the world
    pub chunk_world_size: f32,
    /// The bounds of the voxel grid in terms of chunks. The
    /// number of chunks along each axis (x, y, z).
    pub world_bounds: IVec3,
    /// The memory budget for the `VoxInterner` in bytes.
    pub memory: usize,
    /// LOD represents the resolution of voxel data
    /// Higher values mean lower resolution (coarser data).
    pub lod: Lod
}

/// Wraps `VoxModel` as a Bevy `Resource`
#[derive(Resource)]
pub struct VoxBevy {
    pub model: VoxModel,
    pub lod: Lod,
}

impl VoxBevy {
    pub fn new(
        config: VoxConfig,
    ) -> Self {
        let mut model = VoxModel::with_dimensions(
            config.max_depth,
            config.chunk_world_size,
            config.world_bounds,
            config.memory,
        );

        Self {
            model,
            lod: config.lod
        }
    }

    // Uses integer division to retrieve the chunk the cursor/position is in adjusted for world size scaling
    fn get_chunk(
        &mut self,
        position: Vec3,
        scale: f32,
    ) -> Option<&mut VoxChunk> {
        let chunk_origin = IVec3::new(
            (position.x / scale).floor() as i32,
            (position.y / scale).floor() as i32,
            (position.z / scale).floor() as i32,
        );
        if let Some(chunk) = self.model.chunks.get_mut(&chunk_origin) {
            return Some(chunk)
        }
        //println!("No chunk at position '{:?}'", chunk_origin);
        None
    }

    // Retrieves voxel coordinates within a chunk for world coordinates
    fn get_voxel(
        lod: Lod,
        scale: f32,
        chunk: &mut VoxChunk,
        position: Vec3,
    ) -> Vec3 {

            let chunk_world_origin = chunk.get_position().as_vec3() * scale;

            let normalized_dif = (position - chunk_world_origin) / scale;

            let voxels_per_axis = chunk.voxels_per_axis(lod) as f32;

            normalized_dif * voxels_per_axis
    }

    pub fn get_chunk_and_voxel(
        &mut self,
        position: Vec3,
    ) -> Option<(&mut VoxChunk, Vec3)> {
        let scale = self.model.chunk_world_size;
        let lod = self.lod;
        if let Some(mut chunk) = self.get_chunk(position, scale) {
            let voxel_position = {
                Self::get_voxel(lod, scale, chunk, position)
            };
            return Some((chunk, voxel_position));
        }
        None
    }

    pub fn put_voxel(
        &mut self,
        position: Vec3,
        material: Handle<StandardMaterial>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut registry: ResMut<Registry>, // HashMaps of handles for Materials and Meshes
        mut commands: Commands,
    ) {
        let interner = &mut self.model.get_interner();
        let lod = self.lod;

        // Need some way of managing X*Y*Z voxel placements 
        // When exceeds voxels_per_axis, find new chunk
        // If there is a new chunk, place voxels there (and do recursive checks)

        if let Some((chunk, voxel_position)) = self.get_chunk_and_voxel(position) {
                    
            chunk.set(
                &mut interner.write(),
                voxel_position.as_ivec3(),
                FILLED,
            );

            regenerate_mesh(
                chunk,
                lod,
                &mut interner.write(),
                material,
                &mut meshes,
                &mut registry,
                &mut commands,
            );
            
        }
         
    }

    /*
    pub fn put_sphere(
        &mut self,
        position: Vec3,
        material: Handle<StandardMaterial>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut registry: ResMut<Registry>, // HashMaps of handles for Materials and Meshes
        mut commands: Commands,
        regenerate: bool
    ) {

        let interner = &mut self.model.get_interner();
        let lod = self.lod;

        if let Some((chunk, voxel_position)) = self.get_chunk_and_voxel(position) {
                  
            let (cx, cy, cz) = (center.x, center.y, center.z);
            let radius_squared = radius * radius;

            let voxels_per_axis = chunk.voxels_per_axis(Lod::new(0)) as i32;

            let mut position = IVec3::ZERO;

            let mut batch = chunk.create_batch();

            for y in 0..voxels_per_axis {
                position.y = y;
                for z in 0..voxels_per_axis {
                    position.z = z;
                    for x in 0..voxels_per_axis {
                        position.x = x;
                        let dx = x - cx;
                        let dy = y - cy;
                        let dz = z - cz;

                        let distance_squared = dx * dx + dy * dy + dz * dz;

                        if distance_squared <= radius_squared {

                            let chunk = self.get_chunk(position, scale);

                            
                            batch.set(&mut interner.write(), position, FILLED);
                        }
                    }
                }
            }
        }

        chunk.apply_batch(interner, &batch);

    }
     */
}
    

pub fn regenerate_mesh(
    chunk: &mut VoxChunk,
    lod: Lod,
    interner: &mut VoxInterner<i32>,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    registry: &mut ResMut<Registry>, 
    commands: &mut Commands,
) {

    // Holds chunk mesh data for modification
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    let offset = Vec3::new(0.0, 0.0, 0.0);

    chunk.generate_mesh_arrays(
        interner,
        &mut vertices,
        &mut normals,
        &mut indices,
        offset,
        lod,
    );

    let chunk_origin = chunk.get_position();

    let mut old_mesh = registry.meshes.remove(&chunk_origin);

    /* // DEBUGGING
    println!(
        "{} mesh: {} vertices, {} normals, {} indices",
        if old_mesh.is_some() { "Regenerated" } else { "Generated" },
        vertices.len(),
        normals.len(),
        indices.len(),
    );
     */

    // Remove the old handle from the registry and the old mesh from Bevy
    if let Some(old_mesh) = old_mesh {
        meshes.remove(&old_mesh);
    }

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::all(), // https://github.com/bevyengine/bevy/issues/17987
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    // Handle (reference) for the new Mesh
    let mesh_ref = meshes.add(mesh);

    // Add the new mesh to the registry
    registry.meshes.insert(chunk_origin, mesh_ref.clone());

    // Retrieves chunk to world units
    let world_position: Vec3 = chunk.get_world_position();

    commands
        .spawn((
            Mesh3d(mesh_ref.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(world_position),
        ))
        .insert(Name::new("Voxels".to_string()));
}


pub fn show_chunks(
    mut vox: ResMut<VoxBevy>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut registry: ResMut<Registry>, // Contains HashMaps of handles for Materials and Meshes
    mut commands: Commands,
    mut gizmos: Gizmos
) {

    // The size of a chunk in world space
    let chunk_size = vox.model.chunk_world_size;

    // Iterating over a HashMap of (chunk_origin, chunk)
    for (chunk_origin, chunk) in &mut vox.model.chunks {

        // The world space position of the chunk (bottom-left corner)
        let mut world_position: Vec3 = chunk.get_world_position(); // {x: f32, y: f32, z: 32}

        let corners: Vec<Vec3> = (0..8) // 8 corners in a cuboid
            .map(|i| {
                world_position
                    + Vec3::new(
                        // Avoids hardcoding each corner manually.
                        // Uses the binary representation of i.
                        if i & 1 != 0 { chunk_size } else { 0.0 }, // X
                        if i & 2 != 0 { chunk_size } else { 0.0 }, // Y
                        if i & 4 != 0 { chunk_size } else { 0.0 }, // Z
                    )
            })
            .collect();

        // Pairs of corners to draw lines between
        let edges = [
            (0, 1), (1, 3), (3, 2), (2, 0), // Front face
            (4, 5), (5, 7), (7, 6), (6, 4), // Back face
            (0, 4), (1, 5), (2, 6), (3, 7), // Connecting edges
        ];

        // Draw lines between corners using their indices in the corners array
        for &(start, end) in &edges {
            gizmos.line(
                corners[start],
                corners[end],
                Color::srgba(1.0, 0.0, 0.0, 1.0), // Red
            );
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn initialize_voxel_grid() {

        let config = VoxConfig {
            max_depth: MaxDepth::new(6), // 6 voxels per axis per chunk
            chunk_world_size: 1.28, 
            world_bounds: IVec3::new(4, 4, 4), // 4x4x4 chunks
            memory: 1024 * 1024 * 256, // Example: 256 MB
            lod: Lod::new(HIGHEST_RESOLUTION)
        };

        let voxbevy = VoxBevy::new(config);

        // Assert that the VoxelGrid has the expected number of chunks
        assert_eq!(voxbevy.model.get_bounds_size(), (4 * 4 * 4) as usize);

        println!("VoxelGrid initialized successfully!");
    }
}