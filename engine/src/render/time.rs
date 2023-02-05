use std::time::{Duration, Instant};

pub struct Time {
    last_time: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }

    pub fn time_delta(&self) -> Duration {
        Instant::now().duration_since(self.last_time)
    }

    pub fn delta_seconds_f64(&self) -> f64 {
        self.time_delta().as_secs_f64()
    }

    pub fn delta_seconds_f32(&self) -> f32 {
        self.time_delta().as_secs_f32()
    }

    pub fn post_update(&mut self) {
        self.last_time = Instant::now();
    }
}
