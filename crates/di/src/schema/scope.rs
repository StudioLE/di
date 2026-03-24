//! Service scope configuration.

/// Control how a service is cached after construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Scope {
    /// Cache after first construction, return same instance on subsequent calls.
    Singleton,
    /// Construct a fresh instance on every resolution.
    Transient,
}
