use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Profiler {
    name: String,
    start_ts: Duration,
}

impl Profiler {
    pub fn start(name: &str) -> Profiler {
        println!("{} start", &name);

        Profiler {
            name: name.into(),
            start_ts: Self::get_now(),
        }
    }

    pub fn finish(self) -> Duration {
        let delta = Self::get_now() - self.start_ts;
        println!(
            "{} finish. Took {}ms / {}ns / {}us",
            self.name,
            delta.as_millis(),
            delta.as_micros(),
            delta.as_nanos()
        );
        delta
    }

    fn get_now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
    }
}
