use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread::{JoinHandle, spawn};

type WorkFn = Arc<Box<dyn FnOnce() + Send + Sync>>;

struct WorkQueue {
    workers: Vec<Option<JoinHandle<()>>>,
    tasks: Arc<(Mutex<VecDeque<WorkFn>>, Condvar)>,
    runningCounter: Arc<(AtomicUsize, Condvar)>,
    shutdown: Arc<AtomicBool>,
}

impl WorkQueue {
    pub fn new(numWorkers: usize) -> Self {
        let mut workers = Vec::with_capacity(numWorkers);
        let tasks = Arc::new((Mutex::new(vec![].into()), Condvar::new()));
        let runningCounter = Arc::new((AtomicUsize::new(0), Condvar::new()));
        let shutdown = Arc::new(AtomicBool::new(false));
        
        for _ in 0 .. numWorkers {
            let tasks = Arc::clone(&tasks);
            let runningCounter = Arc::clone(&runningCounter);
            let shutdown = shutdown.clone();
            workers.push(Some(spawn(move || Self::worker(tasks, runningCounter, shutdown))));
        }
        
        Self { workers, tasks, runningCounter, shutdown }
    }
    
    pub fn submit(&self, func: impl FnOnce() + 'static + Send + Sync) {
        let mut vec = self.tasks.0.lock().unwrap();
        vec.push_back(Arc::new(Box::new(func)));
        drop(vec);
        self.tasks.1.notify_one();
    }
    
    pub fn wait_done(&self) {
        let mut tasks = self.tasks.0.lock().unwrap();
        
        while tasks.len() > 0 || self.runningCounter.0.load(Ordering::Acquire) > 0 {
            tasks = self.runningCounter.1.wait(tasks).unwrap();
        }
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
    
    fn worker(tasksData: Arc<(Mutex<VecDeque<WorkFn>>, Condvar)>, runningCounter: Arc<(AtomicUsize, Condvar)>, shutdown: Arc<AtomicBool>) {
        loop {
            let mut tasks = tasksData.0.lock().unwrap();
            while tasks.len() == 0 {
                if shutdown.load(Ordering::Acquire) { return; }
                tasks = tasksData.1.wait(tasks).unwrap();
            }
            
            runningCounter.0.fetch_add(1, Ordering::AcqRel);
            let arc = tasks.pop_front().unwrap();
            drop(tasks); // don't hold lock while running task
            let func = match Arc::try_unwrap(arc) {
                Ok(v) => v,
                Err(_) => panic!("could not unwrap inner arc"),
            };
            func();
            runningCounter.0.fetch_sub(1, Ordering::AcqRel);
            runningCounter.1.notify_all();
        }
    }
}