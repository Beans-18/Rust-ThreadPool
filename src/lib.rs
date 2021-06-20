pub use std::sync::mpsc;
pub use std::sync::{Arc, Mutex};
pub use std::thread;

pub struct ThreadPool {
    Workers: Vec<Workers>,
    Sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(numberofworkers: u16) -> ThreadPool {
        let mut i: Vec<Workers> = Vec::new();
        let (sender, reciever) = mpsc::channel();
        let reciever_arc = Arc::new(Mutex::new(reciever));
        for x in 0..numberofworkers {
            i.push(Workers::new(Arc::clone(&reciever_arc)));
        }
        ThreadPool {
            Workers: i,
            Sender: sender,
        }
    }

    fn execute<T>(&self, function: T) where T: FnOnce() + Send + 'static, {
        self.Sender.send(Message::Job(Box::new(function))).unwrap();
    }
}

pub struct Workers {
    thread: Option<thread::JoinHandle<()>>,
}

impl Workers {
    pub fn new(reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Workers {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv().unwrap();
            match message {
                Message::Job(job) => job(),
                Message::Terminate => break,
            }
        });
        Workers {
            thread: Some(thread),
        }

    }
}



impl Drop for ThreadPool {
    fn drop(&mut self) {
        for i in &self.Workers {
            self.Sender.send(Message::Terminate).unwrap();
        }
    }
}


pub enum Message {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}
