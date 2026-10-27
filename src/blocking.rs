//! Synchronous facade for async client operations.
//!
//! The process shares one Tokio runtime. A call is retried only when its error
//! implements [`crate::RetryStatus`] and reports status 429 or 5xx.

use std::{future::Future, sync::OnceLock, thread, time::Duration};
use tokio::runtime::Runtime;

use crate::generated;
use crate::{RetryStatus, Secret};

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("create shared Tokio runtime for ubicloud-rs"))
}

/// Bounded exponential retry settings.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    /// Maximum total attempts, including the first attempt.
    pub max_attempts: usize,
    /// Delay before the second attempt.
    pub initial_delay: Duration,
    /// Largest delay between attempts.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 4,
            initial_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(3),
        }
    }
}

impl RetryPolicy {
    fn delay(&self, retry_index: usize) -> Duration {
        let multiplier = 1_u32
            .checked_shl(retry_index.min(31) as u32)
            .unwrap_or(u32::MAX);
        self.initial_delay
            .saturating_mul(multiplier)
            .min(self.max_delay)
    }
}

/// Errors produced while constructing an authenticated blocking client.
#[derive(Debug)]
pub enum ClientBuildError {
    /// The bearer credential contains characters invalid in an HTTP header.
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    /// The underlying HTTP client could not be built.
    Http(reqwest::Error),
}

impl std::fmt::Display for ClientBuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeader(error) => {
                write!(formatter, "invalid authentication header: {error}")
            }
            Self::Http(error) => write!(formatter, "unable to build HTTP client: {error}"),
        }
    }
}

impl std::error::Error for ClientBuildError {}

/// A synchronous executor with retry behavior for retryable HTTP failures.
#[derive(Clone)]
pub struct Client {
    inner: generated::Client,
    policy: RetryPolicy,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("blocking::Client")
            .field("inner", &"[REDACTED]")
            .field("policy", &self.policy)
            .finish()
    }
}

impl Client {
    /// Create an executor with default retry settings.
    pub fn new() -> Self {
        Self::with_base_url("https://api.ubicloud.com", RetryPolicy::default())
    }

    /// Create a client for a custom API URL.
    pub fn with_base_url(base_url: &str, policy: RetryPolicy) -> Self {
        Self {
            inner: generated::Client::new(base_url),
            policy,
        }
    }

    /// Create an authenticated client using a credential with redacted Debug output.
    pub fn with_token(
        base_url: &str,
        token: &Secret,
        policy: RetryPolicy,
    ) -> Result<Self, ClientBuildError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.expose()))
            .map_err(ClientBuildError::InvalidHeader)?;
        headers.insert(reqwest::header::AUTHORIZATION, value);
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ClientBuildError::Http)?;
        Ok(Self {
            inner: generated::Client::new_with_client(base_url, http),
            policy,
        })
    }

    /// Borrow the generated async client for selecting API operations.
    pub fn async_client(&self) -> &generated::Client {
        &self.inner
    }

    /// Create an executor around an existing generated client.
    pub fn from_client(inner: generated::Client, policy: RetryPolicy) -> Self {
        Self { inner, policy }
    }

    /// Run an async operation to completion, retrying 429 and 5xx failures.
    ///
    /// The closure is called again for each retry. Use this only for operations
    /// safe to repeat, or when the API supplies an idempotency mechanism.
    pub fn run<T, E, F, Fut>(&self, mut operation: F) -> Result<T, E>
    where
        E: RetryStatus,
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let attempts = self.policy.max_attempts.max(1);
        for attempt in 0..attempts {
            let result = runtime().block_on(operation());
            match result {
                Err(error)
                    if attempt + 1 < attempts
                        && error.status_code().is_some_and(is_retryable_status) =>
                {
                    thread::sleep(self.policy.delay(attempt));
                }
                result => return result,
            }
        }
        unreachable!("at least one attempt always runs")
    }
}

fn is_retryable_status(status: u16) -> bool {
    status == 429 || (500..=599).contains(&status)
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, RetryPolicy, is_retryable_status};
    use crate::RetryStatus;
    use std::{cell::Cell, time::Duration};

    #[derive(Debug)]
    struct FakeError(Option<u16>);

    impl RetryStatus for FakeError {
        fn status_code(&self) -> Option<u16> {
            self.0
        }
    }

    #[test]
    fn authenticated_client_debug_does_not_reveal_token() {
        let secret = crate::Secret::new("credential-value");
        let client = Client::with_token("http://localhost", &secret, RetryPolicy::default())
            .expect("test token is a valid header");
        let debug = format!("{client:?}");
        assert!(!debug.contains("credential-value"));
        assert!(debug.contains("REDACTED"));
    }

    #[test]
    fn retry_statuses_are_limited_to_throttling_and_server_errors() {
        assert!(is_retryable_status(429));
        assert!(is_retryable_status(500));
        assert!(is_retryable_status(599));
        assert!(!is_retryable_status(400));
        assert!(!is_retryable_status(499));
    }

    #[test]
    fn run_retries_retryable_failures_then_returns_success() {
        let calls = Cell::new(0);
        let client = Client::with_base_url(
            "http://localhost",
            RetryPolicy {
                max_attempts: 3,
                initial_delay: Duration::ZERO,
                max_delay: Duration::ZERO,
            },
        );
        let result = client.run(|| {
            let attempt = calls.get() + 1;
            calls.set(attempt);
            async move {
                if attempt < 3 {
                    Err(FakeError(Some(503)))
                } else {
                    Ok("ok")
                }
            }
        });
        assert_eq!(result.expect("third attempt succeeds"), "ok");
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn run_does_not_retry_client_errors() {
        let calls = Cell::new(0);
        let client = Client::with_base_url(
            "http://localhost",
            RetryPolicy {
                max_attempts: 4,
                initial_delay: Duration::ZERO,
                max_delay: Duration::ZERO,
            },
        );
        let result = client.run(|| {
            calls.set(calls.get() + 1);
            async { Err::<(), _>(FakeError(Some(400))) }
        });
        assert!(result.is_err());
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn retry_delay_grows_and_is_capped() {
        let policy = RetryPolicy {
            max_attempts: 4,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(250),
        };
        assert_eq!(policy.delay(0), Duration::from_millis(100));
        assert_eq!(policy.delay(1), Duration::from_millis(200));
        assert_eq!(policy.delay(2), Duration::from_millis(250));
    }
}
