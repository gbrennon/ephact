use crate::value_objects::{InterfaceMode, Marker, OperationMode, OutputPreferences, Permissions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    default_interface: InterfaceMode,
    permissions: Permissions,
    operation_mode: OperationMode,
    output_preferences: OutputPreferences,
    marker: Marker,
    failure_log_retention_hours: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_interface: InterfaceMode::default(),
            permissions: Permissions::default(),
            operation_mode: OperationMode::default(),
            output_preferences: OutputPreferences::default(),
            marker: Marker::default(),
            failure_log_retention_hours: Self::DEFAULT_FAILURE_LOG_RETENTION_HOURS,
        }
    }
}

impl Settings {
    pub const DEFAULT_FAILURE_LOG_RETENTION_HOURS: u64 = 24;

    pub fn default_interface(&self) -> InterfaceMode {
        self.default_interface
    }

    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }

    pub fn operation_mode(&self) -> &OperationMode {
        &self.operation_mode
    }

    pub fn output_preferences(&self) -> &OutputPreferences {
        &self.output_preferences
    }

    pub fn marker(&self) -> &Marker {
        &self.marker
    }

    pub fn failure_log_retention_hours(&self) -> u64 {
        self.failure_log_retention_hours
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.permissions.allow_repo_writes()
    }

    pub fn allow_real_container(&self) -> bool {
        self.permissions.allow_real_container()
    }

    pub fn allow_real_fetcher(&self) -> bool {
        self.permissions.allow_real_fetcher()
    }

    pub fn allow_network(&self) -> bool {
        self.permissions.allow_network()
    }

    pub fn preserve(&self) -> bool {
        self.output_preferences.preserve()
    }

    pub fn verbose(&self) -> bool {
        self.output_preferences.verbose()
    }

    pub fn interactive(&self) -> bool {
        self.operation_mode.interactive()
    }

    pub fn all_workflows(&self) -> bool {
        self.operation_mode.all_workflows()
    }

    pub fn with_default_interface(mut self, value: InterfaceMode) -> Self {
        self.default_interface = value;
        self
    }

    pub fn with_permissions(mut self, value: Permissions) -> Self {
        self.permissions = value;
        self
    }

    pub fn with_operation_mode(mut self, value: OperationMode) -> Self {
        self.operation_mode = value;
        self
    }

    pub fn with_output_preferences(mut self, value: OutputPreferences) -> Self {
        self.output_preferences = value;
        self
    }

    pub fn with_marker(mut self, value: Marker) -> Self {
        self.marker = value;
        self
    }

    pub fn with_failure_log_retention_hours(self, hours: u64) -> Result<Self, String> {
        if hours == 0 {
            return Err("failure log retention hours must be greater than zero".to_string());
        }

        Ok(Self {
            failure_log_retention_hours: hours,
            ..self
        })
    }

    pub fn with_allow_repo_writes(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_repo_writes(value);
        self.with_permissions(permissions)
    }

    pub fn with_allow_real_container(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_real_container(value);
        self.with_permissions(permissions)
    }

    pub fn with_allow_real_fetcher(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_real_fetcher(value);
        self.with_permissions(permissions)
    }

    pub fn with_allow_network(self, value: bool) -> Self {
        let permissions = self.permissions.clone().with_allow_network(value);
        self.with_permissions(permissions)
    }

    pub fn with_preserve(self, value: bool) -> Self {
        let preferences = self.output_preferences.clone().with_preserve(value);
        self.with_output_preferences(preferences)
    }

    pub fn with_verbose(self, value: bool) -> Self {
        let preferences = self.output_preferences.clone().with_verbose(value);
        self.with_output_preferences(preferences)
    }

    pub fn with_interactive(self, value: bool) -> Self {
        let mode = self.operation_mode.clone().with_interactive(value);
        self.with_operation_mode(mode)
    }

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
}
