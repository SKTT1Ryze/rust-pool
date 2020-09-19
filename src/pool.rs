//! Thread Pool Implement

use super::{
    cookie_generator::CookieGenerator,
    task::Task,
    worker::Worker,
    signal::Signal,
    config::*,
    error::Error,
};

use std:: {
    vec::Vec,
    sync::mpsc,
    sync::Arc,
    sync::Mutex,
};

use log::{
    info,
    error,
};

/// Struct of thread pool
/// Contains:  
/// + task queue
/// + worker queue
/// + broadcast to send signal
/// + cookie generator
/// + max worker number
/// + max task number
/// 
pub struct ThreadPool {
    queue: Vec<Task>,
    workers: Vec<Worker>,
    broadcast: mpsc::Sender<Signal>,
    cookie_box: CookieGenerator,
    //receiver: Arc<Mutex<mpsc::Receiver<Signal>>>,
    max_worker: usize,
    max_task: usize,
}

impl ThreadPool {

    /// Create a ThreadPool
    /// with a specified size
    pub fn new(size: usize) -> Result<Self, Error> {
        match (size > 0) && (size < MAX_WORKER) {
            true => {
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));
                let mut workers = Vec::with_capacity(size);
                let mut cookie_box = CookieGenerator::new();

                for id in 0..size {
                    let cookie = match cookie_box.generate() {
                        Ok(cookie) => cookie,
                        Err(err) => {
                            error!("Generate cookie error.");
                            panic!(err);
                        },
                    };
                    let worker = Worker::new(id, cookie, DEFAULT_PRIORITY, Arc::clone(&receiver));
                    workers.push(worker);
                }
                let queue = Vec::new();
                Ok(
                    Self {
                        queue,
                        workers,
                        broadcast: sender,
                        cookie_box: cookie_box,
                        max_worker: MAX_WORKER,
                        max_task: MAX_TASK,
                    }
                )
            },
            false => {
                error!("Given size to create ThreadPool error.");
                Err(Error::PoolCreationError)
            }
        }
        
    }

    /// Push a worker to the ThreadPool
    /// with a specified id
    pub fn push_worker(&mut self, id: usize, receiver: Arc<Mutex<mpsc::Receiver<Signal>>>) -> Result<(), Error> {
        if self.workers.len() >= self.max_worker {
            error!("Workers queue has been full.");
            return Err(Error::PushWrokerError);
        }
        let new_cookie = match self.cookie_box.generate() {
            Ok(cookie) => cookie,
            Err(err) => {
                error!("Generate cookie error.");
                panic!(err);
            }
        };
        let new_worker = Worker::new(id, new_cookie, DEFAULT_PRIORITY, receiver);
        self.workers.push(new_worker);
        Ok(())
    }


    /// Push a task to the task queue
    /// with a function
    pub fn push_task<F>(&mut self, f: F) -> Result<(), Error> 
        where
            F: FnOnce() + Send + 'static
    {
        if self.queue.len() + 1 > self.max_task {
            return Err(Error::PushTaskError);
        }
        let new_task = Box::new(f);
        self.queue.push(new_task);
        Ok(())
        
    }

    /// Execute the ThreadPool
    /// and run all the workers
    pub fn execute(&mut self) -> Result<(), Error>
    {
        match self.run_all_workers() {
            Ok(_) => {},
            Err(err) => {
                error!("Execute ThreadPool error.");
                panic!(err);
            }
        }
        
        //let task = self.queue.pop().unwrap();
        //self.broadcast.send(Signal::NewTask(task)).unwrap();
        
        while let Some(task) = self.queue.pop() {
            self.broadcast.send(Signal::NewTask(task)).unwrap();
        }

        Ok(())
    }

    /// Run all workers
    /// 
    pub fn run_all_workers(&mut self) -> Result<(), Error> {
        for worker in self.workers.iter_mut() {
            let (id, _cookie) = worker.run(worker.receiver.clone())?;
            info!("Worker with id [{}] and cookie [{}] begin running...", id, worker.cookie());
        }
        Ok(())
    }

    /// Run a worker with id
    /// 
    pub fn run_worker(&mut self, id: usize) -> Result<usize, Error> {
        for worker in self.workers.iter_mut() {
            if worker.id() == id {
                let (id, cookie) = worker.run(worker.receiver.clone())?;
                info!("Worker with id [{}] and cookie [{}] begin running...", id, worker.cookie());
                return Ok(cookie);
            }
        }
        error!("Run worker with id {} error", id);
        Err(Error::RunWorkerError)
    }
    /// Get the size of the ThreadPool
    /// 
    pub fn size(&self) -> usize {
        self.workers.len()
    }

    #[cfg(test)]
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Sending terminate message to all workers...");
        for _ in &mut self.workers {
            self.broadcast.send(Signal::Terminate).unwrap();
        }
        info!("Killing all workers...");
        for worker in &mut self.workers {
            info!("Killing worker [{}]...", worker.id());
            match worker.end() {
                Ok(_) => {
                    println!("Killed worker [{}].", worker.id())
                },
                Err(_err) => {
                    println!("Killing worker [{}] failed.", worker.id())
                },
            }
        }
    }
}
