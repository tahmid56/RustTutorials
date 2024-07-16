use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes");
}

fn main() {
    // let my_shared = Mutex::new(0);

    // THIS PORTION CAUSES A DEADLOCK
    // let lock = MY_SHARED.lock().unwrap();
    // let lock = MY_SHARED.lock().unwrap();

    // THIS LINE DROPS THE LOCK
    // std::mem::drop(lock);

    // THIS PORTION IS FOR TRYING TO GET THE LOCK
    // if let Ok(_lock) = MY_SHARED.try_lock(){
    //     println!("Got the lock");
    // }else {
    //     println!("No lock");
    // }

    // THIS PORTION IS FOR DEMONSTRATING THE THREAD POISONING
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread!");
    println!("{:?}", handle.join());
    let lock = MY_SHARED.lock();
    println!("{lock:?}", );

    // THIS PORTION IS FOR RECOVERING DATA FROM THREAD POISONING
    let recovered_data = lock.unwrap_or_else(|poisoned|{
        println!("Thread poisoned, Recovering data");
        poisoned.into_inner()
    });
    println!("{recovered_data:?}", );
}
