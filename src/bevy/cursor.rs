use bevy::{picking::pointer::PointerInteraction, prelude::*};

#[derive(Resource)]
pub struct RayCursor {
    position: Option<Vec3>
}
impl RayCursor {
    pub fn new() -> Self {
        Self {position: None}
    }
    pub fn set(&mut self, position: Option<Vec3>) {
        self.position = position;
    }
    pub fn get(&self) -> Option<Vec3> {
        self.position
    }
    pub fn draw(&self, mut gizmos: Gizmos) {
        if let Some(position) = self.position {
            gizmos
                .sphere(
                    Isometry3d::new(position, Quat::IDENTITY),
                    0.001, // Radius of the sphere
                    Color::srgba(1.0, 0.0, 0.0, 1.0), // Red color
            );
        }
    }
}

/// https://bevyengine.org/examples-webgpu/3d-rendering/3d-viewport-to-world/
pub fn update_cursor(
    mut cursor: ResMut<RayCursor>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    pointers: Query<&PointerInteraction>,
    windows: Query<&Window>,
    gizmos: Gizmos
) {
    let mut position = None;
    if let (Ok(window), Ok((camera, camera_transform))) = (windows.get_single(), camera_query.get_single()) {
        if let Some(cursor_position) = window.cursor_position() {
            // Raytrace onto the nearest mesh hitpoint
            for (point, normal) in pointers
                .iter()
                .filter_map(|interaction| interaction.get_nearest_hit())
                .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
            {
                //println!("Mesh Hit Detected");
                position = Some(point);
            }
            if position.is_none() {
                // Raytrace onto the infinite plane if there is no mesh
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) {
                        position = Some(ray.get_point(distance)); // f32
                    }
                }
            }
        }
    }
    cursor.position = position;
    cursor.draw(gizmos);
}
