//! Ubicloud API client generated from the vendored upstream OpenAPI description.

pub mod blocking;
pub mod generated;
pub use generated::Client;

/// A credential whose formatting never reveals its contents.
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    /// Store a bearer token without exposing it in `Debug` output.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the credential for configuring an HTTP client.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Secret([REDACTED])")
    }
}

/// Error types may expose their HTTP status so the blocking shim can retry.
pub trait RetryStatus {
    /// Return the HTTP status if this error came from an HTTP response.
    fn status_code(&self) -> Option<u16>;
}

#[cfg(test)]
mod tests {
    use super::Secret;

    #[test]
    fn secret_debug_is_redacted() {
        let secret = Secret::new("very-secret-token");
        let debug = format!("{secret:?}");
        assert_eq!(debug, "Secret([REDACTED])");
        assert!(!debug.contains("very-secret-token"));
    }
}

impl RetryStatus for generated::Error<generated::types::Error> {
    fn status_code(&self) -> Option<u16> {
        generated::Error::status(self).map(|status| status.as_u16())
    }
}

#[cfg(test)]
mod fixture_tests {
    use super::generated::types::Project;

    #[test]
    fn recorded_project_fixture_deserializes() {
        let project: Project = serde_json::from_str(include_str!("../fixtures/project.json"))
            .expect("recorded project fixture matches the API schema");
        assert_eq!(project.name, "fixture-project");
        assert_eq!(project.id.to_string(), "pjkkmx0f2vke4h36nk9cm8v8q0");
    }
}
