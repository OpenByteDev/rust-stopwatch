use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

// default zero
// can start
// can check time
// can split
// can temporarily see split time
// can check time since start
// can check all splits times
// can stop
// can check elapsed time until stop
// can check all splits
//
// [start]...[split1]...[split2]...[pause]   [start]...[split3]
// 
// total / elapsed since start
// [                                     ] + [                ]

#[derive(Clone, Debug)]
pub struct TimeSplit {
    start: Instant,
    stop: Option<Instant>,
}

impl Into<Duration> for TimeSplit {
    fn into(self) -> Duration {
        if let Some(stop) = self.stop {
            stop - self.start
        } else {
            self.start.elapsed()
        }
    }
}

// if your last time split doesn't have a stop time, you are still running

/// A stopwatch used to calculate time differences.
#[derive(Clone, Default, Debug)]
pub struct Stopwatch {
    /// The time the stopwatch was split last, if ever.
    splits: Vec<TimeSplit>,
    /// The time elapsed while the stopwatch was running (between start() and stop()).
    elapsed: Duration,
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}s", self.elapsed().as_secs_f64());
    }
}

impl Stopwatch {
    /// Creates a new `Stopwatch`.
    pub fn new() -> Stopwatch {
        Default::default()
    }

    /// Starts the stopwatch.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stops the stopwatch without resetting it.
    pub fn stop(&mut self) -> Duration {
        self.elapsed = self.elapsed();
        self.start_time = None;
        self.splits.clear();
        self.elapsed
    }

    /// Resets all counters and stops the stopwatch.
    pub fn reset(&mut self) {
        self.elapsed = Duration::from_secs(0);
        self.start_time = None;
        self.splits.clear();
    }

    /// Resets and starts the stopwatch again.
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    /// Returns whether the stopwatch is running.
    pub fn is_running(&self) -> bool {
        return self.start_time.is_some();
    }

    /// Returns the elapsed time since the start of the stopwatch.
    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            // stopwatch is running
            Some(t1) => {
                return t1.elapsed() + self.elapsed;
            }
            // stopwatch is not running
            None => {
                return self.elapsed;
            }
        }
    }

    /// Returns the elapsed time since last split or start/restart.
    ///
    /// If the stopwatch is in stopped state this will always return a zero Duration.
    pub fn split(&mut self) -> Duration {
        match self.start_time {
            // stopwatch is running
            Some(start) => {
                let res = match self.splits.last() {
                    Some(split) => split.elapsed(),
                    None => start.elapsed(),
                };
                self.splits.push(Instant::now());
                res
            }
            // stopwatch is not running
            None => Duration::from_secs(0),
        }
    }
}
