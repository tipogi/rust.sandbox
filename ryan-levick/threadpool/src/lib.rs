use std::sync::{mpsc::channel, Mutex};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>
}

fn foo<>(item: &dyn std::fmt::Debug) {
	println!("Hello Ryan")
}

fn bar() {
	// Specialized in u8
	foo(&1u8);
	foo(&2u8);
	// Specialized in String
	foo(&String::new())
}

impl ThreadPool {
    // limit thread to 64
    pub fn new(num_threads: u8) -> Self {
        // Sometimes this is (tx, rx). (transmision, receiver)
        let (sender, receiver) = channel::<Box<dyn Fn()>>();
        let receiver = Mutex::new(receiver);
        let _handles = (0..num_threads)
            .map(|_| {
                // Spawn: Create and start the thread
                std::thread::spawn(|| {  
                    loop {
                        // Receive the work
                        let work = receiver.recv().unwrap();
                        work();
                        // one work, do work
                    }
                }) 
            }).collect();
        Self {
            _handles
        }
    }

    // The generic type is a closure
    pub fn execute<T: Fn()>(&self, work: T) {}
}

#[cfg(test)]
mod tests {

    // Bring to the scope all the types that are outside that module
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        pool.execute(|| println!("Hello from thread"));
    }
}
