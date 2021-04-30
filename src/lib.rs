use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

/// A span of time that is started but might not have an end yet.
#[derive(Clone, Debug)]
pub struct TimeSpan {
    /// The instant at which the span started.
    pub start: Instant,
    /// The instant at which the span stopped, if any.
    pub stop: Option<Instant>,
}

/// Converts a TimeSpan into a Duration.
impl Into<Duration> for TimeSpan {
    fn into(self) -> Duration {
        if let Some(stop) = self.stop {
            stop - self.start
        } else {
            self.start.elapsed()
        }
    }
}

// if your last time span doesn't have a stop time, you are still running

/// A stopwatch used to calculate time differences.
/// # Example
/// ```rust
/// use stopwatch2::*;
///
/// let mut s = Stopwatch::default();
/// s.start(); // Starts the stopwatch.
/// s.start(); // Creates a new time span, which are commonly called "splits".
/// s.stop(); // Stops the stopwatch.
/// println!("{}", s); // Prints the total time.
/// println!("{:?}", s); // Prints the different time spans as debug information.
/// let total_time = s.elapsed(); // returns the total time as a Duration.
/// for span in &s.spans {
///     println!("{:?} -> {:?}", span.start, span.stop);
/// }
/// s.spans.clear(); // Reset the stopwatch.
/// println!("{}", s); // Prints the total time.
/// println!("{:?}", s); // Prints the different time spans as debug information.
/// ```
#[derive(Clone, Default, Debug)]
pub struct Stopwatch {
    /// All time spans that this stopwatch has run or is running.
    /// Only the last timespan is allowed to have no stop value, which means it
    /// is still active.
    pub spans: Vec<TimeSpan>,
}

/// Prints the total time this Stopwatch has run.
impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}s", self.elapsed().as_secs_f64());
    }
}

impl Stopwatch {
    /// Starts the stopwatch.
    ///
    /// If it is already started, it will create a new split.
    /// This means it will stop and start the stopwatch, creating a new TimeSpan
    /// in the process.
    pub fn start(&mut self) -> Option<TimeSpan> {
        // if no split or last split is stopped, create new one.
        let ret = self.stop();
        self.spans.push(TimeSpan {
            start: Instant::now(),
            stop: None,
        });
        return ret;
    }

    /// Stops the stopwatch without resetting it.
    pub fn stop(&mut self) -> Option<TimeSpan> {
        let mut ret = None;
        if self.is_running() {
            self.spans.last_mut().unwrap().stop = Some(Instant::now());
            ret = Some(self.spans.last().unwrap().clone());
        }
        return ret;
    }

    /// Returns whether the stopwatch is running.
    pub fn is_running(&self) -> bool {
        // if no spans or last span has an end, we are not running.
        // equiv: if we have splits and the last one has no stop
        !self.spans.is_empty() && self.spans.last().unwrap().stop.is_none()
    }

    /// Returns the total elapsed time accumulated inside of this stopwatch.
    pub fn elapsed(&self) -> Duration {
        // better way to do the conversion here?
        self.spans.iter().map(|s| {let d: Duration = s.clone().into(); d}).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::*;

    static SLEEP_MS: u64 = 50;
    static TOLERANCE_PERCENTAGE: f64 = 0.3;

    #[test]
    fn repeated_stops() {
        let mut sw = Stopwatch::default();
        for _ in 0..1000 {
            sw.start();
        }
        sw.stop();
        assert_eq!(sw.spans.len(), 1000);
        assert!(sw.spans.last().unwrap().stop.is_some());
    }
    
    #[test]
    fn elapsed_none() {
        let mut sw = Stopwatch::default();
        sw.stop();
        sw.stop();
        assert_eq!(sw.elapsed().as_secs_f32(), 0.0);
    }
    
    #[test]
    fn elapsed_ms() {
        let mut sw = Stopwatch::default();
        sw.start();
        sleep_ms(SLEEP_MS);
        assert_duration_near(sw.elapsed(), SLEEP_MS);
    }
    
    #[test]
    fn stop() {
        let mut sw = Stopwatch::default();
        sw.start();
        sleep_ms(SLEEP_MS);
        sw.stop();
        assert_duration_near(sw.elapsed(), SLEEP_MS);
        sleep_ms(SLEEP_MS);
        assert_duration_near(sw.elapsed(), SLEEP_MS);
    }
    
    #[test]
    fn resume_once() {
        let mut sw = Stopwatch::default();
        assert_eq!(sw.spans.len(), 0);
        sw.start();
        assert_eq!(sw.spans.len(), 1);
        sleep_ms(SLEEP_MS);
        sw.stop();
        assert_eq!(sw.spans.len(), 1);
        assert_duration_near(sw.elapsed(), SLEEP_MS);
        sw.start();
        assert_eq!(sw.spans.len(), 2);
        sleep_ms(SLEEP_MS);
        assert_duration_near(sw.elapsed(), 2 * SLEEP_MS);
    }
    
    #[test]
    fn resume_twice() {
        let mut sw = Stopwatch::default();
        assert_eq!(sw.spans.len(), 0);
        sw.start();
        sleep_ms(SLEEP_MS);
        sw.stop();
        assert_eq!(sw.spans.len(), 1);
        assert_duration_near(sw.elapsed(), SLEEP_MS);
        sw.start();
        assert_eq!(sw.spans.len(), 2);
        sleep_ms(SLEEP_MS);
        sw.start();
        assert_eq!(sw.spans.len(), 3);
        assert_duration_near(sw.elapsed(), 2 * SLEEP_MS);
        sw.start();
        assert_eq!(sw.spans.len(), 4);
        sleep_ms(SLEEP_MS);
        assert_duration_near(sw.elapsed(), 3 * SLEEP_MS);
    }
    
    #[test]
    fn is_running() {
        let mut sw = Stopwatch::default();
        assert!(!sw.is_running());
        sw.start();
        assert!(sw.is_running());
        sw.stop();
        assert!(!sw.is_running());
    }
    
    #[test]
    fn reset() {
        let mut sw = Stopwatch::default();
        sw.start();
        sleep_ms(SLEEP_MS);
        sw.spans.clear();
        assert!(!sw.is_running());
        sw.start();
        sleep_ms(SLEEP_MS);
        assert_duration_near(sw.elapsed(), SLEEP_MS);
    }
    
    // helpers
    fn sleep_ms(ms: u64) {
        std::thread::sleep(Duration::from_millis(ms))
    }
    
    fn assert_near(x: i64, y: i64, tolerance: u64) {
        let diff = (x - y).abs() as u64;
        if diff > tolerance {
            panic!("Expected {:?}, got {:?}", x, y);
        }
    }
    
    fn assert_duration_near(duration: Duration, elapsed: u64) {
        let tolerance_value = (TOLERANCE_PERCENTAGE * elapsed as f64) as u64;
        assert_near(elapsed as i64, duration.as_millis() as i64, tolerance_value);
    }
}

