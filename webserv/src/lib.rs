use std::{
    sync::{
        mpsc,
        Arc,
        Mutex
    },
    thread
};

use core::fmt;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    w: Vec<Worker>,
    s: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError::SizeLessThanOne);
        }

        let (s, r) = mpsc::channel();
        let mut w  = Vec::with_capacity(size);

        let r = Arc::new(Mutex::new(r));

        for id in 0..size {
            w.push(Worker::new(id, Arc::clone(&r)));
        }

        Ok(Self { w, s } )
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.s.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = recv.lock().unwrap().recv().unwrap();

            println!("{id} go a new job!");

            job();
        });

        Worker { id, thread }
    }
}

pub enum PoolCreationError {
    SizeLessThanOne
}

impl PoolCreationError {
    pub fn as_str(&self) -> &'static str {
        match self {
            PoolCreationError::SizeLessThanOne => "Cannot create a pool of size 0"
        }
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), f)
    }
}