use std::{sync::{mpsc::{channel, Sender}, Mutex, Arc}};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
}

fn _foo(_item: &dyn std::fmt::Debug) {
	println!("Hello Ryan")
}

fn _bar() {
	// Specialized in u8
	_foo(&1u8);
	_foo(&2u8);
	// Specialized in String
	_foo(&String::new())
}

impl ThreadPool {
    // limit thread to 64
    pub fn new(num_threads: u8) -> Self {
        // Sometimes this is (tx, rx). (transmision, receiver)
        // In that case the channel requires a trait not a type. To pass a trait, we have to pass dynamically
        // We need that because we are adding a clousure
        // The closure signature could also be Fn(i8) -> i16
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        // With Mutex we can share across different threads and exlusively access the data, previosly
        // they we were sharing the receiver 36:00 min so, it was not exclusive
        // Receiver assumes that is running on in one place, that why we need mutex (Sync trait)
        // With Arc, we have multiple owners of one variable, receiver in that case (Send trait)
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            // increment the counter of the receiver. We cannot pass as ref because if the receiver goes out of 
            // scope, all the depending references will be trash because it is not going to find the reference
            let clone = receiver.clone();
            // With move we are moving all the variables to the clouse scope
            let handle = std::thread::spawn(move || loop {  
                println!("Start the thread...");
                // Receive the work
                // This is not the right way to do it because one of the workers panic, all the workers will panic
                // This it calls poisoning
                let work = clone.lock().unwrap().recv();
                match work {
                    Ok(work) => work(),
                    Err(e) => {
                        println!("ERROR: {}", e);
                        break;
                    }
                };
                //work();
                println!("Thread finished the work!");
            });
            _handles.push(handle);
            // one work, do work 
        }
        Self {
            _handles,
            sender
        }
    }

    // The generic type is a closure
    // Add static lifetime time to not drop in any moment
    // With that we protect not to pass referece variables
    //
    // Explanation of that min 1:20:00 but to recap is because the program already finishes
    // and it drops the argument, in that case closure, that we pass in execute. JUNK
    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

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
        // The reason that we need to add this sleep is because if not when the main 
        // finishes and drops pool (ThreadPool struct) from the memony the receive 
        // gets the error because the sender has been drop and the application panics
        // line: clone.lock().unwrap().recv()
        std::thread::sleep(std::time::Duration::from_secs(8))
    }
}
