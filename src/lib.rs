//#![deny(missing_docs)]
//! # A thread pool written with Rust
//! 
//! - Create a thread pool to run your code
//! - Use log crate to print log
//! - [TODO] Support different scheduling algorithm
//! 
//! 
//! ## Example
//! ```Rust
//! use rust_pool::ThreadPool;
//! 
//! // Create new ThreadPool
//! let mut new_pool = match ThreadPool::new(5) {
//!     Ok(pool) => pool,
//!     Err(_) => {
//!         panic!("Create new ThreadPool error");
//!     },
//! };
//! 
//! // Add your code to the pool
//! for i in 0..8 {
//!     match new_pool.push_task(move ||
//!     {
//!            println!("Task {} is running...", i);
//!     }) {
//!         Err(err) => {
//!             panic!(err);
//!         },
//!         _ => {},
//!     }
//! }
//! 
//! // Execute the pool
//! match new_pool.execute() {
//!     Ok(_) => {},
//!     Err(err) => panic!(err),
//! }
//! ```
//! 
//! See more details in example/src/main.rs  
//! 
//! ## Dependencies
//! + random-number = "0.1.3"
//! 
//! ## TODO
//! + Add algorithm module for supporting different scheduling algorithm
//! 
//! ## **Welcome to contribute!**
//! 
pub mod pool;
pub mod worker;
pub mod cookie_generator;
pub mod task;
pub mod signal;
pub mod config;
pub mod error;

pub use pool::*;

extern crate random_number;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::pool::ThreadPool;

        // Create new ThreadPool
        let mut new_pool = match ThreadPool::new(5) {
            Ok(pool) => pool,
            Err(_) => {
                panic!("New Threadpool error");
            },
        };
        assert_eq!(new_pool.size(), 5);

        // Add your code to the pool
        for i in 0..8 {
            match new_pool.push_task(move || {
                println!("task {}", i);
            }) {
                Ok(_) => {},
                Err(err) => panic!(err),
            }
        }
        assert_eq!(new_pool.queue_len(), 8);
        
        // Execute the pool
        match new_pool.execute() {
            Ok(_) => {},
            Err(err) => panic!(err),
        }
    }
}

