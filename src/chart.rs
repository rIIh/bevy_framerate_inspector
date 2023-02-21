use bevy::prelude::Resource;

#[derive(Resource)]
pub struct FrameTimeChart {
    pub window: Vec<f64>,
    pub index: usize,
    pub max_count: usize,
    pub average_frame_time: f64,
}

impl FrameTimeChart {
    pub fn create(max_count: usize) -> Self {
        Self {
            window: vec![0.0; max_count],
            index: 0,
            max_count,
            average_frame_time: 0.0,
        }
    }

    pub fn update_average_frame_time(&mut self, value: f64) {
        self.average_frame_time = value;
    }

    #[allow(dead_code)]
    pub fn update_max_count(&mut self, max_count: usize) {
        if self.max_count == max_count {
            return;
        }

        self.max_count = max_count;
        self.window = vec![0.0; max_count];
        self.index = 0;
    }

    pub fn push(&mut self, frame_time: f64) {
        self.window[self.index] = frame_time;

        self.index += 1;
        if self.index >= self.max_count {
            self.index = 0;
        }
    }

    pub fn values(&self) -> Vec<f64> {
        self.window.clone()
    }
}
