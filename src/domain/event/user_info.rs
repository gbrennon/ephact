use serde::Serialize;

/// User/actor information included in event payloads.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct UserInfo {
    name: String,
    email: String,
    login: String,
}

impl UserInfo {
    pub fn new(name: String, email: String, login: String) -> Self {
        Self { name, email, login }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn login(&self) -> &str {
        &self.login
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let user = UserInfo::new("name".into(), "email".into(), "login".into());

        assert_eq!(user.name(), "name");
        assert_eq!(user.email(), "email");
        assert_eq!(user.login(), "login");
    }
}
