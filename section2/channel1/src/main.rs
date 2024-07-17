use std::{sync::mpsc, thread::spawn};

enum Command {
    SayHello, Quit
}
fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = spawn(move || {
        while let Ok(command) = rx.recv(){
            match command {
                Command::SayHello => println!("Hello!"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    for _ in 0..10{
        tx.send(Command::SayHello).unwrap();
    }
    println!("Sending Quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
