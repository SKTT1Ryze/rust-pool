//! Worker Implement
use std::{
    sync::Arc,
    sync::Mutex,
    sync::mpsc,
    thread,
};

use super::{
    signal::Signal,
    error::Error,
};

/// Struct of worker  
/// Contains:  
/// + id
/// + inner with cookie
/// + priority (never used in this crate yet)
/// + reference of Receiver
/// + thread to run
/// 
#[allow(dead_code)]
pub struct Worker {
    id: usize,
    inner: Mutex<WorkerInner>,
    // priority use for different algorithms
    priority: usize,
    pub receiver: Arc<Mutex<mpsc::Receiver<Signal>>>,
    thread: Option<thread::JoinHandle<()>>,
}

/// Struct of worker inner  
/// Contains:  
/// + cookie
/// + TODO...
/// 
struct WorkerInner {
    cookie: usize,
}

impl Worker {

    /// Create a new Worker
    /// 
    pub fn new (id: usize, cookie: usize, priority: usize, receiver: Arc<Mutex<mpsc::Receiver<Signal>>>) -> Self {
        Self {
            id,
            inner: Mutex::new(WorkerInner::new(cookie)),
            priority,
            receiver,
            thread: None,
        }
    }

    /// Run this Worker
    /// 
    pub fn run(&mut self, receiver: Arc<Mutex<mpsc::Receiver<Signal>>>) -> Result<(usize, usize), Error>{
        let thread = thread::spawn(
            move || {
                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();
                    match message {
                        Signal::NewTask(task) => {
                            task();
                        },
                        Signal::Terminate => {
                            break;
                        },
                    }
                }
            }
        );
        self.thread = Some(thread);
        Ok((self.id, self.inner.lock().unwrap().cookie))
    }

    /// End this Worker
    /// 
    pub fn end(&mut self) -> Result<(), Error> {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
        Ok(())
    }

    /// Get id of this Worker
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get cookie of this Worker
    #[allow(dead_code)]
    pub fn cookie(&self) -> usize {
        self.inner.lock().unwrap().cookie
    }
}

impl WorkerInner {
    /// Create new WokerInner
    /// 
    pub fn new(cookie: usize) -> Self {
        Self {
            cookie,
        }
    }
}