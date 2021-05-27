use std::{
    error::Error,
    fmt, thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub struct FrameRegulator {
    frame_duration: Duration,
    last: Instant,
}

#[derive(Debug)]
pub struct ZeroFps;
impl fmt::Display for ZeroFps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ZeroFps {}

impl FrameRegulator {
    pub fn new(frame_duration: Duration) -> Self {
        Self {
            frame_duration,
            last: Instant::now(),
        }
    }

    pub fn fps(fps: u64) -> Result<Self, ZeroFps> {
        if fps == 0 {
            Err(ZeroFps)
        } else {
            Ok(Self::new(Duration::from_nanos(1_000_000_000 / fps)))
        }
    }

    pub fn regulate(&mut self) {
        let elapsed = self.last.elapsed();
        if elapsed < self.frame_duration {
            thread::sleep(self.frame_duration - elapsed);
        }
        self.last = Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        self.last.elapsed()
    }
}
