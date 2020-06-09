//! #`FrameBench`
//! This module creates a struct that is used to create a summary
//! of frame times when running a benchmark
use std::fmt;

pub struct FrameBench {
    max: f64,
    min: f64,
    total: f64,
    frame_count: u32,
}

impl Default for FrameBench {
    fn default() -> Self {
        Self {
            max: 0_f64,
            min: std::f64::MAX,
            total: 0_f64,
            frame_count: 0,
        }
    }
}

impl FrameBench {
    pub fn advance_frame(&mut self, delta_time: f64) {
        self.frame_count += 1;
        self.total += delta_time;
        self.min = self.min.min(delta_time);
        self.max = self.max.max(delta_time);
    }
}

impl fmt::Display for FrameBench {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fps = f64::from(self.frame_count) / self.total;
        let avg_frame = self.total / f64::from(self.frame_count);
        write!(
            f,
            "Performance summary: \nAvg FPS: {}\nAvg frame time: {}\nShortest frame time: {}\nLongest frame time: {}\nTotal Time: {}",
            fps, avg_frame, self.min, self.max, self.total
        )
    }
}
