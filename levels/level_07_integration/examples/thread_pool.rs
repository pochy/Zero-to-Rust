use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);

            let handle = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            });

            workers.push(handle);
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .expect("thread pool sender is available")
            .send(Box::new(f))
            .expect("worker threads are available");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        while let Some(worker) = self.workers.pop() {
            worker.join().expect("worker thread should finish");
        }
    }
}

fn main() {
    let pool = ThreadPool::new(2);
    let (result_tx, result_rx) = mpsc::channel::<String>();

    for number in 0..4 {
        let result_tx = result_tx.clone();
        pool.execute(move || {
            let square = number * number;
            result_tx
                .send(format!("job {} -> {}", number, square))
                .expect("main thread receives results");
        });
    }

    drop(result_tx);

    let mut results: Vec<String> = result_rx.iter().collect();
    results.sort();

    for line in results {
        println!("{}", line);
    }

    println!("all jobs submitted");
}
