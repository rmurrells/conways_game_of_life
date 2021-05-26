use std::{
    thread,
    time::{Duration, Instant},
};

pub struct FrameRegulator {
    frame_duration: Duration,
    last: Instant,
}

impl FrameRegulator {
    pub fn new(frame_duration: Duration) -> Self {
        Self {
            frame_duration,
            last: Instant::now(),
        }
    }

    pub fn fps(fps: u64) -> Self {
        Self::new(Duration::from_nanos(1_000_000_000 / fps))
    }

    pub fn regulate(&mut self) {
        let elapsed = self.last.elapsed();
        if elapsed < self.frame_duration {
            thread::sleep(self.frame_duration - elapsed);
        }
        self.last = Instant::now();
    }
}
