use crate::domain::value_objects::{
    InterfaceMode, Marker, OperationMode, OutputPreferences, Permissions,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Settings {
    default_interface: InterfaceMode,
    permissions: Permissions,
    operation_mode: OperationMode,
    output_preferences: OutputPreferences,
    marker: Marker,
}

impl Settings {
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
