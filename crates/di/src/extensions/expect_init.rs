//! Panicking service initialization.
use crate::prelude::*;

impl ServiceProvider {
    /// Run all registered init closures or panic.
    #[must_use]
    #[track_caller]
    pub fn expect_init(self) -> Self {
        self.init().expect("should be able to init services")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_provider_expect_init() {
        let _services = ServiceBuilder::new().build().expect_init();
    }
}
