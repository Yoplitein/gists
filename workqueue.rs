use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread::{JoinHandle, spawn};

type WorkFn = Box<dyn FnOnce() + Send + Sync>;

struct WorkQueue {
    tasks: Arc<(Mutex<VecDeque<Arc<WorkFn>>>, Condvar)>,
    workers: Vec<Option<JoinHandle<()>>>,
    shutdown: Arc<AtomicBool>,
}

impl WorkQueue {
    pub fn new(numWorkers: usize) -> Self {
        let tasks = Arc::new((Mutex::new(vec![].into()), Condvar::new()));
        let mut workers = Vec::with_capacity(numWorkers);
        let shutdown = Arc::new(AtomicBool::new(false));
        
        for _ in 0 .. numWorkers {
            let tasks = Arc::clone(&tasks);
            let shutdown = shutdown.clone();
            workers.push(Some(spawn(move || Self::worker(tasks, shutdown))));
        }
        
        Self { tasks, workers, shutdown }
    }
    
    pub fn submit(&self, func: impl FnOnce() + 'static + Send + Sync) {
        let mut vec = self.tasks.0.lock().unwrap();
        vec.push_back(Arc::new(Box::new(func)));
        drop(vec);
        self.tasks.1.notify_one();
    }
    
    pub fn shutdown(mut self) {
        self.shutdown.store(true, Ordering::Release);
        
        // notify all with the lock held to ensure every worker is either running tasks,
        // or waiting on the condvar; i.e. not between checking shutdown and the call to wait
        let lock = self.tasks.0.lock().unwrap();
        self.tasks.1.notify_all();
        drop(lock);
        
        for handle in &mut self.workers {
            handle.take().map(JoinHandle::join);
        }
    }
    
    fn worker(tasksData: Arc<(Mutex<VecDeque<Arc<WorkFn>>>, Condvar)>, shutdown: Arc<AtomicBool>) {
        loop {
            let mut tasks = tasksData.0.lock().unwrap();
            while tasks.len() == 0 {
                if shutdown.load(Ordering::Acquire) { return; }
                tasks = tasksData.1.wait(tasks).unwrap();
            }
            
            let arc = tasks.pop_front().unwrap();
            drop(tasks); // don't hold lock while running task
            let func = match Arc::try_unwrap(arc) {
                Ok(v) => v,
                Err(_) => panic!("could not unwrap inner arc"),
            };
            func();
        }
    }
}