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
    pub fn get_time(&mut self) -> (u128, u128) {
        let time = self.instant.elapsed().as_nanos();
        let time_squared = time * time;
        self.instant = Instant::now();
        (time, time_squared)
    }
}