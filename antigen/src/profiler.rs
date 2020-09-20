use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Profiler {
    start_ts: Duration,
}

impl Profiler {
    pub fn start() -> Profiler {
        Profiler {
            start_ts: Self::get_now(),
        }
    }

    pub fn finish(self) -> Duration {
        Self::get_now() - self.start_ts
    }

    fn get_now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
    }
}
