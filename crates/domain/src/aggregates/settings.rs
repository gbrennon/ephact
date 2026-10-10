use crate::value_objects::{
    FailureLogRetention, InterfaceMode, Marker, OperationMode, OutputPreferences, Permissions,
};

/// Workflow execution settings for interface, permissions, output, and retention.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Settings {
    default_interface: InterfaceMode,
    permissions: Permissions,
    operation_mode: OperationMode,
    output_preferences: OutputPreferences,
    marker: Marker,
    failure_log_retention: FailureLogRetention,
}

impl Settings {
    /// The default number of hours for retaining failure logs.
    pub const DEFAULT_FAILURE_LOG_RETENTION_HOURS: u64 = FailureLogRetention::DEFAULT_HOURS;

    /// Returns the configured default interface mode.
    pub fn default_interface(&self) -> InterfaceMode {
        self.default_interface
    }

    /// Returns the configured permission settings.
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }

    /// Returns the configured operation mode.
    pub fn operation_mode(&self) -> &OperationMode {
        &self.operation_mode
    }

    /// Returns the configured output preferences.
    pub fn output_preferences(&self) -> &OutputPreferences {
        &self.output_preferences
    }

    /// Returns the configured marker.
    pub fn marker(&self) -> &Marker {
        &self.marker
    }

    /// Returns the configured failure-log retention period.
    pub fn failure_log_retention(&self) -> &FailureLogRetention {
        &self.failure_log_retention
    }

    /// Returns the configured failure-log retention in hours.
    pub fn failure_log_retention_hours(&self) -> u64 {
        self.failure_log_retention.hours()
    }

    /// Returns whether repository writes are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.permissions.allow_repo_writes()
    }

    /// Returns whether real container execution is allowed.
    pub fn allow_real_container(&self) -> bool {
        self.permissions.allow_real_container()
    }

    /// Returns whether real fetcher use is allowed.
    pub fn allow_real_fetcher(&self) -> bool {
        self.permissions.allow_real_fetcher()
    }

    /// Returns whether network access is allowed.
    pub fn allow_network(&self) -> bool {
        self.permissions.allow_network()
    }

    /// Returns whether output should be preserved.
    pub fn preserve(&self) -> bool {
        self.output_preferences.preserve()
    }

    /// Returns whether verbose output is enabled.
    pub fn verbose(&self) -> bool {
        self.output_preferences.verbose()
    }

    /// Returns whether interactive mode is enabled.
    pub fn interactive(&self) -> bool {
        self.operation_mode.interactive()
    }

    /// Returns whether all workflows should be run.
    pub fn all_workflows(&self) -> bool {
        self.operation_mode.all_workflows()
    }

    /// Sets the default interface mode.
    pub fn with_default_interface(mut self, value: InterfaceMode) -> Self {
        self.default_interface = value;
        self
    }

    /// Sets the permission settings.
    pub fn with_permissions(mut self, value: Permissions) -> Self {
        self.permissions = value;
        self
    }

    /// Sets the operation mode.
    pub fn with_operation_mode(mut self, value: OperationMode) -> Self {
        self.operation_mode = value;
        self
    }

    /// Sets the output preferences.
    pub fn with_output_preferences(mut self, value: OutputPreferences) -> Self {
        self.output_preferences = value;
        self
    }

    /// Sets the marker.
    pub fn with_marker(mut self, value: Marker) -> Self {
        self.marker = value;
        self
    }

    /// Sets the failure-log retention period.
    pub fn with_failure_log_retention(self, value: FailureLogRetention) -> Self {
        Self {
            failure_log_retention: value,
            ..self
        }
    }

    /// Sets the failure-log retention period in hours, rejecting zero.
    pub fn with_failure_log_retention_hours(self, hours: u64) -> Result<Self, String> {
        let retention = FailureLogRetention::new(hours)?;
        Ok(self.with_failure_log_retention(retention))
    }

    /// Sets whether repository writes are allowed.
    pub fn with_allow_repo_writes(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_repo_writes(value);
        self.with_permissions(permissions)
    }

    /// Sets whether real container execution is allowed.
    pub fn with_allow_real_container(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_real_container(value);
        self.with_permissions(permissions)
    }

    /// Sets whether real fetcher use is allowed.
    pub fn with_allow_real_fetcher(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_real_fetcher(value);
        self.with_permissions(permissions)
    }

    /// Sets whether network access is allowed.
    pub fn with_allow_network(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_network(value);
        self.with_permissions(permissions)
    }

    /// Sets whether output should be preserved.
    pub fn with_preserve(self, value: bool) -> Self {
        let preferences = self.output_preferences.clone().with_preserve(value);
        self.with_output_preferences(preferences)
    }

    /// Sets whether verbose output is enabled.
    pub fn with_verbose(self, value: bool) -> Self {
        let preferences = self.output_preferences.clone().with_verbose(value);
        self.with_output_preferences(preferences)
    }

    /// Sets whether interactive mode is enabled.
    pub fn with_interactive(self, value: bool) -> Self {
        let mode = self.operation_mode.clone().with_interactive(value);
        self.with_operation_mode(mode)
    }

    /// Sets whether all workflows should be run.
    pub fn with_all_workflows(self, value: bool) -> Self {
        let mode = self.operation_mode.clone().with_all_workflows(value);
        self.with_operation_mode(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_exposes_default_collaborators() {
        let settings = Settings::default();

        assert_eq!(settings.default_interface(), InterfaceMode::Tui);
        assert_eq!(settings.permissions(), &Permissions::default());
        assert_eq!(settings.operation_mode(), &OperationMode::default());
        assert_eq!(settings.output_preferences(), &OutputPreferences::default());
        assert_eq!(settings.marker(), &Marker::default());
    }

    #[test]
    fn default_disables_every_derived_flag() {
        let settings = Settings::default();

        assert!(!settings.allow_repo_writes());
        assert!(!settings.allow_real_container());
        assert!(!settings.allow_real_fetcher());
        assert!(!settings.allow_network());
        assert!(!settings.preserve());
        assert!(!settings.verbose());
        assert!(!settings.interactive());
        assert!(!settings.all_workflows());
    }

    #[test]
    fn with_default_interface_replaces_interface() {
        let settings = Settings::default().with_default_interface(InterfaceMode::Cli);

        assert_eq!(settings.default_interface(), InterfaceMode::Cli);
    }

    #[test]
    fn with_permissions_replaces_permissions() {
        let permissions = Permissions::default().with_allow_network(true);

        let settings = Settings::default().with_permissions(permissions.clone());

        assert_eq!(settings.permissions(), &permissions);
    }

    #[test]
    fn with_operation_mode_replaces_operation_mode() {
        let mode = OperationMode::default().with_interactive(true);

        let settings = Settings::default().with_operation_mode(mode.clone());

        assert_eq!(settings.operation_mode(), &mode);
    }

    #[test]
    fn with_output_preferences_replaces_output_preferences() {
        let preferences = OutputPreferences::default().with_verbose(true);

        let settings = Settings::default().with_output_preferences(preferences.clone());

        assert_eq!(settings.output_preferences(), &preferences);
    }

    #[test]
    fn with_marker_replaces_marker() {
        let marker = Marker::custom_text("ready");

        let settings = Settings::default().with_marker(marker.clone());

        assert_eq!(settings.marker(), &marker);
    }

    #[test]
    fn permission_builders_toggle_derived_flags() {
        let settings = Settings::default()
            .with_allow_repo_writes(true)
            .with_allow_real_container(true)
            .with_allow_real_fetcher(true)
            .with_allow_network(true);

        assert!(settings.allow_repo_writes());
        assert!(settings.allow_real_container());
        assert!(settings.allow_real_fetcher());
        assert!(settings.allow_network());
    }

    #[test]
    fn output_builders_toggle_derived_flags() {
        let settings = Settings::default().with_preserve(true).with_verbose(true);

        assert!(settings.preserve());
        assert!(settings.verbose());
    }

    #[test]
    fn operation_builders_toggle_derived_flags() {
        let settings = Settings::default()
            .with_interactive(true)
            .with_all_workflows(true);

        assert!(settings.interactive());
        assert!(settings.all_workflows());
    }

    #[test]
    fn default_failure_log_retention_is_24_hours() {
        assert_eq!(Settings::default().failure_log_retention_hours(), 24);
    }

    #[test]
    fn positive_failure_log_retention_replaces_default() {
        let settings = Settings::default()
            .with_failure_log_retention_hours(72)
            .expect("positive retention is valid");

        assert_eq!(settings.failure_log_retention_hours(), 72);
    }

    #[test]
    fn zero_failure_log_retention_is_rejected() {
        let error = Settings::default()
            .with_failure_log_retention_hours(0)
            .expect_err("zero retention must be invalid");

        assert!(error.contains("greater than zero"));
    }

    #[test]
    fn settings_store_failure_log_retention_as_a_value_object() {
        let retention = crate::value_objects::FailureLogRetention::new(72)
            .expect("positive retention is valid");
        let settings = Settings::default().with_failure_log_retention(retention);

        assert_eq!(settings.failure_log_retention(), &retention);
    }
}
