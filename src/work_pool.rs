use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct Woker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Woker {
    pub fn new(
        id: usize, 
        job_receiver: Arc<Mutex<Receiver<Job>>>,
        counter: Arc<Mutex<usize>>
    ) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let result = job_receiver.lock().unwrap().recv();
                    match result {
                        Ok(job) => {
                            *counter.lock().unwrap()  -= 1;
                            job();
                            *counter.lock().unwrap()  += 1;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }))
        }
    }
}

pub struct WokerPool {
    counter: Arc<Mutex<usize>>,
    workers: Vec<Woker>,
    job_sender: Option<Sender<Job>>,
}

pub type ThreadSafeWorkPool = Arc<Mutex<WokerPool>>;

impl WokerPool {
    pub fn new(size: usize) -> Self {
        let (job_sender, job_receiver) = channel::<Job>();
        let thread_safe_job_receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::with_capacity(size);
        let thread_saft_counter = Arc::new(Mutex::new(size));
        for i in 0..size {
            workers.push(Woker::new(i,Arc::clone(&thread_safe_job_receiver), Arc::clone(&thread_saft_counter) ))
        }
        Self {
            job_sender: Some(job_sender),
            workers,
            counter: thread_saft_counter,
        }
    }
    pub fn is_full(&self) -> bool {
        *self.counter.lock().unwrap() == 0
    }
    pub fn execute<F>(&mut self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {
        if self.is_full() {
            return
        }
        self.job_sender.as_mut().unwrap().send(Box::new(f)).unwrap();
    }
}
impl Drop for WokerPool {
    fn drop(&mut self) {
        drop(self.job_sender.take());
        for worker in &mut self.workers {
            println!("Work id {} stop", worker.id);
            if let Some(thread) = worker.thread.take() {
                 match thread.join() {
                    Ok(_) => {},
                    Err(_) => { println!("Error when join {}", worker.id) }
                 }
            }
        }
    }
}