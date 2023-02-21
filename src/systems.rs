use crate::FrameTimeChart;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{Res, ResMut},
};
use bevy_inspector_egui::bevy_egui::EguiContext;
use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke};

pub fn update_window_system(diagnostics: Res<Diagnostics>, mut window: ResMut<FrameTimeChart>) {
    let frame_time_diagnostic = match diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        Some(it) => it,
        None => return,
    };

    if let (Some(frame_time), Some(average)) = (
        frame_time_diagnostic.value(),
        frame_time_diagnostic.average(),
    ) {
        window.push(frame_time);
        window.update_average_frame_time(average);
    }
}

pub fn framerate_inspector_gui_system(
    mut egui_context: ResMut<EguiContext>,
    window: Res<FrameTimeChart>,
) {
    let history = window.values();
    let count = history.len();

    egui::Area::new("framerate")
        .anchor(Align2::RIGHT_TOP, egui::Vec2::new(0.0, 0.0))
        .show(egui_context.ctx_mut(), |ui| {
            let junk_color = |color_a: Color32, color_b: Color32, t: f32| -> Color32 {
                use egui::Rgba;
                egui::lerp(Rgba::from(color_a)..=Rgba::from(color_b), t).into()
            };

            ui.label(format!("{} FPS", 1000.0 / window.average_frame_time));

            ui.horizontal(|ui| {
                ui.set_height(128.0);
                ui.set_width(256.0);

                let min_frame_rate = 15.0;
                let middle_frame_rate = 30.0;
                let target_frame_rate = 60.0;

                let max_frame_time = 1000.0 / min_frame_rate;
                let middle_frame_time = 1000.0 / middle_frame_rate;
                let target_frame_time = 1000.0 / target_frame_rate;

                let rect = ui.min_rect();
                let width = rect.width() / count as f32;

                for index in 0..count {
                    let frame_time = history[index];
                    let frame_rate = 1000.0 / frame_time;

                    let height = (frame_time as f32 / max_frame_time as f32) * rect.height();
                    let height = if height > rect.height() {
                        rect.height()
                    } else {
                        height
                    };

                    let space_height = rect.height() - height;

                    let overflow = 1.0 - frame_rate / target_frame_rate;
                    let overflow = if overflow < 0.0 { 0.0 } else { overflow };
                    let overflow = if overflow > 1.0 { 1.0 } else { overflow };

                    ui.painter().rect_filled(
                        Rect::from_two_pos(
                            egui::Pos2::new(
                                rect.left() + (width * index as f32),
                                rect.top() + space_height,
                            ),
                            egui::Pos2::new(
                                rect.left() + (width * (index + 1) as f32),
                                rect.bottom(),
                            ),
                        ),
                        0.0,
                        if index == window.index {
                            Color32::GREEN
                        } else {
                            junk_color(Color32::BLUE, Color32::RED, overflow as f32)
                        },
                    );
                }

                let painter = ui.painter();
                let rect = ui.min_rect();

                let target_top = rect.height()
                    - (target_frame_time as f32 / max_frame_time as f32) * rect.height();
                let middle_top = rect.height()
                    - (middle_frame_time as f32 / max_frame_time as f32) * rect.height();

                painter.line_segment(
                    [
                        Pos2::new(rect.left(), rect.top() + target_top),
                        Pos2::new(rect.right(), rect.top() + target_top),
                    ],
                    Stroke::new(1.0, Color32::GREEN),
                );
                painter.text(
                    Pos2::new(rect.left(), rect.top() + target_top),
                    Align2::LEFT_TOP,
                    format!("{target_frame_rate} FPS"),
                    FontId::monospace(12.0),
                    Color32::GREEN,
                );

                painter.line_segment(
                    [
                        Pos2::new(rect.left(), rect.top() + middle_top),
                        Pos2::new(rect.right(), rect.top() + middle_top),
                    ],
                    Stroke::new(1.0, Color32::YELLOW),
                );
                painter.text(
                    Pos2::new(rect.left(), rect.top() + middle_top),
                    Align2::LEFT_TOP,
                    format!("{middle_frame_rate} FPS"),
                    FontId::monospace(12.0),
                    Color32::YELLOW,
                );

                painter.line_segment(
                    [
                        Pos2::new(rect.left(), rect.top()),
                        Pos2::new(rect.right(), rect.top()),
                    ],
                    Stroke::new(1.0, Color32::RED),
                );
                painter.text(
                    Pos2::new(rect.left(), rect.top()),
                    Align2::LEFT_TOP,
                    format!("{min_frame_rate} FPS"),
                    FontId::monospace(12.0),
                    Color32::RED,
                );
            });
        });
}
