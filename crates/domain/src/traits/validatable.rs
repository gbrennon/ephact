/// Provides a consistent validity check for domain values.
pub trait Validatable {
    /// Returns whether the value satisfies its domain invariants.
    fn is_valid(&self) -> bool;
}
