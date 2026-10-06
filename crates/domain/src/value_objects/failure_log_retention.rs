/// Validates the number of hours that failure logs remain available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailureLogRetention {
    hours: u64,
}

impl FailureLogRetention {
    /// The default failure-log retention period in hours.
    pub const DEFAULT_HOURS: u64 = 24;

    /// Creates a retention value for a positive number of hours.
    ///
    /// # Errors
    ///
    /// Returns an error when `hours` is zero.
    pub fn new(hours: u64) -> Result<Self, String> {
        if hours == 0 {
            return Err("failure log retention hours must be greater than zero".to_string());
        }

        Ok(Self { hours })
    }

    /// Returns the retention duration in hours.
    pub fn hours(&self) -> u64 {
        self.hours
    }
}

/// Provides the default failure-log retention period.
impl Default for FailureLogRetention {
    fn default() -> Self {
        Self::new(Self::DEFAULT_HOURS).expect("the default retention is positive")
    }
}

#[cfg(test)]
mod tests {
    use super::FailureLogRetention;

    #[test]
    fn positive_hours_create_retention_value() {
        let retention = FailureLogRetention::new(24).unwrap();

        assert_eq!(retention.hours(), 24);
    }

    #[test]
    fn zero_hours_are_rejected() {
        let result = FailureLogRetention::new(0);

        assert_eq!(
            result,
            Err("failure log retention hours must be greater than zero".to_string())
        );
    }

    #[test]
    fn maximum_hours_are_accepted() {
        let retention = FailureLogRetention::new(u64::MAX).unwrap();

        assert_eq!(retention.hours(), u64::MAX);
    }
}
