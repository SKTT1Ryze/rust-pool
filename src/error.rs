//! Error Handler for Rust Thread Pool

/// Error Type
pub enum Error {
    PoolCreationError,
    PushTaskError,
    PushWrokerError,
    RunWorkerError,
    CookieGenerateError,
}