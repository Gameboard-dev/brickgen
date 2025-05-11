use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, 
    pbr::wireframe::WireframePlugin, 
    window::PresentMode,
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_window_utils::WindowUtilsPlugin;
use bevy_flycam::prelude::*;

pub fn plugins() -> impl PluginGroup {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ViewBox".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    })
    .add(MeshPickingPlugin)
    .add(NoCameraPlayerPlugin)
    .add(EguiPlugin)
    .add(WireframePlugin)
    .add(WindowUtilsPlugin)
    .add(FrameTimeDiagnosticsPlugin)
}