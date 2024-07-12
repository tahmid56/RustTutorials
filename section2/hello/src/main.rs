fn hello_thread() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello, world!");
    let handle_thread = std::thread::spawn(hello_thread);
    handle_thread.join().unwrap();
}
