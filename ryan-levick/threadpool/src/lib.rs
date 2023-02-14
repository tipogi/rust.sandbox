use std::{sync::{mpsc::{channel, Sender, Receiver}, Mutex, Arc}};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: Sender<Box<dyn Fn() + Send>>
}

impl ThreadPool {
    // limit thread to 64
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = vec![];

        for thread_number in 0..num_threads {
            let clone = receiver.clone();
            let worker = Worker::new(thread_number, clone);
            workers.push(worker);
        }
        Self {
            _workers: workers,
            sender
        }
    }

    pub fn execute<T>(&self, work: T)
        where
            T: Fn() + Send + 'static
    {
        self.sender.send(Box::new(work)).unwrap();
    }
}

struct Worker {
    _id: u8,
    _handle: std::thread::JoinHandle<()>
}

impl Worker {
    fn new(id: u8, rx: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = std::thread::spawn(move || loop {  
            println!("Start the thread...");
            let work = match rx.lock().unwrap().recv() {
                Ok(rx) => rx,
                Err(e) => {
                    println!("ERROR: {}", e);
                    break;
                }
            };
            work();
            println!("Thread finished the work!");
        });
        Self {
            _id: id,
            _handle: handle
        }
    }
}

type Job = Box<dyn Fn() + Send>;

#[cfg(test)]
mod tests {
    // Bring to the scope all the types that are outside that module
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(2);
        let foo = || std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(8))
    }
}
