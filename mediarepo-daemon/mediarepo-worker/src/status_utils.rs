pub struct SimpleProgress {
    pub current: u64,
    pub total: u64,
}

impl Default for SimpleProgress {
    fn default() -> Self {
        Self {
            total: 100,
            current: 0,
        }
    }
}

impl SimpleProgress {
    pub fn new(total: u64) -> Self {
        Self { total, current: 0 }
    }

    /// Sets the total count
    pub fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    /// Increments the current progress by 1
    pub fn tick(&mut self) {
        self.current += 1;
    }

    /// Sets the current progress to a defined value
    pub fn set_current(&mut self, current: u64) {
        self.current = current;
    }

    /// Returns the total progress in percent
    pub fn percent(&self) -> f64 {
        (self.current as f64) / (self.total as f64)
    }
}
