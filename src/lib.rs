use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// Possible errors while creating a ThreadPool.
#[derive(Debug)]
pub enum PoolCreationError {
    /// The amount of threads that are asked for is not supported or is 0.
    UnsupportedAmount,
}

type ThreadResult = Result<ThreadPool, PoolCreationError>;

impl ThreadPool {
    /// Create a new ThreadPool instance.
    ///
    /// # Panics
    /// Panics if n_threads is zero.
    pub fn new(n_threads: usize) -> ThreadResult {
        if n_threads == 0 {
            return Err(PoolCreationError::UnsupportedAmount);
        }

        let mut threads = Vec::with_capacity(n_threads);
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        for i in 0..n_threads {
            threads.push(Worker::new(i, Arc::clone(&rx)));
        }

        return Ok(ThreadPool {
            workers: threads,
            sender: Some(tx),
        });
    }

    /// Schedule a job on one of the available threads.
    ///
    /// # Panics
    /// The ThreadPool uses channels behind a mutex to send jobs to workers. If
    /// one of the task scheduled panics the mutex is poisoned and the server
    /// will panic on locking the mutex.
    pub fn handle<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("Worker {id} is waiting for a job");
            let msg = receiver.lock().unwrap().recv();

            match msg {
                Ok(job) => {
                    println!("Worker {id} got a job");
                    job();
                }
                Err(_) => {
                    println!("Shutting down worker {id}");
                    break;
                }
            }
        });

        return Worker {
            id,
            thread: Some(thread),
        };
    }
}
