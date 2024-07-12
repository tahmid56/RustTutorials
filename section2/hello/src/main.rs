fn hello_thread(n: u32) {
    println!("Hello from thread {n}!");
}

fn main() {
    println!("Hello, world!");
    let mut handle_threads = Vec::new();
    for i in 0..5 {
        let handle_thread = std::thread::spawn(move || hello_thread(i));
        handle_threads.push(handle_thread);
    }
    handle_threads.into_iter().for_each(|h| h.join().unwrap());

    // FOR SPAWNING A THREAD
    // let handle_thread = std::thread::spawn(hello_thread);
    // handle_thread.join().unwrap();
}
