use bevy::prelude::Plugin;
use chart::FrameTimeChart;
use systems::{framerate_inspector_gui_system, update_window_system};

pub(crate) mod chart;
pub(crate) mod systems;

pub struct FrameTimeInspectorPlugin {
    sample_count: usize,
}
impl Default for FrameTimeInspectorPlugin {
    fn default() -> Self {
        Self { sample_count: 100 }
    }
}
impl Plugin for FrameTimeInspectorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(FrameTimeChart::create(self.sample_count))
            .add_system(update_window_system)
            .add_system(framerate_inspector_gui_system);
    }
}
