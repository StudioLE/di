use crate::prelude::*;
use governor::{
    Jitter, Quota, RateLimiter, clock::DefaultClock, state::InMemoryState, state::NotKeyed,
};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::Duration;
use tokio::sync::RwLock;

const RATE_LIMIT_PER_SECOND: u32 = 10;

/// Service for enforcing per-domain HTTP rate limits
#[derive(Clone)]
pub struct HttpRateLimiter {
    /// Per-domain rate limiters (lazily created)
    limiters: Arc<RwLock<HashMap<String, Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>>>>,
    /// Quota configuration (same for all domains)
    quota: Quota,
}

impl Service for HttpRateLimiter {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Infallible>> {
        Ok(Self::default())
    }
}

impl Default for HttpRateLimiter {
    fn default() -> Self {
        Self {
            limiters: Arc::default(),
            quota: Quota::per_second(
                NonZeroU32::new(RATE_LIMIT_PER_SECOND).expect("rate limit is not zero"),
            ),
        }
    }
}

impl HttpRateLimiter {
    /// Create a new rate limiter with the specified requests per second
    ///
    /// # Panics
    ///
    /// Panics if `rate_per_second` is zero
    #[must_use]
    pub fn new(rate_per_second: u32) -> Self {
        let rate = NonZeroU32::new(rate_per_second).expect("rate_per_second must be non-zero");
        let quota = Quota::per_second(rate);
        Self {
            limiters: Arc::new(RwLock::new(HashMap::new())),
            quota,
        }
    }

    /// Wait until a request can proceed for the given domain
    ///
    /// This method blocks asynchronously until the rate limiter allows the request.
    /// Uses jitter to prevent thundering herd when multiple tasks are waiting.
    pub async fn wait_for_permit(&self, domain: &str) {
        let limiter = self.get_limiter(domain).await;
        limiter
            .until_ready_with_jitter(Jitter::up_to(Duration::from_millis(10)))
            .await;
    }

    /// Get or create a rate limiter for the given domain
    ///
    /// Uses double-check locking pattern to avoid unnecessary write locks
    async fn get_limiter(
        &self,
        domain: &str,
    ) -> Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>> {
        let limiters = self.limiters.read().await;
        if let Some(limiter) = limiters.get(domain) {
            return Arc::clone(limiter);
        }
        drop(limiters);
        let mut limiters = self.limiters.write().await;
        if let Some(limiter) = limiters.get(domain) {
            return Arc::clone(limiter);
        }
        let limiter = Arc::new(RateLimiter::direct(self.quota));
        limiters.insert(domain.to_owned(), Arc::clone(&limiter));
        trace!(domain, "Created rate limiter");
        limiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn rate_limiter_enforces_limit() {
        // Arrange
        let limiter = HttpRateLimiter::new(5);
        let start = Instant::now();

        // Act
        for _ in 0..10 {
            limiter.wait_for_permit("example.com").await;
        }

        // Assert
        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_millis(900),
            "Expected rate limiting delay, elapsed: {elapsed:?}"
        );
    }

    #[tokio::test]
    async fn rate_limiter_domain_isolation() {
        // Arrange
        let limiter = HttpRateLimiter::new(5);
        for _ in 0..10 {
            limiter.wait_for_permit("domain1.com").await;
        }

        // Act
        let start = Instant::now();
        for _ in 0..5 {
            limiter.wait_for_permit("domain2.com").await;
        }

        // Assert
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_millis(50),
            "Expected instant access to domain2, elapsed: {elapsed:?}"
        );
    }

    #[tokio::test]
    async fn rate_limiter_creates_separate_limiters_per_domain() {
        // Arrange
        let limiter = HttpRateLimiter::new(5);

        // Act
        limiter.wait_for_permit("a.com").await;
        limiter.wait_for_permit("b.com").await;
        limiter.wait_for_permit("a.com").await;

        // Assert
        let limiters = limiter.limiters.read().await;
        assert_eq!(limiters.len(), 2);
        assert!(limiters.contains_key("a.com"));
        assert!(limiters.contains_key("b.com"));
    }

    #[tokio::test]
    async fn rate_limiter_concurrent_access() {
        // Arrange
        let limiter = Arc::new(HttpRateLimiter::new(5));
        let limiter1 = Arc::clone(&limiter);
        let limiter2 = Arc::clone(&limiter);

        // Act
        let task1 = tokio::spawn(async move {
            for _ in 0..5 {
                limiter1.wait_for_permit("concurrent.com").await;
            }
        });

        let task2 = tokio::spawn(async move {
            for _ in 0..5 {
                limiter2.wait_for_permit("concurrent.com").await;
            }
        });
        let start = Instant::now();
        let _ = tokio::join!(task1, task2);

        // Assert
        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_millis(900),
            "Expected rate limiting across tasks, elapsed: {elapsed:?}"
        );
    }
}
