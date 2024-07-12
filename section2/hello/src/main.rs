fn hello_thread(n: u32) {
    println!("Hello from thread {n}!");
}

fn do_math(i: u32) -> u32{
    let mut n = i+1;
    for _ in 0..10 {
        n *= 2;
    }
    n
}

fn main() {
    println!("Hello, world!");
    let mut handle_threads = Vec::new();
    for i in 0..10 {
        let handle_thread = std::thread::spawn(move || do_math(i));
        handle_threads.push(handle_thread);
    }
    handle_threads.into_iter().for_each(|h| {
        println!("{}",h.join().unwrap());
    });

    // FOR SPAWNING A THREAD
    // let handle_thread = std::thread::spawn(hello_thread);
    // handle_thread.join().unwrap();
}
