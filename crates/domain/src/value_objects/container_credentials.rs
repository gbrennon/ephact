/// Credentials for pulling a container image from a private registry.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerCredentials {
    username: String,
    password: String,
}

impl ContainerCredentials {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let credentials = ContainerCredentials::new("user", "password");

        assert_eq!(credentials.username(), "user");
        assert_eq!(credentials.password(), "password");
    }
}
