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
    s: Option<mpsc::Sender<Job>>,
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

        Ok(Self { w, s: Some(s) } )
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.s.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.s.take());

        for w in self.w.iter_mut() {
            println!("{} is now shutting down", w.id);

            if let Some(t) = w.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = recv.lock().unwrap().recv();

            if let Ok(job) = job {
                println!("{id} got a new job!");
                job();
            } else {
                println!("the channel is closed, punching out");
                break;
            }
        });

        Worker { id, thread: Some(thread) }
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