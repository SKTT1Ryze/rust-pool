/// main.rs

use rust_pool::ThreadPool;

fn main () {
    // Create new ThreadPool
    let mut new_pool = match ThreadPool::new(5) {
        Ok(pool) => pool,
        Err(_) => {
            panic!("Create new ThreadPool error");
        },
    };

    // Add your code to the pool
    for i in 0..8 {
        match new_pool.push_task(move || 
        {
            println!("Task {} is running...", i);
        }) {
            Err(err) => {
                panic!(err);
            },
            _ => {},
        }
    }
    
    // Execute the pool
    match new_pool.execute() {
            Ok(_) => {},
            Err(err) => panic!(err),
    }
}