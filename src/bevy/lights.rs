use bevy::prelude::*;

pub fn spawn_lights(
    mut commands: Commands,
) {
    commands
    .spawn((
        PointLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            intensity: 200.0,
            range: 40.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

