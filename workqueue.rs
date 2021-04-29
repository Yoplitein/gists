use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{JoinHandle, spawn};

struct WorkQueue<T> {
    data: Arc<(Mutex<VecDeque<T>>, Condvar)>,
    workers: Vec<JoinHandle<()>>
}

impl<T> WorkQueue<T> where T: Send + FnOnce() -> () + 'static {
    pub fn new(numWorkers: usize) -> Self {
        let data = Arc::new((Mutex::new(vec![].into()), Condvar::new()));
        let mut workers = Vec::with_capacity(numWorkers);
        for _ in 0 .. numWorkers {
            let data = Arc::clone(&data);
            workers.push(spawn(move || Self::worker(data)));
        }
        
        Self { data, workers }
    }
    
    pub fn submit(&self, func: T) {
        let mut vec = self.data.0.lock().unwrap();
        vec.push_back(func);
        drop(vec);
        self.data.1.notify_one();
    }
    
    fn worker(data: Arc<(Mutex<VecDeque<T>>, Condvar)>) {
        loop {
            let mut tasks = data.0.lock().unwrap();
            while tasks.len() == 0 {
                tasks = data.1.wait(tasks).unwrap();
            }
            
            let func = tasks.pop_front().unwrap();
            drop(tasks); // don't hold lock while running task
            func();
        }
    }
}