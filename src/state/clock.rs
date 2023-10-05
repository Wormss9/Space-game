use std::time::Instant;

pub struct Clock {
    instant: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
    pub fn get_time(&mut self) -> (f64, f64) {
        let time = self.instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;
        let time_squared = time * time;
        self.instant = Instant::now();
        (time, time_squared)
    }
}
