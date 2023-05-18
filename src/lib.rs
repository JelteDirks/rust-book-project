use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
pub enum PoolCreationError {
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

        for i in 0..threads.len() {
            threads.push(Worker::new(i, Arc::clone(&rx)));
        }

        return Ok(ThreadPool {
            workers: threads,
            sender: tx,
        });
    }

    pub fn handle<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        let send = self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("lock receiver")
                .recv()
                .expect("receive message");

            println!("Worker {id} got a job");

            job();
        });

        return Worker { id, thread };
    }
}
