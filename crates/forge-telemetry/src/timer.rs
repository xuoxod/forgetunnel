use std::time::Instant;

/// High-precision execution timer tracking sub-nanosecond / picosecond minutiae
#[derive(Debug, Clone)]
pub struct HighPrecisionTimer {
    start: Instant,
}

impl HighPrecisionTimer {
    /// Start a new high-precision execution timer
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Elapsed time in picoseconds (1 ns = 1,000 ps)
    pub fn elapsed_picoseconds(&self) -> u64 {
        let nanos = self.start.elapsed().as_nanos();
        (nanos.min(u64::MAX as u128 / 1000) as u64) * 1000
    }

    /// Elapsed time in nanoseconds
    pub fn elapsed_nanoseconds(&self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }

    /// Elapsed time in microseconds
    pub fn elapsed_micros(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000.0
    }

    /// Elapsed time in milliseconds
    pub fn elapsed_millis(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000.0
    }
}

impl Default for HighPrecisionTimer {
    fn default() -> Self {
        Self::start()
    }
}
