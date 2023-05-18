pub struct ThreadPool;

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

        return Ok(ThreadPool {});
    }

    pub fn handle<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
