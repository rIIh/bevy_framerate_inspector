use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_framerate_inspector::FrameTimeInspectorPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin) // todo(melvspace): replace with own framerate tracer
        .add_plugin(FrameTimeInspectorPlugin::default())
        .run();
}
