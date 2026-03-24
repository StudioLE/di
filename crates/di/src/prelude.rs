//! Common imports re-exported for convenience.

#[cfg(feature = "async")]
pub use crate::r#async::*;
pub(crate) use crate::extensions::*;
pub use crate::schema::*;
pub use crate::service_builder::*;
pub use crate::service_provider::*;
pub use studiole_di_macros::FromProvider;
#[cfg(feature = "async")]
pub use studiole_di_macros::FromProviderAsync;

pub(crate) use std::any::{Any, TypeId, type_name};
pub(crate) use std::collections::HashMap;
pub(crate) use std::error::Error as StdError;
pub(crate) use std::future::Future;
#[cfg(feature = "traits")]
pub(crate) use std::marker::Unsize;
pub(crate) use std::pin::Pin;
pub(crate) use std::sync::{Arc, Mutex};
pub(crate) use studiole_report::prelude::*;
pub(crate) use thiserror::Error;
pub(crate) use tracing::trace;
