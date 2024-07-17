use std::thread::spawn;


type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Job>();
    let handle = spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from job");
    let job2 = || {
        for i in 0..10{
            println!("{}",i);
        }
    };
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap(); 
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(||println!("Closure hello!"))).unwrap();
    handle.join().unwrap();
}
