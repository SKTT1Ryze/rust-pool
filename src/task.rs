//! Task Implement

/// Task Typedef
pub type Task = Box<dyn FnOnce() + Send + 'static>;
