use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread::{JoinHandle, spawn};

struct WorkQueue<T> {
    data: Arc<(Mutex<VecDeque<T>>, Condvar)>,
    workers: Vec<Option<JoinHandle<()>>>,
    shutdown: Arc<AtomicBool>,
}

impl<T> WorkQueue<T> where T: Send + FnOnce() -> () + 'static {
    pub fn new(numWorkers: usize) -> Self {
        let data = Arc::new((Mutex::new(vec![].into()), Condvar::new()));
        let mut workers = Vec::with_capacity(numWorkers);
        let shutdown = Arc::new(AtomicBool::new(false));
        
        for _ in 0 .. numWorkers {
            let data = Arc::clone(&data);
            let shutdown = shutdown.clone();
            workers.push(Some(spawn(move || Self::worker(data, shutdown))));
        }
        
        Self { data, workers, shutdown }
    }
    
    pub fn submit(&self, func: T) {
        let mut vec = self.data.0.lock().unwrap();
        vec.push_back(func);
        drop(vec);
        self.data.1.notify_one();
    }
    
    pub fn shutdown(&mut self) {
        self.shutdown.store(true, Ordering::Release);
        
        // notify all with the lock held to ensure every worker is either running tasks,
        // or waiting on the condvar; i.e. not between checking shutdown and the call to wait
        let lock = self.data.0.lock().unwrap();
        self.data.1.notify_all();
        drop(lock);
        
        for handle in &mut self.workers {
            handle.take().map(JoinHandle::join);
        }
    }
    
    fn worker(data: Arc<(Mutex<VecDeque<T>>, Condvar)>, shutdown: Arc<AtomicBool>) {
        loop {
            let mut tasks = data.0.lock().unwrap();
            while tasks.len() == 0 {
                if shutdown.load(Ordering::Acquire) { return; }
                tasks = data.1.wait(tasks).unwrap();
            }
            
            let func = tasks.pop_front().unwrap();
            drop(tasks); // don't hold lock while running task
            func();
        }
    }
}