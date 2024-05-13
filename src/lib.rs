use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// here we are saying ob is a type alias for a trait object that holds the type of closure expected by execute()
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool
{
    // Create new thread pool of specified size.
    // The size is the number of threads in the pool.
    // `new()` can panic if size is zero.
    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        // shared ownership + mutability -> smart pointers
        // but since we are working with threads, we want thread safe smart pointers -> arc smart pointer
        // and for safe mutability -> mutex smart pointer
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size
        {
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        return ThreadPool { workers, sender };
    }
    
    pub fn execute<F>(&self, f: F) 
        where F: FnOnce() + Send + 'static
    {
        // closure taken as one of the parameters and sent to the thread via channel
        // wrap closure in box smart pointer
        let job = Box::new(f);
        // send the job down the channel
        self.sender.send(job).unwrap();
    }
}

struct Worker
{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        // store thread that does nothing
        // we want threads to be constantly looking for jobs, hence `loop` is used to run infinite loop
        let thread = thread::spawn(move || loop { 
            let job = receiver
                .lock() // call lock() to acquire the mutex
                .unwrap()   // acquiring might fail, just unwrap it
                .recv()  // receive job from the channel
                .unwrap();
            println!("Worker {} got a job, executing", id);
            job();
        });

        return Worker { id, thread };
    }
}