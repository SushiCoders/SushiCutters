//! #`FrameBench`
//! This module creates a struct that is used to create a summary
//! of frame times when running a benchmark
use amethyst::core::timing::Stopwatch;
use std::collections::HashMap;
use std::fmt;

/// Writes an advance frame to the bench res that it is attached to
/// when it goes out of scope
pub struct ScopeTimer<'s> {
    watch: Stopwatch,
    bench: &'s mut BenchRes,
}

impl<'s> ScopeTimer<'s> {
    pub fn new(bench: &'s mut BenchRes) -> Self {
        let mut watch = Stopwatch::new();
        watch.start();
        ScopeTimer { watch, bench }
    }
}

impl<'s> Drop for ScopeTimer<'s> {
    fn drop(&mut self) {
        let elapsed = self.watch.elapsed();
        self.bench.advance_frame(elapsed.as_secs_f64());
    }
}

/// Benching data
pub struct BenchRes {
    max: f64,
    min: f64,
    total: f64,
    frame_count: u32,
}

impl BenchRes {
    pub fn advance_frame(&mut self, delta_time: f64) {
        self.frame_count += 1;
        self.total += delta_time;
        self.min = self.min.min(delta_time);
        self.max = self.max.max(delta_time);
    }

    pub fn time_scope(&mut self) -> ScopeTimer {
        ScopeTimer::new(self)
    }
}

impl Default for BenchRes {
    fn default() -> Self {
        Self {
            max: 0_f64,
            min: std::f64::MAX,
            total: 0_f64,
            frame_count: 0,
        }
    }
}

impl fmt::Display for BenchRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let avg_frame = self.total / f64::from(self.frame_count);
        write!(
            f,
            "Avg frame time: {}\nShortest frame time: {}\nLongest frame time: {}\nTotal Time: {}\nFrame Count: {}",
            avg_frame, self.min, self.max, self.total, self.frame_count
        )
    }
}

/// All benching data
#[derive(Default)]
pub struct FrameBench {
    main: BenchRes,
    systems: HashMap<String, BenchRes>,
}

impl FrameBench {
    pub fn advance_frame(&mut self, delta_time: f64) {
        self.main.advance_frame(delta_time);
    }

    pub fn time_scope(&mut self, name: String) -> ScopeTimer {
        let bench = self.systems.entry(name).or_insert_with(BenchRes::default);
        bench.time_scope()
    }

    // Saves to file. Uses an append so that we can have multiple benchmarks
    // in the same file
    //
    // Uses an environment variable for benchmark name OR uses bench.out
    pub fn save_to_file(&self) -> std::io::Result<()> {
        use std::fs::OpenOptions;
        use std::io::prelude::*;
        let file_name: String = std::env::var("BENCHMARK_OUT").unwrap_or("bench.out".to_string());

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();

        writeln!(file, "{}", self)?;
        writeln!(file, "-------------------")?;

        Ok(())
    }
}

impl fmt::Display for FrameBench {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fps = f64::from(self.main.frame_count) / self.main.total;
        write!(f, "Performance summary: \nAvg FPS: {}\n{}", fps, self.main)?;

        for (key, value) in &self.systems {
            write!(f, "\nPerformance for <{}>\n{}", key, value)?;
        }

        Ok(())
    }
}
